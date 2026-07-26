use std::time::Instant;

use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// AI-Generative: CPUによるテキスト生成シミュレーション
pub struct AiGenerative {
    vocab_size: usize,
    sequence_length: usize,
}

impl AiGenerative {
    pub fn new() -> Self {
        Self {
            vocab_size: 10000,        // 10K vocab (reduced from 32K)
            sequence_length: 100_000, // 100K tokens (reduced from 500K)
        }
    }
}

impl BenchModule for AiGenerative {
    fn name(&self) -> &'static str {
        "AI-Generative"
    }

    fn prepare(&mut self) -> Result<(), String> {
        log::info!(
            "AI-Generative: prepared {} vocab, {} tokens",
            self.vocab_size, self.sequence_length,
        );
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let mut samples = Vec::new();

        for iter in 0..3 {
            // 重みテーブル生成
            let mut rng = 88888u32 + iter * 1000;
            let mut weights = vec![0.0f64; self.vocab_size];
            let mut total_weight = 0.0;
            for w in &mut weights {
                rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
                *w = (rng >> 16) as f64 + 0.1;
                total_weight += *w;
            }

            // CDF 構築 (累積分布)
            let mut cdf = Vec::with_capacity(self.vocab_size);
            let mut cum = 0.0;
            for w in &weights {
                cum += w / total_weight;
                cdf.push(cum);
            }

            let start = Instant::now();

            // トークン生成
            let mut state = 42u32;
            let mut _output_count = 0u32;
            for _ in 0..self.sequence_length {
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                let r: f64 = (state >> 16) as f64 / 65536.0;

                let _idx = match cdf.binary_search_by(|p| p.partial_cmp(&r).unwrap()) {
                    Ok(i) => i,
                    Err(i) => i.min(self.vocab_size - 1),
                };
                _output_count += 1;
            }

            let elapsed = start.elapsed();
            let tokens_per_sec = if elapsed.as_secs_f64() > 0.0 {
                self.sequence_length as f64 / elapsed.as_secs_f64()
            } else { 0.0 };

            log::info!("AI-Generative iteration {}: {:.0} tokens/sec ({}ms)",
                iter + 1, tokens_per_sec, elapsed.as_millis());
            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: tokens_per_sec / 1000.0, // K tokens/sec
                label: "generation_score".into(),
            });
        }

        log::info!("AI-Generative completed with {} samples", samples.len());
        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> {
        log::info!("AI-Generative: teardown complete");
        Ok(())
    }
}
