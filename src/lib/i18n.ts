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
