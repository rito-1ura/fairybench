use std::time::Instant;
use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// Cpu-Hash: ハッシュ計算スループット (DefaultHasher)
pub struct CpuHash {
    data_size: usize,
}

impl CpuHash {
    pub fn new() -> Self {
        Self { data_size: 1_000_000 }
    }
}

impl BenchModule for CpuHash {
    fn name(&self) -> &'static str { "Cpu-Hash" }

    fn prepare(&mut self) -> Result<(), String> {
        log::info!("Cpu-Hash: prepared {} elements", self.data_size);
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        use std::hash::{Hash, Hasher};
        let n = self.data_size;
        let data: Vec<u64> = (0..n).map(|i| i as u64).collect();
        let mut samples = Vec::new();

        for _ in 0..3 {
            let start = Instant::now();
            let mut h = std::collections::hash_map::DefaultHasher::new();
            for &v in &data {
                v.hash(&mut h);
                std::hint::black_box(h.finish());
            }
            let elapsed = start.elapsed();
            let hashes_per_sec = n as f64 / elapsed.as_secs_f64();
            log::info!("Cpu-Hash: {:.0} hashes/sec ({}ms)", hashes_per_sec, elapsed.as_millis());
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: hashes_per_sec / 1_000_000.0,
                label: "Mhashes/sec".into(),
            });
        }
        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> { Ok(()) }
}
