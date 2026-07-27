use std::time::Instant;
use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// Cpu-Compress: 圧縮シミュレーション（データ変換スループット測定）
pub struct CpuCompress {
    data_size: usize,
}

impl CpuCompress {
    pub fn new() -> Self {
        Self { data_size: 5_000_000 }
    }
}

impl BenchModule for CpuCompress {
    fn name(&self) -> &'static str { "Cpu-Compress" }

    fn prepare(&mut self) -> Result<(), String> {
        log::info!("Cpu-Compress: prepared {} elements", self.data_size);
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let n = self.data_size;
        let src: Vec<u8> = (0..n).map(|i| (i % 256) as u8).collect();
        let mut samples = Vec::new();

        for _ in 0..3 {
            // Simulate compression: run-length encode pairs
            let start = Instant::now();
            let mut out = Vec::with_capacity(n / 2);
            let mut i = 0;
            while i < n {
                let b = src[i];
                let mut count = 1u16;
                i += 1;
                while i < n && src[i] == b && count < 65535 {
                    count += 1;
                    i += 1;
                }
                out.push(b);
                out.extend_from_slice(&count.to_le_bytes());
                std::hint::black_box(&out);
            }
            let elapsed = start.elapsed();
            let ratio = if n > 0 { out.len() as f64 / n as f64 } else { 1.0 };
            let throughput = n as f64 / elapsed.as_secs_f64() / 1_000_000.0;
            log::info!("Cpu-Compress: {:.2} MB/s, ratio={:.3}", throughput, ratio);
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: throughput,
                label: "MB/s".into(),
            });
        }
        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> { Ok(()) }
}
