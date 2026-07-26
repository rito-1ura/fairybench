# FairyBench

> GPU / CPU / Memory / Storage / AI — 総合デスクトップベンチマーク

[![Tauri](https://img.shields.io/badge/Tauri-2.x-818CF8?style=flat-square)](https://v2.tauri.app)
[![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00?style=flat-square)](https://svelte.dev)
[![Rust](https://img.shields.io/badge/Rust-1.97-000000?style=flat-square)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-22C55E?style=flat-square)](LICENSE)

## Overview

FairyBench は現実の GPU・CPU ワークロードを模した 8 種類のベンチマークモジュールを搭載した Windows 向け総合ベンチマークツールです。wgpu による GPU レンダリングからディスク IOPS、AI 推論シミュレーションまで、システム全体のパフォーマンスを定量評価します。

### Key Features

- **8 ベンチマークモジュール** — Raster/PathTrace/3DScene レンダリング、Procedural、Memory Bandwidth、Storage Throughput、AI Inference、AI Generative
- **FairyScore 統計エンジン** — 95% CI、IQR 除外、CV によるばらつき指標
- **リアルタイムストリーミング** — 各モジュールの実行状況をライブ表示
- **拡張サーマルモニタリング** — CPU/GPU 温度、負荷率、クロック、消費電力、ファン回転数
- **AAA ゲーム要件解析** — 22 タイトルとスコア比較。予想 FPS・快適度自動判定
- **A/B 比較** — 過去実行結果を横並び比較
- **スコア推移グラフ** — 履歴を棒＋折れ線の SVG チャートで可視化
- **リーダーボード** — 実行結果ランキング。クリックで詳細表示
- **2 表示モード** — ダッシュボード（全情報） / シンプル（スコア重視）
- **5 プリセット** — Quick / Standard / Precision / Max / Extreme

## Modules

| Module | Description | Unit |
|--------|-------------|------|
| Render-Raster | 三角形ラスタライズ (wgpu) | M verts/s |
| Render-PathTrace | パストレーシング (wgpu compute) | M rays/s |
| Render-3DScene | 3D シーンレンダリング (wgpu) | FPS |
| Render-Procedural | CPU プロシージャルメッシュ生成 | particles/s |
| Memory-Bandwidth | STREAM 相当メモリ帯域 | GB/s |
| Storage-Throughput | シーケンシャル R/W + ランダム 4K IOPS | MB/s / IOPS |
| AI-Inference | CPU 行列乗算 (ML 推論模擬) | GLFOP/s |
| AI-Generative | CPU テキスト生成模擬 | tokens/s |

## System Requirements

- OS: Windows 10 / 11 (64-bit)
- GPU: NVIDIA, AMD, or Intel GPU with Vulkan/DirectX 12 support
- CPU: x86-64, 4+ cores
- RAM: 8 GB minimum
- Storage: 200 MB free

## Installation

### From Releases

1. Go to the [Releases page](https://github.com/rito-1ura/fairybench/releases)
2. Download the latest `.msi` or `.exe` installer
3. Run the installer and follow the prompts

### Build from Source

```bash
# Prerequisites: Rust 1.77+, Node.js 20+, Tauri CLI

# Clone
git clone https://github.com/rito-1ura/fairybench.git
cd fairybench

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Usage

1. Launch FairyBench
2. Select preset (Quick / Standard / Precision / Max / Extreme)
3. Click **Run All** — 7–8 modules execute sequentially with real-time progress display
4. View results: FairyScore, per-module scores, 95% CI, CV
5. Use **Dashboard** for full metrics; **Simple** for focused score view
6. Compare past runs via **A/B Comparison** in the stats panel
7. Click leaderboard entries for detailed module breakdowns
8. Use **Game Analysis** after a run to check AAA game compatibility

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | Svelte 5, TypeScript, Vite |
| Backend | Rust, wgpu 0.23, rusqlite |
| Framework | Tauri 2.x |
| Graphics API | DirectX 12 / Vulkan / Metal (via wgpu) |
| Storage | SQLite (bundled) |
| Font | Figtree (Google Fonts) |

## Project Structure

```
fairybench/
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── DashboardMode.svelte   # Full dashboard view
│   │   ├── SimpleMode.svelte      # Simple score view
│   │   └── components/
│   │       └── StatsPanel.svelte  # History, A/B compare, chart
│   └── app.css             # Global styles (Figtree, theme, animations)
├── src-tauri/              # Rust backend
│   └── src/
│       ├── lib.rs          # Tauri commands (run_benchmark, analyze_games, etc.)
│       ├── db/             # SQLite database layer
│       ├── games.rs        # AAA game requirements database
│       ├── orchestrator/   # Module orchestration & results
│       ├── stats/          # Statistical engine
│       ├── thermal/        # WMIC + nvidia-smi sensor polling
│       └── workloads/      # 8 benchmark modules
│           ├── render_raster.rs
│           ├── render_pathtrace.rs
│           ├── render_scene3d.rs
│           ├── render_procedural.rs
│           ├── memory_bandwidth.rs
│           ├── storage_throughput.rs
│           ├── ai_inference.rs
│           └── ai_generative.rs
├── landing/                # Vercel-deployed landing page
└── vercel.json
```

## Scoring

**FairyScore** is computed as the weighted sum of all module sub-scores:

- Each module runs 3+ iterations (configurable via preset)
- Outliers excluded via IQR method
- 95% confidence interval reported
- Coefficient of Variation (CV) indicates result consistency
- Overall score = sum of per-module normalized scores

## License

MIT

---

Built with [Tauri](https://v2.tauri.app), [Svelte](https://svelte.dev), and [Rust](https://www.rust-lang.org).
