use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::stats::RunResult;

/// DB操作
#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedRun {
    pub run_id: String,
    pub executed_at: String,
    pub overall_raw: f64,
    pub ci_lower: f64,
    pub ci_upper: f64,
    pub cv: f64,
    pub runs_used: i32,
    pub runs_excluded: i32,
    pub hardware_config_hash: String,
}

impl Database {
    /// データベースを開く（なければ作成）
    pub fn open(path: &Path) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.initialize()?;
        Ok(db)
    }

    /// テーブル初期化
    fn initialize(&self) -> SqlResult<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS run_results (
                run_id TEXT PRIMARY KEY,
                executed_at TEXT NOT NULL,
                overall_raw REAL NOT NULL,
                ci_lower REAL NOT NULL,
                ci_upper REAL NOT NULL,
                cv REAL NOT NULL,
                runs_used INTEGER NOT NULL DEFAULT 0,
                runs_excluded INTEGER NOT NULL DEFAULT 0,
                hardware_config_hash TEXT NOT NULL DEFAULT ''
            );

            CREATE TABLE IF NOT EXISTS sub_scores (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_id TEXT NOT NULL,
                module_name TEXT NOT NULL,
                raw_score REAL NOT NULL,
                normalized_score REAL NOT NULL,
                FOREIGN KEY (run_id) REFERENCES run_results(run_id)
            );

            CREATE TABLE IF NOT EXISTS metric_samples (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_id TEXT NOT NULL,
                module_name TEXT NOT NULL,
                timestamp_ms INTEGER NOT NULL,
                value REAL NOT NULL,
                label TEXT NOT NULL,
                FOREIGN KEY (run_id) REFERENCES run_results(run_id)
            );"
        )?;
        Ok(())
    }

    /// 結果を保存
    pub fn save_run(&self, result: &RunResult) -> SqlResult<()> {
        let tx = self.conn.unchecked_transaction()?;

        tx.execute(
            "INSERT INTO run_results (run_id, executed_at, overall_raw, ci_lower, ci_upper, cv, runs_used, runs_excluded, hardware_config_hash)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                result.run_id,
                result.executed_at.to_rfc3339(),
                result.overall_raw,
                result.ci_lower,
                result.ci_upper,
                result.cv,
                result.runs_used as i32,
                result.runs_excluded as i32,
                result.hardware_config_hash,
            ],
        )?;

        for (name, sub) in &result.sub_scores {
            tx.execute(
                "INSERT INTO sub_scores (run_id, module_name, raw_score, normalized_score) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![result.run_id, name, sub.raw_score, sub.normalized_score],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// 全実行結果を取得
    pub fn list_runs(&self) -> SqlResult<Vec<SavedRun>> {
        let mut stmt = self.conn.prepare(
            "SELECT run_id, executed_at, overall_raw, ci_lower, ci_upper, cv, runs_used, runs_excluded, hardware_config_hash
             FROM run_results ORDER BY executed_at DESC"
        )?;

        let runs = stmt.query_map([], |row| {
            Ok(SavedRun {
                run_id: row.get(0)?,
                executed_at: row.get(1)?,
                overall_raw: row.get(2)?,
                ci_lower: row.get(3)?,
                ci_upper: row.get(4)?,
                cv: row.get(5)?,
                runs_used: row.get(6)?,
                runs_excluded: row.get(7)?,
                hardware_config_hash: row.get(8)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;

        Ok(runs)
    }

    /// 結果を削除
    pub fn delete_run(&self, run_id: &str) -> SqlResult<()> {
        let tx = self.conn.unchecked_transaction()?;
        tx.execute("DELETE FROM metric_samples WHERE run_id = ?1", rusqlite::params![run_id])?;
        tx.execute("DELETE FROM sub_scores WHERE run_id = ?1", rusqlite::params![run_id])?;
        tx.execute("DELETE FROM run_results WHERE run_id = ?1", rusqlite::params![run_id])?;
        tx.commit()?;
        Ok(())
    }

    /// 個別実行結果の詳細（サブスコア含む）を取得
    pub fn get_run_detail(&self, run_id: &str) -> SqlResult<Option<crate::stats::RunResult>> {
        let mut stmt = self.conn.prepare(
            "SELECT run_id, executed_at, overall_raw, ci_lower, ci_upper, cv, runs_used, runs_excluded, hardware_config_hash
             FROM run_results WHERE run_id = ?1"
        )?;
        let mut rows = stmt.query_map(rusqlite::params![run_id], |row| {
            let overall_raw: f64 = row.get(2)?;
            let ci_lower: f64 = row.get(3)?;
            let ci_upper: f64 = row.get(4)?;
            Ok(crate::stats::RunResult {
                run_id: row.get(0)?,
                executed_at: row.get::<_, String>(1)?.parse::<chrono::DateTime<chrono::Utc>>().unwrap_or_default(),
                duration_ms: 0,
                sub_scores: std::collections::HashMap::new(),
                overall_raw,
                ci_lower,
                ci_upper,
                cv: row.get(5)?,
                runs_used: row.get(6)?,
                runs_excluded: row.get(7)?,
                hardware_config_hash: row.get(8)?,
            })
        })?;
        if let Some(row) = rows.next() {
            let mut result = row?;
            // サブスコアを取得
            let mut sub_stmt = self.conn.prepare(
                "SELECT module_name, raw_score, normalized_score FROM sub_scores WHERE run_id = ?1"
            )?;
            let subs = sub_stmt.query_map(rusqlite::params![run_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?, row.get::<_, f64>(2)?))
            })?;
            for sub in subs {
                let (name, raw, norm) = sub?;
                result.sub_scores.insert(name.clone(), crate::stats::SubScore {
                    module_name: name.clone(),
                    raw_score: raw,
                    normalized_score: norm,
                    runs: vec![],
                    excluded: vec![],
                });
            }
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use chrono::Utc;

    #[test]
    fn test_database_operations() {
        let path = Path::new(":memory:");
        let db = Database::open(path).unwrap();

        let result = RunResult {
            run_id: "test-1".to_string(),
            executed_at: Utc::now(),
            duration_ms: 5000,
            sub_scores: {
                let mut m = HashMap::new();
                m.insert("Render-Raster".to_string(), crate::stats::SubScore {
                    module_name: "Render-Raster".to_string(),
                    raw_score: 12450.0,
                    normalized_score: 12450.0,
                    runs: vec![12400.0, 12450.0, 12500.0],
                    excluded: vec![],
                });
                m
            },
            overall_raw: 12450.0,
            ci_lower: 12300.0,
            ci_upper: 12600.0,
            cv: 0.02,
            runs_used: 3,
            runs_excluded: 0,
            hardware_config_hash: "abc123".to_string(),
        };

        db.save_run(&result).unwrap();
        let runs = db.list_runs().unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].run_id, "test-1");
        assert!((runs[0].overall_raw - 12450.0).abs() < 0.1);

        db.delete_run("test-1").unwrap();
        let runs = db.list_runs().unwrap();
        assert_eq!(runs.len(), 0);
    }
}
