mod db;
mod games;
mod orchestrator;
mod plugin;
mod stats;
mod thermal;
mod workloads;

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use serde::Serialize;
use tauri::{Emitter, Manager, State};
use crate::orchestrator::BenchModule;

/// アプリケーション状態
pub struct AppState {
    pub db: Mutex<db::Database>,
    pub stats_engine: stats::StatEngine,
    pub thermal_monitor: Mutex<thermal::ThermalMonitor>,
    pub plugin_host: Mutex<plugin::PluginHost>,
}

// ===== デバイス情報 =====
#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    pub adapter_name: String,
    pub backend: String,
    pub device_type: String,
    pub driver_info: String,
    pub os_info: String,
    pub cpu_name: String,
    pub cpu_cores: u32,
    pub total_memory_gb: f64,
    pub api_version: String,
}

/// ストリーミング用の進捗イベント
#[derive(Debug, Clone, Serialize)]
pub struct ModuleProgress {
    pub module: String,
    pub score: f64,
    pub label: String,
    pub duration_ms: i64,
    pub phase: String,
}

// ===== Tauri Commands =====

#[tauri::command]
fn get_version() -> String {
    "FairyBench v0.1.0-prototype".to_string()
}

#[tauri::command]
fn get_device_info() -> DeviceInfo {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }));
    let info = adapter.as_ref().map(|a| a.get_info());

    let cpu_name = std::process::Command::new("wmic")
        .args(["cpu", "get", "name"]).output().ok()
        .and_then(|o| {
            if !o.status.success() { return None; }
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.lines().nth(1).map(|l| l.trim().to_string())
        }).unwrap_or_default();

    let total_memory_gb = std::process::Command::new("wmic")
        .args(["computersystem", "get", "TotalPhysicalMemory"]).output().ok()
        .and_then(|o| {
            if !o.status.success() { return None; }
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.lines().nth(1)
                .and_then(|l| l.trim().parse::<f64>().ok())
                .map(|b| b / (1024.0 * 1024.0 * 1024.0))
        }).unwrap_or(0.0);

    let cpu_cores = std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(0);

    DeviceInfo {
        adapter_name: info.as_ref().map(|i| i.name.clone()).unwrap_or_default(),
        backend: format!("{:?}", info.as_ref().map(|i| i.backend).unwrap_or(wgpu::Backend::Empty)),
        device_type: format!("{:?}", info.as_ref().map(|i| i.device_type).unwrap_or(wgpu::DeviceType::Other)),
        driver_info: info.as_ref().map(|i| i.driver.clone()).unwrap_or_default(),
        os_info: std::env::consts::OS.to_string(),
        cpu_name,
        cpu_cores,
        total_memory_gb,
        api_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

/// ベンチマーク実行（async: メインスレッドをブロックしない）
#[tauri::command]
async fn run_benchmark(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<stats::RunResult, String> {
    let engine = &state.stats_engine;

    let mut modules: Vec<Box<dyn BenchModule>> = vec![
        Box::new(workloads::RenderRaster::new()),
        Box::new(workloads::RenderPathTrace::new()),
        Box::new(workloads::RenderProcedural::new()),
        Box::new(workloads::Render3DScene::new()),
        Box::new(workloads::StorageThroughput::new()),
        Box::new(workloads::MemoryBandwidth::new()),
        Box::new(workloads::AiInference::new()),
        Box::new(workloads::AiGenerative::new()),
    ];

    let mut raw_results = Vec::new();

    for module in &mut modules {
        let name = module.name();
        log::info!("Benchmark: starting module '{}'", name);

        // モジュール開始
        let _ = app_handle.emit("benchmark-event", ModuleProgress {
            module: name.to_string(),
            score: 0.0,
            label: String::new(),
            duration_ms: 0,
            phase: "start".into(),
        });

        module.prepare()?;

        // Heartbeat: emit pulse every 250ms during module execution
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        let emitter = app_handle.clone();
        let mname = name.to_string();
        let start_time = std::time::Instant::now();
        let hb = std::thread::spawn(move || {
            while r.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(250));
                if !r.load(Ordering::Relaxed) { break; }
                let elapsed = start_time.elapsed().as_secs_f64();
                let _ = emitter.emit("benchmark-event", ModuleProgress {
                    module: mname.clone(),
                    score: elapsed,
                    label: format!("{:.1}s", elapsed),
                    duration_ms: (elapsed * 1000.0) as i64,
                    phase: "pulse".into(),
                });
            }
        });

        let samples = module.run(&orchestrator::RunPolicy::FixedIterations(3))?;
        running.store(false, Ordering::Relaxed);
        hb.join().ok();
        let duration = if samples.len() >= 2 {
            samples.last().unwrap().timestamp_ms - samples.first().unwrap().timestamp_ms
        } else { 0 };
        module.teardown()?;

        let avg_score = if samples.is_empty() { 0.0 } else {
            samples.iter().map(|s| s.value).sum::<f64>() / samples.len() as f64
        };
        let label = samples.first().map(|s| s.label.clone()).unwrap_or_default();

        let _ = app_handle.emit("benchmark-event", ModuleProgress {
            module: name.to_string(),
            score: avg_score,
            label,
            duration_ms: duration,
            phase: "complete".into(),
        });

        raw_results.push(orchestrator::RawModuleResult::new(name, samples, duration));
        log::info!("Benchmark: module '{}' completed", name);
    }

    let result = engine.process(raw_results);

    // DB保存
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.save_run(&result).map_err(|e| e.to_string())?;

    log::info!("Benchmark complete: run_id={}, score={}", result.run_id, result.overall_raw);
    Ok(result)
}

#[tauri::command]
fn list_results(state: State<AppState>) -> Result<Vec<db::SavedRun>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.list_runs().map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_result(state: State<AppState>, run_id: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_run(&run_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_thermal_snapshot(state: State<AppState>) -> thermal::ThermalSample {
    let mut monitor = state.thermal_monitor.lock().unwrap();
    monitor.sample()
}

#[tauri::command]
fn get_plugin_info(state: State<AppState>) -> Vec<plugin::PluginManifest> {
    let host = state.plugin_host.lock().unwrap();
    host.loaded_plugins.clone()
}

/// AAAゲーム要件解析
#[tauri::command]
fn analyze_games(result: stats::RunResult) -> Vec<games::GameAnalysis> {
    let user_score = result.overall_raw;
    let user_memory_gb = std::process::Command::new("wmic")
        .args(["computersystem", "get", "TotalPhysicalMemory"])
        .output().ok()
        .and_then(|o| {
            if !o.status.success() { return None; }
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.lines().nth(1)
                .and_then(|l| l.trim().parse::<f64>().ok())
                .map(|b| b / (1024.0 * 1024.0 * 1024.0))
        }).unwrap_or(16.0);
    games::analyze_games(user_score, user_memory_gb)
}

/// 個別実行結果の詳細を取得
#[tauri::command]
fn get_run_detail(state: State<AppState>, run_id: String) -> Result<Option<stats::RunResult>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_run_detail(&run_id).map_err(|e| e.to_string())
}

// ===== Logging Setup =====
fn setup_logging(app_dir: &std::path::Path) -> Result<(), fern::InitError> {
    let log_dir = app_dir.join("logs");
    std::fs::create_dir_all(&log_dir).ok();
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(), record.target(), message))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(log_dir.join("fairybench.log"))?)
        .chain(fern::Dispatch::new()
            .level(log::LevelFilter::Warn)
            .chain(fern::log_file(log_dir.join("fairybench-error.log"))?))
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

// ===== App Setup =====
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            setup_logging(&app_dir).expect("failed to setup file logging");

            let db_path = app_dir.join("fairybench.db");
            let database = db::Database::open(&db_path).expect("failed to open database");

            log::info!("FairyBench initialized. DB at: {:?}, logs at: {:?}", db_path, app_dir.join("logs"));

            app.manage(AppState {
                db: Mutex::new(database),
                stats_engine: stats::StatEngine::default(),
                thermal_monitor: Mutex::new(thermal::ThermalMonitor::new()),
                plugin_host: Mutex::new(plugin::PluginHost::new()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_version,
            get_device_info,
            run_benchmark,
            list_results,
            delete_result,
            get_thermal_snapshot,
            get_plugin_info,
            analyze_games,
            get_run_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running FairyBench");
}
