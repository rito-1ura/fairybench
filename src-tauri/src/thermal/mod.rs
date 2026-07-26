use serde::{Deserialize, Serialize};
use std::process::Command;

/// サーマル/電力/システムモニタリング（拡張版）
#[derive(Debug, Default)]
pub struct ThermalMonitor {
    history: Vec<ThermalSample>,
}

/// 1時点の拡張サンプル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalSample {
    pub timestamp_ms: i64,
    pub cpu_temp_avg: Option<f64>,
    pub gpu_temp: Option<f64>,
    pub cpu_clock_ghz: Option<f64>,
    pub gpu_clock_mhz: Option<f64>,
    pub power_watts: Option<f64>,
    pub cpu_load_pct: Option<f64>,
    pub gpu_load_pct: Option<f64>,
    pub gpu_mem_used_mb: Option<f64>,
    pub gpu_mem_total_mb: Option<f64>,
    pub gpu_fan_pct: Option<f64>,
    pub sys_mem_used_gb: Option<f64>,
    pub sys_mem_total_gb: Option<f64>,
    pub sensors_available: bool,
}

fn nvidia_query(flag: &str) -> Option<f64> {
    let out = Command::new("nvidia-smi")
        .args(["--query-gpu", flag, "--format=csv,noheader,nounits"])
        .output()
        .ok()?;
    if !out.status.success() { return None; }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() || s.to_lowercase().contains("not found") { return None; }
    s.parse::<f64>().ok()
}

fn wmic_get(path: &str, property: &str) -> Option<f64> {
    let out = Command::new("wmic")
        .args(["path", path, "get", property])
        .output()
        .ok()?;
    if !out.status.success() { return None; }
    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in stdout.lines().skip(1) {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            if let Ok(val) = trimmed.parse::<f64>() {
                return Some(val);
            }
        }
    }
    None
}

impl ThermalMonitor {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn history(&self) -> &[ThermalSample] {
        &self.history
    }

    /// 拡張サンプリング: CPU温度/負荷、GPU温度/負荷/メモリ/ファン、システムメモリ
    pub fn sample(&mut self) -> ThermalSample {
        let mut s = ThermalSample {
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            cpu_temp_avg: None, gpu_temp: None,
            cpu_clock_ghz: None, gpu_clock_mhz: None,
            power_watts: None,
            cpu_load_pct: None, gpu_load_pct: None,
            gpu_mem_used_mb: None, gpu_mem_total_mb: None,
            gpu_fan_pct: None,
            sys_mem_used_gb: None, sys_mem_total_gb: None,
            sensors_available: false,
        };

        // CPU温度
        s.cpu_temp_avg = {
            let raw = Command::new("wmic")
                .args(["path", "Win32_PerfFormattedData_Counters_ThermalZoneInformation", "get", "Temperature"])
                .output().ok().and_then(|o| {
                    if !o.status.success() { return None; }
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    for line in stdout.lines().skip(1) {
                        let trimmed = line.trim();
                        if let Ok(val) = trimmed.parse::<f64>() {
                            let c = val / 10.0 - 273.15;
                            if c > -50.0 && c < 150.0 { return Some(c); }
                            return Some(val);
                        }
                    }
                    None
                });
            raw.or_else(|| {
                Command::new("wmic")
                    .args(["/namespace:\\\\root\\wmi", "PATH", "MSAcpi_ThermalZoneTemperature", "get", "CurrentTemperature"])
                    .output().ok().and_then(|o| {
                        if !o.status.success() { return None; }
                        String::from_utf8_lossy(&o.stdout).lines().skip(1)
                            .filter_map(|l| l.trim().parse::<i32>().ok())
                            .filter_map(|v| { let c = v as f64 / 10.0 - 273.15; if c > -50.0 && c < 150.0 { Some(c) } else { None } })
                            .next()
                    })
            })
        };
        if s.cpu_temp_avg.is_some() { s.sensors_available = true; }

        // GPU温度
        s.gpu_temp = nvidia_query("temperature.gpu");
        if s.gpu_temp.is_some() { s.sensors_available = true; }

        // CPUクロック
        if let Some(mhz) = wmic_get("Win32_Processor", "CurrentClockSpeed") {
            s.cpu_clock_ghz = Some(mhz / 1000.0);
            s.sensors_available = true;
        }

        // GPUクロック・メモリ・ファン
        s.gpu_clock_mhz = nvidia_query("clocks.gr");
        s.gpu_mem_used_mb = nvidia_query("memory.used");
        s.gpu_mem_total_mb = nvidia_query("memory.total");
        s.gpu_fan_pct = nvidia_query("fan.speed");
        for v in [&s.gpu_clock_mhz, &s.gpu_mem_used_mb, &s.gpu_mem_total_mb, &s.gpu_fan_pct] {
            if v.is_some() { s.sensors_available = true; }
        }

        // 消費電力
        s.power_watts = nvidia_query("power.draw");
        if s.power_watts.is_some() { s.sensors_available = true; }

        // GPU負荷
        s.gpu_load_pct = nvidia_query("utilization.gpu");
        if s.gpu_load_pct.is_some() { s.sensors_available = true; }

        // CPU負荷
        s.cpu_load_pct = wmic_get("Win32_Processor", "LoadPercentage");
        if s.cpu_load_pct.is_some() { s.sensors_available = true; }

        // システムメモリ
        if let Some(total_b) = wmic_get("Win32_ComputerSystem", "TotalPhysicalMemory") {
            s.sys_mem_total_gb = Some(total_b / (1024.0 * 1024.0 * 1024.0));
        }
        if let Some(free_b) = wmic_get("Win32_OperatingSystem", "FreePhysicalMemory") {
            if let Some(total_b) = wmic_get("Win32_ComputerSystem", "TotalPhysicalMemory") {
                let free_bytes = free_b * 1024.0; // WMIC returns KB
                s.sys_mem_used_gb = Some((total_b - free_bytes) / (1024.0 * 1024.0 * 1024.0));
            }
        }

        // 履歴保持（最新60件）
        self.history.push(s.clone());
        if self.history.len() > 60 {
            self.history.remove(0);
        }

        s
    }
}
