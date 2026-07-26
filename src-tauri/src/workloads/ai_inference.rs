use std::time::Instant;

use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// AI-Inference: CPU行列乗算による推論性能シミュレーション
/// 512x512 行列乗算 (~134 MFLOP)
pub struct AiInference {
    matrix_size: usize,
}

impl AiInference {
    pub fn new() -> Self {
        Self {
            matrix_size: 512,   // 512×512 (reduced from 1024 for speed)
        }
    }
}

impl BenchModule for AiInference {
    fn name(&self) -> &'static str {
        "AI-Inference"
    }

    fn prepare(&mut self) -> Result<(), String> {
        let n = self.matrix_size as f64;
        log::info!(
            "AI-Inference: prepared {}x{} matrix (~{:.1} MFLOP)",
            self.matrix_size, self.matrix_size,
            2.0 * n * n * n / 1e6,
        );
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let n = self.matrix_size;
        let mut samples = Vec::new();

        // 3 iterations
        for iter in 0..3 {
            let mut rng = 12345u32 + iter;
            let mut a = vec![0.0f64; n * n];
            let mut b = vec![0.0f64; n * n];
            let mut c = vec![0.0f64; n * n];

            for i in 0..n * n {
                rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
                a[i] = (rng >> 16) as f64 / 65536.0;
                rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
                b[i] = (rng >> 16) as f64 / 65536.0;
            }

            let start = Instant::now();

            // i-k-j ループ
            for i in 0..n {
                for k in 0..n {
                    let aik = a[i * n + k];
                    if aik.abs() < 1e-10 { continue; }
                    let row_offset = i * n;
                    for j in 0..n {
                        c[row_offset + j] += aik * b[k * n + j];
                    }
                }
            }

            let elapsed = start.elapsed();
            let flops = 2.0 * n as f64 * n as f64 * n as f64;
            let gflops = if elapsed.as_secs_f64() > 0.0 {
                flops / elapsed.as_secs_f64() / 1e9
            } else { 0.0 };

            log::info!("AI-Inference iteration {}: {:.2} GFLOP/s ({}ms)",
                iter + 1, gflops, elapsed.as_millis());
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: gflops,
                label: "inference_score".into(),
            });
        }

        log::info!("AI-Inference completed with {} samples", samples.len());
        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> {
        log::info!("AI-Inference: teardown complete");
        Ok(())
    }
}
