use std::time::Instant;

use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// Memory-Bandwidth: STREAMベンチマーク相当のメモリ帯域測定
pub struct MemoryBandwidth {
    array_size: usize,
}

impl MemoryBandwidth {
    pub fn new() -> Self {
        Self {
            array_size: 2_097_152, // 2M elements × 8 bytes = 16 MB per array (reduced from 8M)
        }
    }
}

impl BenchModule for MemoryBandwidth {
    fn name(&self) -> &'static str {
        "Memory-Bandwidth"
    }

    fn prepare(&mut self) -> Result<(), String> {
        log::info!(
            "Memory-Bandwidth: prepared {} elements (~{} MB per array)",
            self.array_size,
            (self.array_size * 8) / (1024 * 1024),
        );
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let n = self.array_size;
        let scalar = 3.0f64;

        let mut a = vec![1.0f64; n];
        let mut b = vec![2.0f64; n];
        let mut c = vec![0.0f64; n];

        let bytes_per_iter = (n * 8) as f64;
        let mut samples = Vec::new();

        // Copy: c = a
        {
            let start = Instant::now();
            for i in 0..n {
                c[i] = a[i];
            }
            let elapsed = start.elapsed();
            let gb_s = if elapsed.as_secs_f64() > 0.0 {
                (bytes_per_iter * 2.0) / elapsed.as_secs_f64() / 1e9
            } else { 0.0 };
            log::info!("Memory-Bandwidth Copy: {:.2} GB/s ({}ms)", gb_s, elapsed.as_millis());
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: gb_s,
                label: "copy_score".into(),
            });
        }

        // Scale: b = scalar * c
        {
            let start = Instant::now();
            for i in 0..n {
                b[i] = scalar * c[i];
            }
            let elapsed = start.elapsed();
            let gb_s = if elapsed.as_secs_f64() > 0.0 {
                (bytes_per_iter * 2.0) / elapsed.as_secs_f64() / 1e9
            } else { 0.0 };
            log::info!("Memory-Bandwidth Scale: {:.2} GB/s ({}ms)", gb_s, elapsed.as_millis());
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: gb_s,
                label: "scale_score".into(),
            });
        }

        // Add: c = a + b
        {
            let start = Instant::now();
            for i in 0..n {
                c[i] = a[i] + b[i];
            }
            let elapsed = start.elapsed();
            let gb_s = if elapsed.as_secs_f64() > 0.0 {
                (bytes_per_iter * 3.0) / elapsed.as_secs_f64() / 1e9
            } else { 0.0 };
            log::info!("Memory-Bandwidth Add: {:.2} GB/s ({}ms)", gb_s, elapsed.as_millis());
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: gb_s,
                label: "add_score".into(),
            });
        }

        // Triad: a = b + scalar * c
        {
            let start = Instant::now();
            for i in 0..n {
                a[i] = b[i] + scalar * c[i];
            }
            let elapsed = start.elapsed();
            let gb_s = if elapsed.as_secs_f64() > 0.0 {
                (bytes_per_iter * 3.0) / elapsed.as_secs_f64() / 1e9
            } else { 0.0 };
            log::info!("Memory-Bandwidth Triad: {:.2} GB/s ({}ms)", gb_s, elapsed.as_millis());
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: gb_s,
                label: "triad_score".into(),
            });
        }

        log::info!("Memory-Bandwidth completed with {} samples", samples.len());
        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> {
        log::info!("Memory-Bandwidth: teardown complete");
        Ok(())
    }
}
