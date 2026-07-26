use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::time::Instant;

use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// 拡張ストレージテスト: シーケンシャルR/W + ランダム4K IOPS + レイテンシ
pub struct StorageThroughput {
    file_size_mb: u64,
    block_size: usize,
    _random_blocks: usize,
}

impl StorageThroughput {
    pub fn new() -> Self {
        Self {
            file_size_mb: 64,
            block_size: 1024 * 1024, // 1MB sequential
            _random_blocks: 2000,     // 2000× 4K random IOPS
        }
    }
}

impl BenchModule for StorageThroughput {
    fn name(&self) -> &'static str { "Storage-Throughput" }

    fn prepare(&mut self) -> Result<(), String> {
        log::info!("Storage-Throughput: {}MB file, {}KB sequential / 4K random", self.file_size_mb, self.block_size / 1024);
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let tmp = std::env::temp_dir().join("fairybench_storage_tmp.bin");
        let total_bytes = self.file_size_mb * 1024 * 1024;
        let blocks = (total_bytes / self.block_size as u64) as usize;
        let data = vec![0xABu8; self.block_size];
        let mut samples = Vec::new();

        // === Sequential Write ===
        {
            let file = File::create(&tmp).map_err(|e| format!("Create: {e}"))?;
            let mut w = std::io::BufWriter::with_capacity(self.block_size, file);
            let start = Instant::now();
            for _ in 0..blocks {
                w.write_all(&data).map_err(|e| format!("Write: {e}"))?;
            }
            w.flush().map_err(|e| format!("Flush: {e}"))?;
            let elapsed = start.elapsed();
            let mbps = if elapsed.as_secs_f64() > 0.0 { self.file_size_mb as f64 / elapsed.as_secs_f64() } else { 0.0 };
            log::info!("Seq Write: {:.0} MB/s ({}ms)", mbps, elapsed.as_millis());
            samples.push(MetricSample { timestamp_ms: chrono::Utc::now().timestamp_millis(), value: mbps, label: "seq_write_mbps".into() });
        }

        // === Sequential Read ===
        {
            let file = File::open(&tmp).map_err(|e| format!("Open: {e}"))?;
            let mut r = std::io::BufReader::with_capacity(self.block_size, file);
            let mut buf = vec![0u8; self.block_size];
            let start = Instant::now();
            for _ in 0..blocks {
                r.read_exact(&mut buf).map_err(|e| format!("Read: {e}"))?;
            }
            let elapsed = start.elapsed();
            let mbps = if elapsed.as_secs_f64() > 0.0 { self.file_size_mb as f64 / elapsed.as_secs_f64() } else { 0.0 };
            log::info!("Seq Read: {:.0} MB/s ({}ms)", mbps, elapsed.as_millis());
            samples.push(MetricSample { timestamp_ms: chrono::Utc::now().timestamp_millis(), value: mbps, label: "seq_read_mbps".into() });
        }

        // === Random 4K Read IOPS ===
        {
            let mut file = File::open(&tmp).map_err(|e| format!("Open rand: {e}"))?;
            let file_len = file.metadata().map(|m| m.len()).unwrap_or(total_bytes);
            let max_offset = file_len.saturating_sub(4096);
            let mut rng = 42u32;
            let mut buf = [0u8; 4096];
            let start = Instant::now();
            let count = self._random_blocks;
            for _ in 0..count {
                rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                let offset = (rng as u64) % max_offset;
                file.seek(SeekFrom::Start(offset)).map_err(|e| format!("Seek: {e}"))?;
                file.read_exact(&mut buf).map_err(|e| format!("Read: {e}"))?;
            }
            let elapsed = start.elapsed();
            let iops = if elapsed.as_secs_f64() > 0.0 { count as f64 / elapsed.as_secs_f64() } else { 0.0 };
            let latency_us = if iops > 0.0 { (1.0 / iops) * 1_000_000.0 } else { 0.0 };
            log::info!("Random 4K Read: {:.0} IOPS ({:.1}us avg)", iops, latency_us);
            samples.push(MetricSample { timestamp_ms: chrono::Utc::now().timestamp_millis(), value: iops, label: "rand_read_iops".into() });
        }

        let _ = std::fs::remove_file(&tmp);
        log::info!("Storage-Throughput completed: {} samples", samples.len());
        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> {
        log::info!("Storage-Throughput: teardown complete");
        Ok(())
    }
}
