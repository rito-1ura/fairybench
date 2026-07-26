use serde::{Deserialize, Serialize};

/// ゲームタイトルのシステム要件と分析結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEntry {
    pub title: String,
    pub category: String,          // "AAA", "eSports", "Indie", "Simulation"
    pub min_score: f64,            // FairyScore下限 (動作可能目安)
    pub rec_score: f64,            // FairyScore推奨 (快適動作目安)
    pub min_memory_gb: u32,
    pub rec_memory_gb: u32,
    pub storage_gb: u32,
    pub gpu_min: String,           // 例: "GTX 1060"
    pub gpu_rec: String,           // 例: "RTX 3070"
    pub has_raytracing: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GameAnalysis {
    pub title: String,
    pub category: String,
    pub status: String,            // "optimal" | "playable" | "borderline" | "insufficient" | "unknown"
    pub status_jp: String,
    pub fps_est: String,
    pub score_pct: f64,
    pub min_score: f64,
    pub rec_score: f64,
    pub gpu_min: String,
    pub gpu_rec: String,
    pub storage_gb: u32,
    pub has_raytracing: bool,
    pub details: Vec<String>,
}

/// 組み込みゲームデータベース
const GAME_DB: &str = include_str!("../data/games.json");

pub fn load_games() -> Vec<GameEntry> {
    serde_json::from_str(GAME_DB).unwrap_or_else(|e| {
        log::error!("Failed to parse games.json: {e}");
        vec![
            GameEntry {
                title: "Example Game".into(),
                category: "AAA".into(),
                min_score: 0.0,
                rec_score: 0.0,
                min_memory_gb: 16,
                rec_memory_gb: 32,
                storage_gb: 100,
                gpu_min: "GTX 1060".into(),
                gpu_rec: "RTX 3070".into(),
                has_raytracing: true,
                notes: vec![],
            }
        ]
    })
}

/// ユーザースコアとゲーム要件を比較
pub fn analyze_games(user_score: f64, user_memory_gb: f64) -> Vec<GameAnalysis> {
    let games = load_games();
    games.into_iter().map(|g| {
        // スコア充足率
        let score_pct = if g.rec_score > 0.0 {
            (user_score / g.rec_score * 100.0).min(200.0)
        } else {
            0.0
        };

        let (status, status_jp, fps_est, details) = if user_score >= g.rec_score {
            let fps = if score_pct >= 150.0 { "90-144 FPS" } else if score_pct >= 120.0 { "60-90 FPS" } else { "60+ FPS" };
            ("optimal", "快適", fps, {
                let mut d = vec![format!("Score: {:.0}% of recommended", score_pct)];
                if user_memory_gb >= g.rec_memory_gb as f64 {
                    d.push(format!("RAM {:.0}GB >= {}GB OK", user_memory_gb, g.rec_memory_gb));
                } else {
                    d.push(format!("RAM {:.0}GB < {}GB (upgrade recommended)", user_memory_gb, g.rec_memory_gb));
                }
                d
            })
        } else if user_score >= g.min_score {
            let fps = if score_pct >= 80.0 { "45-60 FPS" } else { "30-45 FPS" };
            ("playable", "可", fps, {
                let mut d = vec![format!("Score: {:.0}% of recommended", score_pct)];
                if user_memory_gb >= g.min_memory_gb as f64 {
                    d.push(format!("RAM {:.0}GB >= {}GB OK", user_memory_gb, g.min_memory_gb));
                } else {
                    d.push(format!("RAM {:.0}GB < {}GB (minimum not met)", user_memory_gb, g.min_memory_gb));
                }
                d
            })
        } else if user_score >= g.min_score * 0.7 {
            ("borderline", "限界", "Below 30 FPS", {
                vec![
                    format!("Score: {:.0}% of recommended", score_pct),
                    format!("Score: {:.0}% of minimum", user_score / g.min_score * 100.0),
                    "Lower settings / resolution required".into(),
                ]
            })
        } else {
            ("insufficient", "不可", "—", {
                vec![
                    format!("Score: {:.0}% of minimum", user_score / g.min_score.max(1.0) * 100.0),
                    "Hardware upgrade required".into(),
                ]
            })
        };

        // レイトレーシング関連
        let mut details = details;
        if g.has_raytracing && status == "optimal" {
            details.push("Ray tracing: possible".into());
        } else if g.has_raytracing && status == "playable" {
            details.push("Ray tracing: not recommended".into());
        } else if g.has_raytracing {
            details.push("Ray tracing: unavailable".into());
        }

        // ストレージ
        details.push(format!("Storage: {}GB required", g.storage_gb));

        GameAnalysis {
            title: g.title,
            category: g.category,
            status: status.to_string(),
            status_jp: status_jp.to_string(),
            fps_est: fps_est.to_string(),
            score_pct,
            min_score: g.min_score,
            rec_score: g.rec_score,
            gpu_min: g.gpu_min,
            gpu_rec: g.gpu_rec,
            storage_gb: g.storage_gb,
            has_raytracing: g.has_raytracing,
            details,
        }
    }).collect()
}
