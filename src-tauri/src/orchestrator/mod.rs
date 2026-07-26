use serde::{Deserialize, Serialize};

/// 測定モジュールの抽象契約 (§3.2)
pub trait BenchModule: Send {
    /// モジュール名（例: "Render-Raster"）
    fn name(&self) -> &'static str;

    /// シーン/モデル/データセットの読み込みとウォームアップ
    fn prepare(&mut self) -> Result<(), String>;

    /// 実行（固定反復回数 or 最小実行時間ポリシーの両対応）
    fn run(&mut self, policy: &RunPolicy) -> Result<Vec<MetricSample>, String>;

    /// 後片付けとメモリ解放の検証
    fn teardown(&mut self) -> Result<(), String>;
}

/// 実行ポリシー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunPolicy {
    /// 固定反復回数
    FixedIterations(u32),
    /// 最小実行時間（秒）＋自動追加
    MinDuration { seconds: u32, cv_threshold: f64 },
    /// 簡易（1回実行）
    Quick,
}

/// 1サンプルのメトリクス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSample {
    pub timestamp_ms: i64,
    pub value: f64,
    pub label: String,       // "fps", "tokens/sec", "IOPS", etc.
}

/// モジュール実行の生結果（統計エンジンに渡される前）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawModuleResult {
    pub module_name: String,
    pub samples: Vec<MetricSample>,
    pub raw_score: f64,
    pub duration_ms: i64,
}

impl RawModuleResult {
    pub fn new(module_name: &str, samples: Vec<MetricSample>, duration_ms: i64) -> Self {
        // raw_score はサンプル平均
        let raw_score = if samples.is_empty() {
            0.0
        } else {
            samples.iter().map(|s| s.value).sum::<f64>() / samples.len() as f64
        };
        Self {
            module_name: module_name.to_string(),
            samples,
            raw_score,
            duration_ms,
        }
    }
}
