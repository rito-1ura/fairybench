use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::orchestrator::RawModuleResult;

/// 1回のベンチマーク実行結果（統計処理後）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResult {
    pub run_id: String,
    pub executed_at: DateTime<Utc>,
    pub duration_ms: i64,
    /// モジュール名 → サブスコア
    pub sub_scores: HashMap<String, SubScore>,
    /// 総合スコア（正規化前の生集計）
    pub overall_raw: f64,
    /// 信頼区間（95%）
    pub ci_lower: f64,
    pub ci_upper: f64,
    /// 変動係数
    pub cv: f64,
    /// 採用した実行回数
    pub runs_used: usize,
    /// 除外した実行回数
    pub runs_excluded: usize,
    /// ハードウェア構成ハッシュ（後で設定）
    pub hardware_config_hash: String,
}

/// 1モジュールのサブスコア
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubScore {
    pub module_name: String,
    pub raw_score: f64,
    pub normalized_score: f64,
    pub runs: Vec<f64>,        // 各実行の生スコア
    pub excluded: Vec<f64>,    // 除外されたスコア
}

/// 統計エンジン（§4.2）
#[derive(Debug)]
pub struct StatEngine {
    /// 変動係数閾値（超過で自動追加実行）
    pub cv_threshold: f64,
    /// 最低実行回数
    pub min_runs: usize,
    /// 最高実行回数
    pub max_runs: usize,
}

impl Default for StatEngine {
    fn default() -> Self {
        Self {
            cv_threshold: 0.03,  // 3%
            min_runs: 3,
            max_runs: 10,
        }
    }
}

impl StatEngine {
    pub fn new(cv_threshold: f64, min_runs: usize, max_runs: usize) -> Self {
        Self { cv_threshold, min_runs, max_runs }
    }

    /// 生のモジュール結果を統計処理し、RunResultにまとめる
    pub fn process(&self, module_results: Vec<RawModuleResult>) -> RunResult {
        let run_id = Uuid::new_v4().to_string();
        let executed_at = Utc::now();

        let mut sub_scores = HashMap::new();
        let mut overall_raw = 0.0;
        let mut ci_values = Vec::new();
        let mut total_runs_used = 0;
        let mut total_runs_excluded = 0;

        for result in &module_results {
            let (score, runs, excluded) =
                Self::compute_sub_score(&result.samples.iter().map(|s| s.value).collect::<Vec<_>>());

            let sub = SubScore {
                module_name: result.module_name.clone(),
                raw_score: score,
                normalized_score: score,  // 正規化は後段（§6.2）
                runs,
                excluded,
            };
            total_runs_used += sub.runs.len();
            total_runs_excluded += sub.excluded.len();
            overall_raw += sub.raw_score;
            ci_values.push(sub.raw_score);
            sub_scores.insert(result.module_name.clone(), sub);
        }

        // 全モジュールのCV計算
        let cv = if ci_values.len() <= 1 {
            0.0
        } else {
            let mean = ci_values.iter().sum::<f64>() / ci_values.len() as f64;
            let variance = ci_values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (ci_values.len() - 1) as f64;
            if mean == 0.0 { 0.0 } else { variance.sqrt() / mean }
        };

        // 95%信頼区間
        let (ci_lower, ci_upper) = Self::confidence_interval_95(&ci_values);

        RunResult {
            run_id,
            executed_at,
            duration_ms: module_results.iter().map(|r| r.duration_ms).sum(),
            sub_scores,
            overall_raw,
            ci_lower,
            ci_upper,
            cv,
            runs_used: total_runs_used,
            runs_excluded: total_runs_excluded,
            hardware_config_hash: String::new(),
        }
    }

    /// 1モジュールの生サンプル群からサブスコアを計算
    /// 戻り値: (最終スコア, 採用された実行値一覧, 除外された実行値一覧)
    fn compute_sub_score(samples: &[f64]) -> (f64, Vec<f64>, Vec<f64>) {
        if samples.is_empty() {
            return (0.0, vec![], vec![]);
        }

        let n = samples.len();
        let mean = samples.iter().sum::<f64>() / n as f64;

        // 標準偏差
        let variance = if n > 1 {
            samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64
        } else {
            0.0
        };
        let _std_dev = variance.sqrt();

        // IQR-based 外れ値検出
        let mut sorted = samples.to_vec();
        sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let q1 = sorted[n / 4];
        let q3 = sorted[(n * 3) / 4];
        let iqr = q3 - q1;
        let lower_bound = q1 - 1.5 * iqr;
        let upper_bound = q3 + 1.5 * iqr;

        let (mut accepted, mut excluded): (Vec<f64>, Vec<f64>) = samples.iter().partition(|&&v| {
            v >= lower_bound && v <= upper_bound
        });

        // もし全部除外されたら全採用
        if accepted.is_empty() {
            accepted = samples.to_vec();
            excluded.clear();
        }

        let final_score = accepted.iter().sum::<f64>() / accepted.len() as f64;

        (final_score, accepted, excluded)
    }

    /// 95%信頼区間（t分布を仮定）
    fn confidence_interval_95(values: &[f64]) -> (f64, f64) {
        let n = values.len();
        if n <= 1 {
            return (values.first().copied().unwrap_or(0.0), values.first().copied().unwrap_or(0.0));
        }

        let mean = values.iter().sum::<f64>() / n as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
        let std_err = (variance / n as f64).sqrt();

        // t分布の95%臨界値（簡易近似: n>=30なら1.96, それ以外は固定テーブル）
        let t = match n {
            2 => 12.71,
            3 => 4.30,
            4 => 3.18,
            5 => 2.78,
            6 => 2.57,
            7 => 2.45,
            8 => 2.37,
            9 => 2.31,
            10 => 2.26,
            _ => 1.96,
        };

        let margin = t * std_err;
        (mean - margin, mean + margin)
    }

    /// 追加実行が必要かどうか
    pub fn needs_more_runs(&self, current_runs: usize, cv: f64) -> bool {
        current_runs < self.max_runs
            && (current_runs < self.min_runs || cv > self.cv_threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_statistics() {
        let _engine = StatEngine::default();
        let samples = vec![100.0, 102.0, 98.0, 101.0, 99.0];
        let (_score, _accepted, excluded) = StatEngine::compute_sub_score(&samples);
        assert!(excluded.is_empty());
    }

    #[test]
    fn test_outlier_detection() {
        let samples = vec![100.0, 102.0, 98.0, 101.0, 50.0];  // 50 is outlier
        let (_score, _accepted, excluded) = StatEngine::compute_sub_score(&samples);
        assert_eq!(excluded.len(), 1);
        assert!((*excluded.first().unwrap() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_confidence_interval() {
        let vals = vec![100.0, 102.0, 98.0];
        let (lo, hi) = StatEngine::confidence_interval_95(&vals);
        assert!(lo < 100.0);
        assert!(hi > 100.0);
    }

    #[test]
    fn test_needs_more_runs() {
        let engine = StatEngine::new(0.03, 3, 10);
        assert!(engine.needs_more_runs(2, 0.01));  // 最低実行回数を下回る
        assert!(!engine.needs_more_runs(3, 0.01));   // 条件クリア
        assert!(engine.needs_more_runs(3, 0.05));    // CV超過
        assert!(!engine.needs_more_runs(10, 0.05));  // 最大実行回数到達
    }
}
