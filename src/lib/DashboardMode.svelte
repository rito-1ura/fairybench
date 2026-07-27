<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { listen } from '@tauri-apps/api/event'
  import StatsPanel from './components/StatsPanel.svelte'
  import Scene3DView from './components/Scene3DView.svelte'
  import DiskView from './components/DiskView.svelte'

  interface SubScore { module_name: string; raw_score: number; normalized_score: number }
  interface RunResult {
    run_id: string; overall_raw: number; ci_lower: number; ci_upper: number;
    cv: number; runs_used: number; runs_excluded: number;
    sub_scores: Record<string, SubScore>
  }
  interface SavedRun {
    run_id: string; executed_at: string; overall_raw: number;
    ci_lower: number; ci_upper: number; cv: number;
    runs_used: number; runs_excluded: number; hardware_config_hash: string
  }
  interface ThermalSample {
    timestamp_ms: number; cpu_temp_avg: number | null; gpu_temp: number | null;
    cpu_clock_ghz: number | null; gpu_clock_mhz: number | null; power_watts: number | null;
    cpu_load_pct: number | null; gpu_load_pct: number | null;
    gpu_mem_used_mb: number | null; gpu_mem_total_mb: number | null;
    fan_speed_pct: number | null; sys_mem_used_gb: number | null; sys_mem_total_gb: number | null;
    sensors_available: boolean;
  }
  interface GameAnalysis {
    title: string; category: string; status: string; status_jp: string;
    fps_est: string; score_pct: number; min_score: number; rec_score: number;
    gpu_min: string; gpu_rec: string; storage_gb: number; has_raytracing: boolean;
    details: string[];
  }
  interface DeviceInfo {
    adapter_name: string; backend: string; device_type: string;
    driver_info: string; os_info: string; cpu_name: string;
    cpu_cores: number; total_memory_gb: number; api_version: string
  }
  interface ModuleProgress {
    module: string; score: number; label: string; duration_ms: number; phase: string
  }

  let { onSwitch, version }: { onSwitch: () => void; version: string } = $props()

  // State
  let running = $state(false)
  let result = $state<RunResult | null>(null)
  let history = $state<SavedRun[]>([])
  let thermal = $state<ThermalSample | null>(null)
  let deviceInfo = $state<DeviceInfo | null>(null)
  let precision = $state<'quick' | 'standard' | 'high' | 'max' | 'extreme'>('standard')
  let currentModule = $state('Ready')
  let elapsed = $state(0)
  let timer: ReturnType<typeof setInterval> | null = null
  let completedModules = $state<{name:string; score:number; label:string}[]>([])
  let unlisten: (() => void) | null = null
  let currentRunScore = $state(0)
  let scoreAnimTimers: ReturnType<typeof setInterval>[] = []
  let destroyed = false
  let lastAddedScore = $state(0)
  let lastAddedName = $state('')

  // Show added score briefly, then fade
  let addTimer: ReturnType<typeof setTimeout> | null = null
  function flashAdded(score: number, name: string) {
    lastAddedScore = score; lastAddedName = name
    if (addTimer) clearTimeout(addTimer)
    addTimer = setTimeout(() => { lastAddedScore = 0; lastAddedName = '' }, 2500)
  }

  // Game analysis — frontend-side, no Rust invoke needed
  const GAMES = [
    { title:"Cyberpunk 2077", category:"AAA", min:8_000_000, rec:25_000_000, mem:32, gpu_min:"GTX 1060", gpu_rec:"RTX 3070", rt:true, storage:70 },
    { title:"Baldur's Gate 3", category:"AAA", min:6_000_000, rec:18_000_000, mem:16, gpu_min:"GTX 970", gpu_rec:"RTX 2060 Super", storage:150 },
    { title:"Elden Ring", category:"AAA", min:5_000_000, rec:15_000_000, mem:16, gpu_min:"GTX 1060", gpu_rec:"RTX 2070", storage:60 },
    { title:"Alan Wake 2", category:"AAA", min:10_000_000, rec:28_000_000, mem:32, gpu_min:"RTX 2060", gpu_rec:"RTX 4070", rt:true, storage:90 },
    { title:"Starfield", category:"AAA", min:7_000_000, rec:20_000_000, mem:32, gpu_min:"GTX 1070", gpu_rec:"RTX 2080", storage:125 },
    { title:"Black Myth: Wukong", category:"AAA", min:9_000_000, rec:26_000_000, mem:32, gpu_min:"RTX 2060", gpu_rec:"RTX 4080", rt:true, storage:130 },
    { title:"Call of Duty: MW III", category:"AAA", min:5_000_000, rec:16_000_000, mem:16, gpu_min:"GTX 1060", gpu_rec:"RTX 3060", rt:true, storage:150 },
    { title:"Hogwarts Legacy", category:"AAA", min:6_000_000, rec:18_000_000, mem:32, gpu_min:"GTX 960", gpu_rec:"RTX 2080", rt:true, storage:85 },
    { title:"The Last of Us Part I", category:"AAA", min:8_000_000, rec:22_000_000, mem:32, gpu_min:"GTX 1060", gpu_rec:"RTX 3060", storage:100 },
    { title:"God of War Ragnarok", category:"AAA", min:6_000_000, rec:18_000_000, mem:16, gpu_min:"GTX 1060", gpu_rec:"RTX 2070", storage:80 },
    { title:"Final Fantasy XVI", category:"AAA", min:8_000_000, rec:24_000_000, mem:32, gpu_min:"GTX 1070", gpu_rec:"RTX 3080", storage:100 },
    { title:"Tekken 8", category:"AAA", min:4_000_000, rec:12_000_000, mem:16, gpu_min:"GTX 1050", gpu_rec:"RTX 2060", storage:60 },
    { title:"Helldivers 2", category:"AAA", min:5_000_000, rec:15_000_000, mem:16, gpu_min:"GTX 1060", gpu_rec:"RTX 2070", storage:70 },
    { title:"MS Flight Sim 2024", category:"Simulation", min:10_000_000, rec:30_000_000, mem:32, gpu_min:"RTX 2060", gpu_rec:"RTX 4080", rt:true, storage:150 },
    { title:"Cities: Skylines II", category:"Simulation", min:5_000_000, rec:20_000_000, mem:32, gpu_min:"GTX 970", gpu_rec:"RTX 3080", storage:60 },
    { title:"Fortnite", category:"eSports", min:2_000_000, rec:10_000_000, mem:16, gpu_min:"Intel UHD", gpu_rec:"RTX 2060", rt:true, storage:40 },
    { title:"Valorant", category:"eSports", min:500_000, rec:5_000_000, mem:8, gpu_min:"Intel HD", gpu_rec:"GTX 1050", storage:30 },
    { title:"Apex Legends", category:"eSports", min:2_000_000, rec:8_000_000, mem:12, gpu_min:"GTX 660", gpu_rec:"GTX 1060", storage:60 },
    { title:"Overwatch 2", category:"eSports", min:1_500_000, rec:7_000_000, mem:12, gpu_min:"GTX 600", gpu_rec:"GTX 1060", storage:50 },
    { title:"Hades II", category:"Indie", min:1_000_000, rec:4_000_000, mem:16, gpu_min:"GTX 950", gpu_rec:"GTX 1060", storage:20 },
    { title:"Elden Ring: Nightreign", category:"AAA", min:6_000_000, rec:18_000_000, mem:16, gpu_min:"GTX 1060", gpu_rec:"RTX 2070", storage:60 },
    { title:"Avowed", category:"AAA", min:8_000_000, rec:24_000_000, mem:32, gpu_min:"RTX 2060", gpu_rec:"RTX 4070", rt:true, storage:100 },
  ]
  let gameAnalysis = $state<{title:string;status_jp:string;fps_est:string;notes:string}[]|null>(null)
  let showGameAnalysis = $state(false)
  function loadGameAnalysis() {
    if (!result) return
    const score = result.overall_raw
    const analysis = GAMES.map(g => {
      const pct = g.rec > 0 ? Math.min(200, score / g.rec * 100) : 0
      let status_jp: string, fps_est: string, notes: string
      if (score >= g.rec) {
        const fps = pct >= 150 ? '90-144 FPS' : pct >= 120 ? '60-90 FPS' : '60+ FPS'
        status_jp = '快適'; fps_est = fps
        notes = `${pct.toFixed(0)}% of recommended`
      } else if (score >= g.min) {
        const fps = pct >= 80 ? '45-60 FPS' : '30-45 FPS'
        status_jp = '可'; fps_est = fps
        notes = `${pct.toFixed(0)}% of recommended`
      } else if (score >= g.min * 0.7) {
        status_jp = '限界'; fps_est = 'Below 30 FPS'
        notes = `${(score / g.min * 100).toFixed(0)}% of minimum`
      } else {
        status_jp = '不可'; fps_est = '—'
        notes = `${(score / Math.max(g.min,1) * 100).toFixed(0)}% of minimum`
      }
      if (g.rt && status_jp === '快適') notes += ' · RT possible'
      else if (g.rt && status_jp === '可') notes += ' · RT not recommended'
      else if (g.rt) notes += ' · RT unavailable'
      notes += ` · ${g.storage}GB`
      return { title: g.title, status_jp, fps_est, notes }
    })
    gameAnalysis = analysis
    showGameAnalysis = true
  }

  // Leaderboard detail modal
  let detailResult = $state<RunResult | null>(null)
  let showDetailModal = $state(false)

  // Realtime chart data
  let chartData = $state<{time: number; score: number}[]>([])

  // Formatters
  const formatScore = (v: number): string => v.toLocaleString('en-US', { maximumFractionDigits: 0 })
  const formatTime = (s: number): string => `${String(Math.floor(s / 60)).padStart(2, '0')}:${String(s % 60).padStart(2, '0')}`
  const formatShort = (v: number): string => {
    if (v >= 1e9) return (v / 1e9).toFixed(2) + 'B'
    if (v >= 1e6) return (v / 1e6).toFixed(2) + 'M'
    if (v >= 1e3) return (v / 1e3).toFixed(1) + 'K'
    return v.toFixed(1)
  }
  const formatPct = (v: number): string => (v * 100).toFixed(1) + '%'
  const formatTemp = (v: number | null): string => v != null ? v.toFixed(0) + '°C' : '--'
  const formatClock = (v: number | null, unit: string): string => v != null ? v.toFixed(2) + ' ' + unit : '--'
  const formatWatts = (v: number | null): string => v != null ? v.toFixed(1) + ' W' : '--'

  // SVG icons as strings
  const svgs = {
    cpu: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M9 9h6v6H9z"/><path d="M9 2v2M15 2v2M9 20v2M15 20v2M2 9h2M2 15h2M20 9h2M20 15h2"/></svg>',
    gpu: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2" y="4" width="20" height="14" rx="2"/><circle cx="12" cy="11" r="4"/><path d="M16 18v2M8 18v2"/></svg>',
    storage: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M21 12c0 1.66-4.03 3-9 3s-9-1.34-9-3"/><path d="M3 5v14c0 1.66 4.03 3 9 3s9-1.34 9-3V5"/></svg>',
    memory: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="4" y="4" width="16" height="16" rx="2"/><rect x="7" y="7" width="3" height="10" rx="1"/><rect x="14" y="7" width="3" height="10" rx="1"/></svg>',
    ai: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 2a7 7 0 0 1 7 7c0 2.4-1.2 4.5-3 5.7V17H8v-2.3C6.2 13.5 5 11.4 5 9a7 7 0 0 1 7-7z"/><path d="M9 17h6"/><path d="M10 20h4"/></svg>',
    trophy: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M6 9H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h2"/><path d="M18 9h2a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2h-2"/><path d="M4 3h16v5c0 4.42-3.58 8-8 8s-8-3.58-8-8V3z"/><path d="M12 16v5"/><path d="M8 21h8"/></svg>',
    thermometer: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z"/></svg>',
    chart: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 20V10"/><path d="M9 20V6"/><path d="M14 20V11"/><path d="M19 20V14"/></svg>',
    lightning: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M13 2 3 14h9l-1 8 10-12h-9l1-8z"/></svg>',
    refresh: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 12a9 9 0 1 1-9-9"/><path d="M21 3v5h-5"/></svg>',
    play: '<svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>',
    stop: '<svg viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="6" width="12" height="12" rx="2"/></svg>',
    bolt: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>',
    info: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><circle cx="12" cy="8" r="0.5" fill="currentColor"/></svg>',
  }

  const moduleIcons: Record<string, string> = {
    'Render-Raster': svgs.gpu,
    'Render-PathTrace': svgs.gpu,
    'Render-Procedural': svgs.cpu,
    'Storage-Throughput': svgs.storage,
    'Memory-Bandwidth': svgs.memory,
    'AI-Inference': svgs.ai,
    'AI-Generative': svgs.ai,
  }

  // Dev info
  async function loadDeviceInfo() {
    try { deviceInfo = await invoke<DeviceInfo>('get_device_info') } catch {}
  }
  $effect(() => { loadDeviceInfo() })

  // History
  async function loadHistory() {
    try { history = await invoke<SavedRun[]>('list_results') } catch {}
  }
  $effect(() => { loadHistory() })

  // Clear all animation timers
  function clearAllTimers() {
    for (const t of scoreAnimTimers) clearInterval(t)
    scoreAnimTimers = []
  }

  // Score counter animation (count-up effect)
  const SCORE_ANIM_DURATION = 600
  function animateScore(from: number, to: number, cb: (v: number) => void) {
    if (destroyed) return
    const start = performance.now()
    const id = setInterval(() => {
      const elapsed = performance.now() - start
      const pct = Math.min(1, elapsed / SCORE_ANIM_DURATION)
      cb(from + (to - from) * pct)
      if (pct >= 1) clearInterval(id)
    }, 16)
    scoreAnimTimers = [...scoreAnimTimers, id]
  }

  // Run benchmark
  async function runBenchmark() {
    running = true
    result = null
    completedModules = []
    elapsed = 0
    currentRunScore = 0
    currentModule = 'Starting...'
    destroyed = false
    clearAllTimers()

    // Listen for streaming events
    unlisten = await listen<ModuleProgress>('benchmark-event', (event) => {
      if (destroyed) return
      const data = event.payload
      if (data.phase === 'start') {
        currentModule = data.module
      } else if (data.phase === 'complete') {
        const entry = { name: data.module, score: data.score, label: data.label }
        completedModules = [...completedModules, entry]
        const oldScore = currentRunScore
        const newScore = currentRunScore + data.score
        animateScore(oldScore, newScore, (v) => { if (!destroyed) currentRunScore = v })
        flashAdded(data.score, data.module)
      } else if (data.phase === 'pulse') {
        // Real-time module progress — update elapsed time display
        currentModule = data.module + ' (' + data.label + ')'
      }
    })

    timer = setInterval(() => { if (!destroyed) elapsed++ }, 1000)

    const thermalInt = setInterval(async () => {
      if (destroyed) return
      try { thermal = await invoke<ThermalSample>('get_thermal_snapshot') } catch {}
    }, 2000)

    try {
      const res = await invoke<RunResult>('run_benchmark')
      if (!destroyed) {
        result = res
        currentModule = 'Complete'
        currentRunScore = res.overall_raw
        await loadHistory()
      }
    } catch (e) {
      if (!destroyed) {
        console.error('Benchmark failed:', e)
        currentModule = 'Error'
      }
    }

    running = false
    if (timer) clearInterval(timer)
    clearInterval(thermalInt)
    if (unlisten) { unlisten(); unlisten = null }
    clearAllTimers()
  }

  function stopBenchmark() {
    destroyed = true
    running = false
    if (timer) clearInterval(timer)
    if (unlisten) { unlisten(); unlisten = null }
    clearAllTimers()
    currentModule = 'Stopped'
  }

  async function deleteRun(runId: string) {
    try {
      await invoke('delete_result', { runId })
      await loadHistory()
    } catch {}
  }

  // Leaderboard click → detail modal
  async function showRunDetail(run: SavedRun) {
    try {
      const detail = await invoke<RunResult | null>('get_run_detail', { runId: run.run_id })
      detailResult = detail
      showDetailModal = true
    } catch {}
  }

  // Derived: leaderboard from history sorted by score
  let leaderboard = $derived(
    [...history].sort((a, b) => b.overall_raw - a.overall_raw)
  )
</script>

<div class="dashboard">
  <!-- Header -->
  <div class="header">
    <div class="header-left">
      <div class="logo-icon">F</div>
      <span class="logo-text">FairyBench <span class="subtitle">Dashboard</span></span>
      <span class="badge" class:ok={!running} class:live={running} class:err={currentModule === 'Error'}>
        <span class="status-dot"></span>
        {running ? 'Running' : currentModule === 'Error' ? 'Error' : 'Ready'}
      </span>
    </div>
    <div class="header-actions">
      <button class="btn btn-outline" onclick={() => onSwitch()}>
        {@html svgs.lightning} Simple Mode
      </button>
    </div>
  </div>

  <!-- Controls -->
  <div class="controls">
    {#if running}
      <button class="btn btn-danger" onclick={stopBenchmark}>
        {@html svgs.stop} Stop
      </button>
    {:else}
      <button class="btn btn-primary" onclick={runBenchmark}>
        {@html svgs.play} Run All
      </button>
      {#if result && !showGameAnalysis}
        <button class="btn btn-outline" onclick={loadGameAnalysis}>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:16px;height:16px"><path d="M20 12l-8 8-8-8M20 6l-8 8-8-8"/></svg>
          Game Analysis
        </button>
      {/if}
    {/if}
    <div class="select-mode">
      <button class="opt" class:active={precision === 'quick'} onclick={() => precision = 'quick'}>Quick</button>
      <button class="opt" class:active={precision === 'standard'} onclick={() => precision = 'standard'}>Standard</button>
      <button class="opt" class:active={precision === 'high'} onclick={() => precision = 'high'}>Precision</button>
      <button class="opt" class:active={precision === 'max'} onclick={() => precision = 'max'}>Max</button>
      <button class="opt" class:active={precision === 'extreme'} onclick={() => precision = 'extreme'}>Extreme</button>
    </div>
    <div class="live-stat">Current: <strong>{currentModule}</strong></div>
    {#if running}
      <div class="live-stat timer">{formatTime(elapsed)}</div>
    {/if}
  </div>

  <!-- Main Content: Sidebar + Grid -->
  <div class="main-area">
    <!-- Leaderboard Sidebar -->
    <aside class="sidebar">
      <div class="sidebar-header">
        {@html svgs.trophy}
        <span>Leaderboard</span>
        <span class="sidebar-badge">{leaderboard.length}</span>
      </div>
      <div class="sidebar-body">
        {#if leaderboard.length === 0}
          <div class="empty-leaderboard">Run a benchmark to appear here</div>
        {:else}
          {#each leaderboard as run, i (run.run_id)}
            <div class="lb-entry" class:current={run.run_id === result?.run_id} style="animation-delay: {i * 30}ms"
                 onclick={() => showRunDetail(run)} role="button" tabindex="-1"
                 onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); showRunDetail(run) } }}>
              <span class="lb-rank" class:gold={i===0} class:silver={i===1} class:bronze={i===2}>
                {#if i === 0}
                  {@html svgs.trophy}
                {:else}
                  #{i+1}
                {/if}
              </span>
              <div class="lb-info">
                <span class="lb-score">{formatScore(run.overall_raw)}</span>
                <span class="lb-cv">CV: {formatPct(run.cv)}</span>
              </div>
              <span class="lb-date">{new Date(run.executed_at).toLocaleDateString('ja-JP', {month:'short', day:'numeric'})}</span>
            </div>
          {/each}
        {/if}
      </div>
    </aside>

    <!-- Panel Grid -->
    <div class="panel-grid">
      <!-- Metrics Panel -->
      <div class="panel">
        <div class="panel-header">
          <span class="panel-title">{@html svgs.chart} Modules</span>
          <span class="panel-badge">{running ? 'streaming' : result ? 'latest' : 'standby'}</span>
        </div>
        <div class="panel-body">
          {#if completedModules.length > 0}
            <div class="module-list">
              {#each completedModules as mod, i}
                <div class="module-row" style="animation-delay: {i * 50}ms" class:active={mod.name === currentModule && running}>
                  <span class="module-icon">{@html moduleIcons[mod.name] || svgs.bolt}</span>
                  <span class="module-name">{mod.name}</span>
                  <span class="module-score">{formatShort(mod.score)}</span>
                  <span class="module-label">{mod.label}</span>
                  {#if running && mod.name === currentModule}
                    <span class="module-pulse"></span>
                  {/if}
                </div>
              {/each}
            </div>
            {#if running}
              <div class="stream-indicator">Streaming... {@html svgs.bolt}</div>
            {/if}
          {:else if running}
            <div class="loading-state">
              <div class="spinner"></div>
              <span>Waiting for modules...</span>
            </div>
          {:else}
            <div class="placeholder-text">Run benchmark to see module scores stream in real-time</div>
          {/if}
        </div>
      </div>

      <!-- Real-time Module Visualization -->
      {#if running}
        {#if currentModule.startsWith('Render-3DScene')}
          <div class="panel">
            <div class="panel-header">
              <span class="panel-title">{@html svgs.gpu} 3D Scene Render</span>
              <span class="panel-badge live"><span class="status-dot"></span> Live</span>
            </div>
            <div class="panel-body" style="padding:0">
              <Scene3DView {running} />
            </div>
          </div>
        {:else if currentModule.startsWith('Storage-Throughput')}
          <div class="panel">
            <div class="panel-header">
              <span class="panel-title">{@html svgs.storage} Disk Throughput</span>
              <span class="panel-badge live"><span class="status-dot"></span> Live</span>
            </div>
            <div class="panel-body" style="padding:0">
              <DiskView {running} />
            </div>
          </div>
        {/if}
      {/if}

      <!-- Thermal Panel -->
      <div class="panel">
        <div class="panel-header">
          <span class="panel-title">{@html svgs.thermometer} Thermal</span>
          <span class="panel-badge" class:green={thermal?.sensors_available} class:yellow={!thermal?.sensors_available}>
            {thermal?.sensors_available ? 'Live' : 'No sensor'}
          </span>
        </div>
        <div class="panel-body">
          <div class="temp-rows">
            <div class="temp-row">
              <span class="temp-label">{@html svgs.cpu} CPU</span>
              <div class="temp-track">
                <div class="temp-fill" style="width: {thermal?.cpu_temp_avg != null ? ((thermal.cpu_temp_avg - 25) / 75 * 100).toFixed(0) + '%' : '3%'}"></div>
              </div>
              <span class="temp-val">{formatTemp(thermal?.cpu_temp_avg ?? null)}</span>
            </div>
            <div class="temp-row">
              <span class="temp-label">{@html svgs.gpu} GPU</span>
              <div class="temp-track">
                <div class="temp-fill" style="width: {thermal?.gpu_temp != null ? ((thermal.gpu_temp - 25) / 75 * 100).toFixed(0) + '%' : '3%'}"></div>
              </div>
              <span class="temp-val">{formatTemp(thermal?.gpu_temp ?? null)}</span>
            </div>
            <div class="temp-divider"></div>
            <div class="temp-row">
              <span class="temp-label">{@html svgs.cpu} Clock</span>
              <span class="temp-val mono">{formatClock(thermal?.cpu_clock_ghz ?? null, 'GHz')}</span>
            </div>
            <div class="temp-row">
              <span class="temp-label">{@html svgs.gpu} Clock</span>
              <span class="temp-val mono">{formatClock(thermal?.gpu_clock_mhz ?? null, 'MHz')}</span>
            </div>
            <div class="temp-row">
              <span class="temp-label">{@html svgs.lightning} Power</span>
              <span class="temp-val mono">{formatWatts(thermal?.power_watts ?? null)}</span>
            </div>
          </div>
          {#if thermal?.cpu_load_pct != null}
            <div class="temp-row">
              <span class="temp-label">{@html svgs.cpu} Load</span>
              <div class="temp-track">
                <div class="temp-fill" style="width: {thermal.cpu_load_pct}%"></div>
              </div>
              <span class="temp-val">{thermal.cpu_load_pct.toFixed(0)}%</span>
            </div>
          {/if}
          {#if thermal?.gpu_load_pct != null}
            <div class="temp-row">
              <span class="temp-label">{@html svgs.gpu} Load</span>
              <div class="temp-track">
                <div class="temp-fill" style="width: {thermal.gpu_load_pct}%"></div>
              </div>
              <span class="temp-val">{thermal.gpu_load_pct.toFixed(0)}%</span>
            </div>
          {/if}
          {#if thermal?.gpu_mem_used_mb != null && thermal?.gpu_mem_total_mb != null}
            <div class="temp-row">
              <span class="temp-label">{@html svgs.memory} VRAM</span>
              <div class="temp-track">
                <div class="temp-fill" style="width: {(thermal.gpu_mem_used_mb / thermal.gpu_mem_total_mb * 100).toFixed(0)}%"></div>
              </div>
              <span class="temp-val">{(thermal.gpu_mem_used_mb / 1024).toFixed(0)}GB</span>
            </div>
          {/if}
          {#if thermal?.fan_speed_pct != null}
            <div class="temp-row">
              <span class="temp-label">{@html svgs.cpu} Fan</span>
              <div class="temp-track">
                <div class="temp-fill" style="width: {thermal.fan_speed_pct}%"></div>
              </div>
              <span class="temp-val">{thermal.fan_speed_pct.toFixed(0)}%</span>
            </div>
          {/if}
          {#if thermal?.sys_mem_used_gb != null && thermal?.sys_mem_total_gb != null}
            <div class="temp-row">
              <span class="temp-label">{@html svgs.memory} RAM</span>
              <div class="temp-track">
                <div class="temp-fill" style="width: {(thermal.sys_mem_used_gb / thermal.sys_mem_total_gb * 100).toFixed(0)}%"></div>
              </div>
              <span class="temp-val">{thermal.sys_mem_used_gb.toFixed(1)}GB</span>
            </div>
          {/if}
        </div>
      </div>

      <!-- Score Panel -->
      <div class="panel">
        <div class="panel-header">
          <span class="panel-title">{@html svgs.bolt} Score</span>
          <span class="panel-badge">FairyScore</span>
        </div>
        <div class="panel-body score-panel-body">
          {#if result || running}
            <div class="score-big gradient-text">{formatScore(currentRunScore)}</div>
            {#if lastAddedScore > 0}
              <div class="score-added">+{formatShort(lastAddedScore)} <span class="score-added-name">{lastAddedName}</span></div>
            {/if}
            {#if result}
              <div class="score-ci">95% CI: {formatScore(result.ci_lower)} – {formatScore(result.ci_upper)}</div>
              <div class="score-meta">
                <span>CV: {formatPct(result.cv)}</span>
                <span>{result.runs_used} runs</span>
                {#if result.runs_excluded > 0}
                  <span class="excluded">({result.runs_excluded} excluded)</span>
                {/if}
              </div>
            {/if}
          {:else}
            <div class="placeholder-text">Run benchmark to see your FairyScore</div>
          {/if}
        </div>
      </div>

      <!-- Stats Panel -->
      <StatsPanel {result} {history} {deleteRun} {formatScore} />
    </div>
  </div>

  <!-- Bottom Bar -->
  <div class="bottom-bar">
    <div class="bottom-row">
      <div class="bottom-left">
        {#if deviceInfo}
          <span class="chip" title="GPU">{@html svgs.gpu} {deviceInfo.adapter_name || '--'}</span>
          <span class="chip" title="Backend">{@html svgs.cpu} {deviceInfo.backend}</span>
          <span class="chip" title="OS">{deviceInfo.os_info}</span>
          {#if deviceInfo.cpu_name}
            <span class="chip" title="CPU">{deviceInfo.cpu_name.slice(0, 40)}</span>
          {/if}
          <span class="chip">{deviceInfo.cpu_cores}C / {deviceInfo.total_memory_gb.toFixed(0)}GB</span>
        {/if}
        <span class="chip">{history.length} results</span>
        <span class="chip">{version}</span>
      </div>
      <div class="bottom-right">
        <button class="btn btn-outline btn-sm" onclick={() => loadHistory()}>
          {@html svgs.refresh}
        </button>
      </div>
    </div>
  </div>
</div>

<!-- Detail Modal -->
{#if showDetailModal && detailResult}
  <div class="modal-overlay" onclick={() => showDetailModal = false}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px">
        <h3 style="font-size:14px;font-weight:600">詳細結果</h3>
        <button class="btn btn-outline btn-sm" onclick={() => showDetailModal = false}>{@html svgs.info} Close</button>
      </div>
      <div class="score-big gradient-text" style="font-size:28px;text-align:center;margin-bottom:12px">
        {formatScore(detailResult.overall_raw)}
      </div>
      <div style="font-size:11px;color:var(--text-muted);text-align:center;margin-bottom:16px">
        95% CI: {formatScore(detailResult.ci_lower)} – {formatScore(detailResult.ci_upper)}
        &nbsp; CV: {formatPct(detailResult.cv)}
      </div>
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:4px">
        {#each Object.entries(detailResult.sub_scores) as [name, sub]}
          <div style="display:flex;justify-content:space-between;padding:4px 8px;background:var(--bg-primary);border-radius:4px;font-size:11px">
            <span style="color:var(--text-muted)">{name}</span>
            <span style="color:var(--text-primary);font-weight:500">{formatScore(sub.raw_score)}</span>
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<!-- Game Analysis Modal (popup) -->
{#if showGameAnalysis && gameAnalysis}
  <div class="modal-overlay" onclick={() => { showGameAnalysis = false; gameAnalysis = null }} role="dialog" aria-modal="true">
    <div class="modal-content game-modal" onclick={(e) => e.stopPropagation()}>
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px">
        <h3 style="font-size:14px;font-weight:600;display:flex;align-items:center;gap:6px">
          {@html svgs.chart} Game Analysis
        </h3>
        <button class="btn btn-outline btn-sm" onclick={() => { showGameAnalysis = false; gameAnalysis = null }}>Close</button>
      </div>
      <div style="display:flex;flex-direction:column;gap:4px;max-height:60vh;overflow-y:auto;padding-right:4px">
        {#each gameAnalysis as g}
          <div class="game-entry" class:status-pass={g.status_jp === '快適'} class:status-ok={g.status_jp === '可'} class:status-limit={g.status_jp === '限界'} class:status-no={g.status_jp === '不可'}>
            <div class="game-title">{g.title}</div>
            <div class="game-status" class:game-pass={g.status_jp === '快適'} class:game-ok={g.status_jp === '可'} class:game-limit={g.status_jp === '限界'} class:game-no={g.status_jp === '不可'}>{g.status_jp}</div>
            <div class="game-fps">{g.fps_est}</div>
            <div class="game-notes">{g.notes}</div>
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .dashboard {
    display: flex; flex-direction: column; height: 100vh;
    padding: 16px 20px 16px;
  }
  .header { display: flex; align-items: center; justify-content: space-between; flex-shrink: 0; margin-bottom: 12px; }
  .header-left { display: flex; align-items: center; gap: 10px; }
  .logo-icon {
    width: 28px; height: 28px;
    background: linear-gradient(135deg, var(--accent) 0%, #6366F1 100%);
    border-radius: 6px; display: flex; align-items: center; justify-content: center;
    font-size: 13px; font-weight: 700; color: var(--bg-primary);
  }
  .logo-text { font-size: 16px; font-weight: 600; }
  .subtitle { font-size: 11px; color: var(--text-muted); font-weight: 400; margin-left: 4px; }
  .badge {
    display: inline-flex; align-items: center; gap: 4px;
    padding: 2px 8px; border-radius: 3px; font-size: 10px; font-weight: 500;
    background: rgba(251,191,36,0.12); border: 1px solid rgba(251,191,36,0.25); color: var(--yellow);
  }
  .badge.ok { background: rgba(52,211,153,0.12); border-color: rgba(52,211,153,0.25); color: var(--green); }
  .badge.err { background: rgba(248,113,113,0.12); border-color: rgba(248,113,113,0.25); color: var(--red); }
  .status-dot { width: 6px; height: 6px; border-radius: 50%; background: currentColor; }
  .badge.live .status-dot { animation: pulse-dot 1.2s ease-in-out infinite; }
  @keyframes pulse-dot { 0%,100%{opacity:1} 50%{opacity:0.3} }

  .controls { display: flex; gap: 8px; align-items: center; flex-shrink: 0; margin-bottom: 12px; flex-wrap: wrap; }
  .select-mode { display: flex; background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius); padding: 2px; }
  .opt {
    padding: 4px 10px; border-radius: 5px; font-size: 11px; color: var(--text-muted);
    cursor: pointer; transition: all 0.15s; border: none; background: none; font-family: inherit;
  }
  .opt.active { background: var(--accent); color: var(--bg-primary); font-weight: 600; }
  .opt:not(.active):hover { color: var(--text-primary); }
  .live-stat { font-size: 11px; color: var(--text-muted); }
  .live-stat strong { color: var(--text-secondary); font-weight: 500; }
  .timer { font-variant-numeric: tabular-nums; }

  /* Main area: sidebar + grid */
  .main-area { display: flex; gap: 12px; flex: 1; min-height: 0; }

  /* Sidebar / Leaderboard */
  .sidebar {
    width: 240px; flex-shrink: 0;
    background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius-lg);
    display: flex; flex-direction: column; overflow: hidden;
  }
  .sidebar-header {
    display: flex; align-items: center; gap: 6px;
    padding: 12px 14px; border-bottom: 1px solid var(--bg-tertiary);
    font-size: 12px; font-weight: 600; color: var(--text-secondary);
  }
  .sidebar-header :global(svg) { width: 14px; height: 14px; }
  .sidebar-badge {
    margin-left: auto; font-size: 10px; padding: 1px 6px; border-radius: 3px;
    background: var(--bg-tertiary); color: var(--text-muted);
  }
  .sidebar-body { flex: 1; overflow-y: auto; padding: 6px; }
  .empty-leaderboard { font-size: 11px; color: var(--text-muted); text-align: center; padding: 24px 8px; }

  .lb-entry {
    display: flex; align-items: center; gap: 6px;
    padding: 6px 8px; border-radius: 5px; margin-bottom: 2px;
    background: var(--bg-primary); border: 1px solid var(--border);
    animation: fadeInItem 0.3s ease both;
    transition: all 0.2s;
  }
  .lb-entry.current { border-color: var(--accent); background: rgba(129,140,248,0.06); }
  .lb-entry:hover { border-color: var(--accent); }
  .lb-rank { font-size: 11px; font-weight: 600; width: 22px; text-align: center; color: var(--text-muted); flex-shrink: 0; }
  .lb-rank :global(svg) { width: 14px; height: 14px; vertical-align: middle; }
  .lb-rank.gold { color: var(--yellow); }
  .lb-rank.silver { color: #94A3B8; }
  .lb-rank.bronze { color: #D97706; }
  .lb-info { flex: 1; min-width: 0; }
  .lb-score { display: block; font-size: 12px; font-weight: 600; color: var(--text-primary); }
  .lb-cv { font-size: 9px; color: var(--text-muted); }
  .lb-date { font-size: 9px; color: var(--text-muted); flex-shrink: 0; }

  @keyframes fadeInItem { from { opacity: 0; transform: translateX(-6px); } to { opacity: 1; transform: translateX(0); } }

  /* Panel Grid */
  .panel-grid {
    display: grid; grid-template-columns: 1fr 1fr; gap: 10px;
    flex: 1; min-height: 0; overflow-y: auto; padding-bottom: 4px;
  }
  .panel {
    background: var(--bg-secondary); border: 1px solid var(--border);
    border-radius: var(--radius-lg); overflow: hidden;
    display: flex; flex-direction: column;
  }
  .panel-header {
    display: flex; align-items: center; gap: 6px;
    padding: 10px 14px 8px; border-bottom: 1px solid var(--bg-tertiary);
  }
  .panel-title {
    font-size: 12px; font-weight: 600; color: var(--text-secondary);
    display: flex; align-items: center; gap: 5px;
  }
  .panel-title :global(svg) { width: 13px; height: 13px; }
  .panel-badge {
    margin-left: auto;
    font-size: 9px; padding: 1px 6px; border-radius: 3px;
    color: var(--text-muted); background: var(--bg-tertiary);
    transition: all 0.3s;
  }
  .panel-badge.green { color: var(--green); background: rgba(52,211,153,0.1); }
  .panel-badge.yellow { color: var(--yellow); background: rgba(251,191,36,0.1); }
  .panel-body { padding: 10px 14px 14px; flex: 1; }

  /* Game Analysis */
  .game-entry {
    display: grid; grid-template-columns: 1fr auto auto 1fr; gap: 8px;
    align-items: center; padding: 8px 10px; border-radius: 5px;
    background: var(--bg-primary); border: 1px solid var(--border); font-size: 11px;
    line-height: 1.5;
  }
  .game-entry.status-pass { border-left: 3px solid var(--green); }
  .game-entry.status-ok { border-left: 3px solid var(--accent); }
  .game-entry.status-limit { border-left: 3px solid var(--yellow); }
  .game-entry.status-no { border-left: 3px solid var(--red); }
  .game-title { font-weight: 600; color: var(--text-primary); word-break: break-word; min-width: 0; }
  .game-status { font-size: 10px; font-weight: 600; padding: 2px 8px; border-radius: 4px; text-align: center; white-space: nowrap; }
  .game-pass { color: var(--green); background: rgba(34,197,94,0.1); }
  .game-ok { color: var(--accent); background: rgba(129,140,248,0.1); }
  .game-limit { color: var(--yellow); background: rgba(245,158,11,0.1); }
  .game-no { color: var(--red); background: rgba(239,68,68,0.1); }
  .game-fps { font-size: 10px; color: var(--text-muted); text-align: right; white-space: nowrap; min-width: fit-content; }
  .game-notes { font-size: 10px; color: var(--text-muted); word-break: break-word; min-width: 0; }

  /* Modal overlay — shared with detail modal */
  .modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.55); display: flex; align-items: center; justify-content: center; z-index: 1000; }
  .modal-content { background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius); padding: 20px; min-width: 280px; max-width: 90vw; box-shadow: 0 8px 40px rgba(0,0,0,0.5); }
  .game-modal { width: min(560px, 92vw); max-height: 80vh; }

  /* Module list */
  .module-list { display: flex; flex-direction: column; gap: 3px; }
  .module-row {
    display: flex; align-items: center; gap: 6px;
    padding: 5px 8px; border-radius: 5px;
    background: var(--bg-primary); border: 1px solid var(--border);
    animation: fadeInItem 0.35s ease both;
    transition: all 0.2s;
    position: relative; overflow: hidden;
  }
  .module-row.active { border-color: var(--accent); }
  .module-icon { display: flex; align-items: center; }
  .module-icon :global(svg) { width: 14px; height: 14px; color: var(--text-muted); }
  .module-name { font-size: 11px; color: var(--text-secondary); flex: 1; }
  .module-score { font-size: 12px; font-weight: 600; color: var(--accent); font-variant-numeric: tabular-nums; }
  .module-label { font-size: 9px; color: var(--text-muted); }
  .module-pulse {
    position: absolute; inset: 0;
    background: linear-gradient(90deg, transparent, rgba(129,140,248,0.08), transparent);
    animation: shimmer 1.5s ease-in-out infinite;
  }
  @keyframes shimmer { 0%{transform:translateX(-100%)} 100%{transform:translateX(100%)} }

  .stream-indicator {
    display: flex; align-items: center; gap: 4px;
    font-size: 10px; color: var(--accent); margin-top: 6px;
    animation: pulse-text 1.2s ease-in-out infinite;
  }
  .stream-indicator :global(svg) { width: 12px; height: 12px; }
  @keyframes pulse-text { 0%,100%{opacity:0.6} 50%{opacity:1} }

  .loading-state {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    gap: 8px; padding: 20px 0; color: var(--text-muted); font-size: 11px;
  }
  .spinner {
    width: 24px; height: 24px; border-radius: 50%;
    border: 2px solid var(--bg-tertiary);
    border-top-color: var(--accent);
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  .placeholder-text { font-size: 11px; color: var(--text-muted); text-align: center; padding: 16px 8px; line-height: 1.5; }

  /* Thermal */
  .temp-rows { display: flex; flex-direction: column; gap: 6px; }
  .temp-row { display: flex; align-items: center; gap: 8px; }
  .temp-label {
    display: flex; align-items: center; gap: 4px;
    width: 60px; font-size: 10px; color: var(--text-secondary); flex-shrink: 0;
  }
  .temp-label :global(svg) { width: 11px; height: 11px; }
  .temp-track { flex: 1; height: 5px; background: var(--bg-tertiary); border-radius: 3px; overflow: hidden; }
  .temp-fill {
    height: 100%; border-radius: 3px;
    background: linear-gradient(90deg, var(--green), var(--yellow), var(--red));
    transition: width 0.5s ease;
  }
  .temp-val { width: 52px; font-size: 10px; font-weight: 500; text-align: right; flex-shrink: 0; }
  .mono { font-variant-numeric: tabular-nums; }
  .temp-divider { height: 1px; background: var(--bg-tertiary); margin: 2px 0; }

  /* Score */
  .score-panel-body { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 16px; }
  .score-big {
    font-size: 36px; font-weight: 200; color: var(--accent);
    font-variant-numeric: tabular-nums;
    transition: all 0.3s;
  }
  .score-added {
    display: flex; align-items: center; gap: 6px;
    margin-top: 4px; font-size: 13px; font-weight: 500;
    color: var(--green);
    animation: fadeInAdded 0.4s ease both;
  }
  .score-added-name {
    font-size: 10px; color: var(--text-muted); font-weight: 400;
  }
  @keyframes fadeInAdded {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
  }
  .score-ci { font-size: 10px; color: var(--text-muted); margin-top: 4px; }
  .score-meta { display: flex; gap: 12px; margin-top: 6px; font-size: 10px; color: var(--text-muted); }
  .excluded { color: var(--yellow); }

  /* Bottom Bar */
  .bottom-bar { border-top: 1px solid var(--bg-tertiary); padding-top: 10px; flex-shrink: 0; }
  .bottom-row { display: flex; align-items: center; justify-content: space-between; gap: 8px; flex-wrap: wrap; }
  .bottom-left, .bottom-right { display: flex; gap: 6px; align-items: center; flex-wrap: wrap; }
  .chip {
    display: inline-flex; align-items: center; gap: 4px;
    padding: 3px 8px; background: var(--bg-secondary); border: 1px solid var(--border);
    border-radius: 4px; font-size: 10px; color: var(--text-muted); white-space: nowrap;
  }
  .chip :global(svg) { width: 11px; height: 11px; }
  .btn-sm { padding: 4px 10px; font-size: 11px; }

  @media (max-width: 900px) {
    .sidebar { width: 180px; }
  }
  @media (max-width: 720px) {
    .sidebar { display: none; }
    .panel-grid { grid-template-columns: 1fr; }
  }
</style>
