export type Lang = 'ja' | 'en' | 'default'

const _default: Record<string, string> = {}
const en: Record<string, string> = {
  'app.title': 'FairyBench',
  'leaderboard': 'Leaderboard',
  'modules': 'Modules',
  'thermal': 'Thermal',
  'score': 'Score',
  'stats': 'Statistics',
  'run_all': 'Run All',
  'stop': 'Stop',
  'simple_mode': 'Simple Mode',
  'game_analysis': 'Game Analysis',
  'score_history': 'Score History',
  'run_history': 'Run History',
  'latest': 'Latest',
  'streaming': 'Streaming',
  'standby': 'Standby',
  'empty_leaderboard': 'Run a benchmark to appear here',
  'empty_history': 'No results yet',
  'waiting_modules': 'Waiting for modules...',
  'placeholder_modules': 'Run benchmark to see module scores stream in real-time',
  'final_score': 'Final Score',
  'ci_95': '95% CI',
  'cv': 'CV',
  'runs': 'runs',
  'accepted_excluded': 'Accepted / Excluded',
  'no_sensor': 'No sensor',
  'live': 'Live',
  'fps': 'FPS',
  'quality': 'Quality',
  'raw_score': 'Raw Score',
  'resolution': 'Resolution',
  'api': 'API',
  '3d_scene': '3D Scene Render',
  'disk_thru': 'Disk Throughput',
  'hash_thru': 'Hash Throughput',
  'compress_speed': 'Compress Speed',
  'sort_rate': 'Sort Rate',
  'fp_math': 'FP MATH',
  'clock': 'Clock',
  'power': 'Power',
  'load': 'Load',
  'fan': 'Fan',
  'ram': 'RAM',
  'vram': 'VRAM',
  'use_all': 'Use All Cores',
  'high_perf': 'High Performance',
  'fairy_score': 'FairyScore',

  // Landing page
  'lp_badge': 'v0.2.0 — Desktop Benchmark Suite',
  'lp_title_1': 'Measure Your Machine',
  'lp_title_2': 'Without Compromise',
  'lp_sub': "12 benchmark modules spanning GPU, storage, memory, AI, and CPU — powered by Rust, wgpu, and Tauri 2. Real results, not estimates.",
  'lp_dl_btn': 'Download for Windows',
  'lp_src_btn': 'Source Code',
  'lp_nav_modules': 'Modules',
  'lp_nav_specs': 'Specs',
  'lp_nav_faq': 'FAQ',
  'lp_nav_github': 'GitHub',
  'lp_nav_dl': 'Download',
  'lp_stat_1n': '12', 'lp_stat_1l': 'Benchmark Modules', 'lp_stat_1s': 'Across 5 categories',
  'lp_stat_2n': '5', 'lp_stat_2l': 'Categories', 'lp_stat_2s': 'GPU / Storage / RAM / AI / CPU',
  'lp_stat_3n': '3-5', 'lp_stat_3l': 'Minutes per Run', 'lp_stat_3s': 'Full suite benchmark',
  'lp_stat_4n': '1', 'lp_stat_4l': 'Desktop App', 'lp_stat_4s': 'Native. No web. No cloud.',
  'lp_mod_label': 'Benchmark Modules',
  'lp_mod_title': '12 modules, 5 categories',
  'lp_mod_sub': 'Every module runs in isolation. Results are scored, normalized, and logged to local SQLite with full history tracking.',
  'lp_spec_label': 'Technical Specifications',
  'lp_spec_title': 'Built for performance, measured in precision',
  'lp_spec_1t': 'Rendering Engine', 'lp_spec_1d': 'wgpu (WebGPU) with D3D12/Vulkan/Metal backends. GPU compute shaders for procedural generation and post-processing.',
  'lp_spec_2t': 'Desktop Native', 'lp_spec_2d': 'Tauri 2 framework. Single ~8MB executable. No Electron overhead. Native Windows API access.',
  'lp_spec_3t': 'Real-time Analysis', 'lp_spec_3d': 'Live scoring with animated transitions. Confidence intervals, CV, outlier detection. Per-module breakdown.',
  'lp_spec_4t': 'Privacy First', 'lp_spec_4d': 'All data stored locally in SQLite. Online leaderboard is opt-in. No telemetry, no analytics.',
  'lp_spec_5t': 'Fast Runs', 'lp_spec_5d': 'Full 12-module suite completes in 3-5 minutes. Concurrent GPU + CPU execution where possible.',
  'lp_spec_6t': 'Thermal Monitoring', 'lp_spec_6d': 'Real-time CPU/GPU temperature, power draw, and clock frequency sampled every 2 seconds.',
  'lp_dl_title': 'Ready to benchmark?',
  'lp_dl_sub': 'Download the latest release for Windows. No installer required — portable executable.',
  'lp_dl_btn_lg': 'Download v0.2.0',
  'lp_dl_gh': 'View on GitHub',
  'lp_dl_meta': 'Windows 10/11 · ~8 MB · Open Source (MIT)',
  'lp_faq_label': 'FAQ',
  'lp_faq_title': 'Common questions',
  'lp_faq_1q': 'Is FairyBench free?', 'lp_faq_1a': 'Yes. Fully open source under MIT license. No ads, no telemetry, no paywalls.',
  'lp_faq_2q': 'What hardware does it support?', 'lp_faq_2a': 'Windows 10/11 with any modern GPU supporting wgpu (D3D12/Vulkan/Metal). NVMe SSDs recommended.',
  'lp_faq_3q': 'How long does a benchmark take?', 'lp_faq_3a': 'Full suite runs in 3-5 minutes. Individual modules run in 15-60 seconds each.',
  'lp_faq_4q': 'Can I compare results online?', 'lp_faq_4a': 'Yes! Results are automatically submitted to the global leaderboard. View rankings at fairybench.vercel.app.',
  'lp_faq_5q': 'Does it affect system performance?', 'lp_faq_5a': "Benchmarks are read-only — they measure, they don't modify. Thermal monitoring is passive observation.",
  'lp_showcase_label': 'Desktop App Preview',
  'lp_showcase_title': 'See it in action',
  'lp_showcase_sub': 'Real-time benchmark dashboard with live scoring, thermal monitoring, and module telemetry.',
  'lp_footer_desc': 'Open-source PC benchmark suite. Built with Rust, Tauri 2, Svelte 5, and wgpu.',
}

const ja: Record<string, string> = {
  'app.title': 'FairyBench',
  'leaderboard': 'リーダーボード',
  'modules': 'モジュール',
  'thermal': 'サーマル',
  'score': 'スコア',
  'stats': '統計解析',
  'run_all': 'すべて実行',
  'stop': '停止',
  'simple_mode': 'シンプルモード',
  'game_analysis': 'ゲーム解析',
  'score_history': 'スコア推移',
  'run_history': '実行履歴',
  'latest': '最新',
  'streaming': '測定中',
  'standby': '待機中',
  'empty_leaderboard': 'ベンチマークを実行するとここに表示されます',
  'empty_history': 'まだ実行結果がありません',
  'waiting_modules': 'モジュール待機中...',
  'placeholder_modules': 'ベンチマークを実行するとモジュールスコアがリアルタイム表示されます',
  'final_score': '最終スコア',
  'ci_95': '95% CI',
  'cv': 'CV',
  'runs': '回',
  'accepted_excluded': '採用 / 除外',
  'no_sensor': 'センサーなし',
  'live': '動作中',
  'fps': 'FPS',
  'quality': '品質',
  'raw_score': '生スコア',
  'resolution': '解像度',
  'api': 'API',
  '3d_scene': '3Dシーンレンダリング',
  'disk_thru': 'ディスクスループット',
  'hash_thru': 'ハッシュスループット',
  'compress_speed': '圧縮速度',
  'sort_rate': 'ソート速度',
  'fp_math': 'FP演算',
  'clock': 'クロック',
  'power': '消費電力',
  'load': '負荷',
  'fan': 'ファン',
  'ram': 'RAM',
  'vram': 'VRAM',
  'use_all': '全コア使用',
  'high_perf': '高パフォーマンス',
  'fairy_score': 'FairyScore',

  // Landing page
  'lp_badge': 'v0.2.0 — デスクトップベンチマーク',
  'lp_title_1': 'あなたのマシンを',
  'lp_title_2': '徹底測定',
  'lp_sub': "GPU、ストレージ、メモリ、AI、CPUの12モジュールを搭載。Rust、wgpu、Tauri 2で構築された本格派ベンチマーク。",
  'lp_dl_btn': 'Windows版をダウンロード',
  'lp_src_btn': 'ソースコード',
  'lp_nav_modules': 'モジュール',
  'lp_nav_specs': 'スペック',
  'lp_nav_faq': 'よくある質問',
  'lp_nav_github': 'GitHub',
  'lp_nav_dl': 'ダウンロード',
  'lp_stat_1n': '12', 'lp_stat_1l': 'ベンチマークモジュール', 'lp_stat_1s': '5カテゴリ',
  'lp_stat_2n': '5', 'lp_stat_2l': 'カテゴリ', 'lp_stat_2s': 'GPU / ストレージ / RAM / AI / CPU',
  'lp_stat_3n': '3-5', 'lp_stat_3l': '分で完了', 'lp_stat_3s': 'フルスイート',
  'lp_stat_4n': '1', 'lp_stat_4l': 'デスクトップアプリ', 'lp_stat_4s': 'ネイティブ。Web不要。クラウド不要。',
  'lp_mod_label': 'ベンチマークモジュール',
  'lp_mod_title': '12モジュール、5カテゴリ',
  'lp_mod_sub': '各モジュールは独立して実行。結果はスコア化・正規化され、SQLiteに履歴保存されます。',
  'lp_spec_label': '技術仕様',
  'lp_spec_title': '精密測定のために設計',
  'lp_spec_1t': 'レンダリングエンジン', 'lp_spec_1d': 'wgpu (WebGPU) を採用。D3D12/Vulkan/Metalバックエンド。GPUコンピュートシェーダーでプロシージャル生成。',
  'lp_spec_2t': 'デスクトップネイティブ', 'lp_spec_2d': 'Tauri 2フレームワーク。約8MBの単一実行ファイル。Electron不要。ネイティブWindows APIにアクセス。',
  'lp_spec_3t': 'リアルタイム解析', 'lp_spec_3d': 'アニメーション付きライブスコアリング。信頼区間、CV、外れ値検出。モジュール別内訳と履歴トレンド。',
  'lp_spec_4t': 'プライバシー最優先', 'lp_spec_4d': '全データはローカルSQLiteに保存。オンラインリーダーボードはオプトイン。テレメトリなし。',
  'lp_spec_5t': '高速実行', 'lp_spec_5d': '12モジュールのフルスイートは3-5分で完了。各モジュール15-60秒。GPU+CPU同時実行可能。',
  'lp_spec_6t': '温度監視', 'lp_spec_6d': 'ベンチマーク中、CPU/GPUの温度、消費電力、クロック周波数を2秒間隔でサンプリング。',
  'lp_dl_title': 'ベンチマークを始めましょう',
  'lp_dl_sub': 'Windows向け最新リリースをダウンロード。インストーラー不要のポータブル実行ファイル。',
  'lp_dl_btn_lg': 'v0.2.0 をダウンロード',
  'lp_dl_gh': 'GitHubで見る',
  'lp_dl_meta': 'Windows 10/11 · ~8 MB · オープンソース (MIT)',
  'lp_faq_label': 'よくある質問',
  'lp_faq_title': 'よくある質問',
  'lp_faq_1q': 'FairyBenchは無料ですか？', 'lp_faq_1a': 'はい。MITライセンスの完全オープンソースです。広告、テレメトリ、有料化はありません。',
  'lp_faq_2q': '対応ハードウェアは？', 'lp_faq_2a': 'Windows 10/11、wgpu対応GPU（D3D12/Vulkan/Metal）。NVMe SSD推奨。',
  'lp_faq_3q': 'ベンチマークの所要時間は？', 'lp_faq_3a': 'フルスイートは3-5分。個別モジュールは15-60秒です。',
  'lp_faq_4q': 'オンラインで結果を比較できますか？', 'lp_faq_4a': 'はい。グローバルリーダーボードに自動送信されます。fairybench.vercel.appでランキングを確認。',
  'lp_faq_5q': 'システムに影響はありますか？', 'lp_faq_5a': 'ベンチマークは読み取り専用です。測定のみ行い、変更は加えません。温度監視は受動的観測です。',
  'lp_showcase_label': 'デスクトップアプリプレビュー',
  'lp_showcase_title': '実際の動作画面',
  'lp_showcase_sub': 'リアルタイムベンチマークダッシュボード。ライブスコアリング、温度監視、モジュールテレメトリ。',
  'lp_footer_desc': 'オープンソースPCベンチマークスイート。Rust、Tauri 2、Svelte 5、wgpuで構築。',
}

export function t(key: string): string {
  const l = (typeof localStorage !== 'undefined' ? localStorage.getItem('lang') : null) as Lang | null
  if (l === 'en' && en[key]) return en[key]
  if (l === 'ja' && ja[key]) return ja[key]
  return _default[key] || key
}

export function setLang(l: Lang) {
  if (typeof localStorage !== 'undefined') localStorage.setItem('lang', l)
}

export function getLang(): Lang {
  return (typeof localStorage !== 'undefined' ? localStorage.getItem('lang') : null) as Lang || 'default'
}

export function formatScore(v: number, lang?: Lang): string {
  const l = lang || getLang()
  if (l === 'en') return v.toLocaleString('en-US')
  if (l === 'ja') {
    const abs = Math.abs(v)
    const sign = v < 0 ? '-' : ''
    const n = Math.floor(abs)
    const oku = Math.floor(n / 100_000_000)
    const man = Math.floor((n % 100_000_000) / 10_000)
    const rem = n % 10_000
    let result = ''
    if (oku > 0) result += oku + '億'
    if (man > 0) result += man + '万'
    if (rem > 0 || (oku === 0 && man === 0)) result += String(rem)
    if (result === '') result = '0'
    return sign + result
  }
  return v.toLocaleString()
}

export function formatShortScore(v: number): string {
  const l = getLang()
  if (l === 'en') {
    if (v >= 1_000_000) return (v / 1_000_000).toFixed(2) + 'M'
    if (v >= 1_000) return (v / 1_000).toFixed(1) + 'K'
    return v.toFixed(0)
  }
  // default/ja
  if (v >= 1_000_000) return (v / 1_000_000).toFixed(2) + 'M'
  if (v >= 1_000) return (v / 1_000).toFixed(1) + 'K'
  return v.toFixed(0)
}
