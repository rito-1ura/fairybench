use std::time::Instant;
use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// Cpu-Sort: ソートアルゴリズムスループット
pub struct CpuSort {
    data_size: usize,
}

impl CpuSort {
    pub fn new() -> Self {
        Self::with_size(500_000)
    }
    pub fn with_size(data_size: usize) -> Self {
        Self { data_size }
    }
}

impl BenchModule for CpuSort {
    fn name(&self) -> &'static str { "Cpu-Sort" }

    fn prepare(&mut self) -> Result<(), String> {
        log::info!("Cpu-Sort: prepared {} elements", self.data_size);
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let mut samples = Vec::new();

        for _ in 0..3 {
            let mut data: Vec<f64> = (0..self.data_size).map(|i| {
                let p = (i as f64 * 7.123456789).fract();
                p * 10.0 - 5.0
            }).collect();
            let start = Instant::now();
            data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let elapsed = start.elapsed();
            let _: f64 = data.iter().sum::<f64>(); // prevent optimization

            let items_per_sec = self.data_size as f64 / elapsed.as_secs_f64() / 1_000_000.0;
            log::info!("Cpu-Sort: {:.2}M items/sec ({}ms)", items_per_sec, elapsed.as_millis());
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: items_per_sec,
                label: "M items/s".into(),
            });
        }
        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> { Ok(()) }
}
