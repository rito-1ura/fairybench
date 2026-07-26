use serde::{Deserialize, Serialize};

/// プラグインマニフェスト（§4.4）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    /// 対応OS（"windows", "macos", または空=全対応）
    pub supported_os: Vec<String>,
    /// 測定対象カテゴリ（"render", "ai", "storage", "custom"）
    pub category: String,
    /// プラグインインターフェースバージョン（§4.5）
    pub api_version_major: u32,
    pub api_version_minor: u32,
}

/// プラグインホスト（§4.4 スケルトン）
/// v1では同梱プラグインのみを対象とし、外部プラグインレジストリは未実装
#[derive(Debug, Default)]
pub struct PluginHost {
    pub loaded_plugins: Vec<PluginManifest>,
}

impl PluginHost {
    pub fn new() -> Self {
        Self { loaded_plugins: Vec::new() }
    }

    /// マニフェストの互換性を検証（§4.5）
    /// 現行APIバージョンは major=1, minor=0
    pub fn validate_manifest(&self, manifest: &PluginManifest) -> Result<(), String> {
        if manifest.api_version_major != 1 {
            return Err(format!(
                "プラグイン API バージョン non-compatible: expected major=1, got major={}",
                manifest.api_version_major
            ));
        }
        // 対応OSチェック
        if !manifest.supported_os.is_empty() {
            let os = std::env::consts::OS;
            if !manifest.supported_os.iter().any(|s| s == os) {
                return Err(format!(
                    "プラグイン '{}' は OS '{}' をサポートしていません",
                    manifest.name, os
                ));
            }
        }
        Ok(())
    }

    /// プラグインをロード（スケルトン）
    /// v1ではマニフェスト検証のみ行い、実際の動的ロードは後フェーズ
    pub fn load_plugin(&mut self, manifest: PluginManifest) -> Result<(), String> {
        self.validate_manifest(&manifest)?;
        log::info!("プラグイン '{}' v{} をロードしました", manifest.name, manifest.version);
        self.loaded_plugins.push(manifest);
        Ok(())
    }
}
