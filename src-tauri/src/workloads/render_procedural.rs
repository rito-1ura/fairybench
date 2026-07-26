use std::time::Instant;

use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// CPU主体の幾何/物理演算を測定するRender-Proceduralモジュール
/// 粒子シミュレーション: N個の粒子の位置更新 + 衝突検出 + 重力計算
pub struct RenderProcedural {
    num_particles: usize,
    num_iterations: u32,
    particles: Vec<Particle>,
}

#[derive(Clone)]
struct Particle {
    pos_x: f64,
    pos_y: f64,
    pos_z: f64,
    vel_x: f64,
    vel_y: f64,
    vel_z: f64,
    mass: f64,
}

impl RenderProcedural {
    pub fn new() -> Self {
        Self {
            num_particles: 500_000,
            num_iterations: 10,
            particles: Vec::new(),
        }
    }

    /// 重力計算（全対全のN体問題を近似: セル分割はせず単純化）
    fn compute_gravity(&self, idx: usize) -> (f64, f64, f64) {
        let p = &self.particles[idx];
        let mut ax = 0.0;
        let mut ay = 0.0;
        let mut az = 0.0;

        // 近傍粒子のみ計算（全対全は O(n^2) で重すぎるため、
        // 近傍200粒子に限定して現実的な時間に収める）
        let start = if idx > 100 { idx - 100 } else { 0 };
        let end = (idx + 100).min(self.num_particles);

        for j in start..end {
            if j == idx { continue; }
            let other = &self.particles[j];
            let dx = other.pos_x - p.pos_x;
            let dy = other.pos_y - p.pos_y;
            let dz = other.pos_z - p.pos_z;
            let dist_sq = dx * dx + dy * dy + dz * dz + 1e-10;
            let inv_dist = 1.0 / dist_sq.sqrt();
            let force = other.mass * inv_dist * inv_dist * inv_dist;
            ax += dx * force;
            ay += dy * force;
            az += dz * force;
        }

        (ax, ay, az)
    }
}

impl BenchModule for RenderProcedural {
    fn name(&self) -> &'static str {
        "Render-Procedural"
    }

    fn prepare(&mut self) -> Result<(), String> {
        let mut rng_state = 55555u32;
        self.particles = (0..self.num_particles).map(|_| {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let px = ((rng_state >> 16) as f64 / 65536.0) * 100.0 - 50.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let py = ((rng_state >> 16) as f64 / 65536.0) * 100.0 - 50.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let pz = ((rng_state >> 16) as f64 / 65536.0) * 100.0 - 50.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let vx = ((rng_state >> 16) as f64 / 65536.0) * 2.0 - 1.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let vy = ((rng_state >> 16) as f64 / 65536.0) * 2.0 - 1.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let vz = ((rng_state >> 16) as f64 / 65536.0) * 2.0 - 1.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let mass = ((rng_state >> 16) as f64 / 65536.0) * 10.0 + 1.0;
            Particle { pos_x: px, pos_y: py, pos_z: pz, vel_x: vx, vel_y: vy, vel_z: vz, mass }
        }).collect();

        log::info!(
            "Render-Procedural: prepared {} particles, {} iterations",
            self.num_particles, self.num_iterations,
        );
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let mut samples = Vec::new();

        for iter in 0..self.num_iterations {
            let start = Instant::now();

            // Step 1: 重力計算 + 速度更新
            let mut accelerations = Vec::with_capacity(self.num_particles);
            for i in 0..self.num_particles {
                let (ax, ay, az) = self.compute_gravity(i);
                accelerations.push((ax, ay, az));
            }

            // Step 2: 位置更新
            let dt = 0.01;
            for i in 0..self.num_particles {
                let p = &mut self.particles[i];
                let (ax, ay, az) = accelerations[i];
                p.vel_x += ax * dt;
                p.vel_y += ay * dt;
                p.vel_z += az * dt;
                p.pos_x += p.vel_x * dt;
                p.pos_y += p.vel_y * dt;
                p.pos_z += p.vel_z * dt;
            }

            let elapsed = start.elapsed();

            // Metric: particle-updates per second
            let ops_per_sec = (self.num_particles as f64 * 2.0) / elapsed.as_secs_f64();

            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: ops_per_sec,
                label: "particles/sec".into(),
            });

            log::info!(
                "Render-Procedural iteration {}: {:.2}M particles/sec ({}ms)",
                iter + 1,
                ops_per_sec / 1_000_000.0,
                elapsed.as_millis()
            );
        }

        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> {
        self.particles.clear();
        log::info!("Render-Procedural: teardown complete");
        Ok(())
    }
}
