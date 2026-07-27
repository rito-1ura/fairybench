use std::time::Instant;
use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// Cpu-Float: 浮動小数点演算（行列乗算）
pub struct CpuFloat {
    matrix_size: usize,
}

impl CpuFloat {
    pub fn new() -> Self {
        Self::with_size(512)
    }
    pub fn with_size(matrix_size: usize) -> Self {
        Self { matrix_size }
    }
}

impl BenchModule for CpuFloat {
    fn name(&self) -> &'static str { "Cpu-Float" }

    fn prepare(&mut self) -> Result<(), String> {
        log::info!("Cpu-Float: prepared {}x{} matrix", self.matrix_size, self.matrix_size);
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let n = self.matrix_size;
        let mut samples = Vec::new();

        for _ in 0..3 {
            let a: Vec<f64> = (0..n*n).map(|i| (i as f64 * 0.001).sin()).collect();
            let b: Vec<f64> = (0..n*n).map(|i| (i as f64 * 0.002).cos()).collect();
            let mut c = vec![0.0f64; n*n];

            let start = Instant::now();
            // Simple tiled matrix multiply
            let tile = 32;
            for ti in (0..n).step_by(tile) {
                let imax = (ti + tile).min(n);
                for tj in (0..n).step_by(tile) {
                    let jmax = (tj + tile).min(n);
                    for tk in (0..n).step_by(tile) {
                        let kmax = (tk + tile).min(n);
                        for i in ti..imax {
                            let row_offset = i * n;
                            for k in tk..kmax {
                                let aik = a[row_offset + k];
                                let b_row_offset = k * n;
                                for j in tj..jmax {
                                    c[row_offset + j] += aik * b[b_row_offset + j];
                                }
                            }
                        }
                    }
                }
            }
            let elapsed = start.elapsed();
            let flops = (2.0 * n as f64).powi(3) / elapsed.as_secs_f64() / 1e9;
            let _ = std::hint::black_box(&c);

            log::info!("Cpu-Float: {:.2} GFLOPS ({}ms)", flops, elapsed.as_millis());
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: flops,
                label: "GFLOPS".into(),
            });
        }
        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> { Ok(()) }
}
