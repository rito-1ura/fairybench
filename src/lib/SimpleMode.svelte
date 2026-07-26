<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'

  interface SubScore { module_name: string; raw_score: number; normalized_score: number }
  interface RunResult {
    run_id: string; overall_raw: number; ci_lower: number; ci_upper: number;
    cv: number; runs_used: number; runs_excluded: number;
    sub_scores: Record<string, SubScore>
  }
  interface ThermalSample {
    timestamp_ms: number; cpu_temp_avg: number | null; gpu_temp: number | null;
    cpu_clock_ghz: number | null; gpu_clock_mhz: number | null; power_watts: number | null;
    cpu_load_pct: number | null; gpu_load_pct: number | null;
    gpu_mem_used_mb: number | null; gpu_mem_total_mb: number | null;
    sensors_available: boolean
  }

  let { onSwitch, version }: { onSwitch: () => void; version: string } = $props()

  let running = $state(false)
  let result = $state<RunResult | null>(null)
  let currentModule = $state('')
  let progress = $state(0)
  let thermal = $state<ThermalSample | null>(null)
  let thermalInt: ReturnType<typeof setInterval> | null = null

  const modules = [
    'Render-Raster', 'Render-PathTrace', 'Render-Procedural',
    'AI-Inference', 'AI-Generative', 'Storage-Throughput',
    'Memory-Bandwidth'
  ]

  const svgs = {
    dashboard: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/></svg>',
    play: '<svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>',
    clock: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>',
    gpu: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2" y="4" width="20" height="14" rx="2"/><circle cx="12" cy="11" r="4"/><path d="M16 18v2M8 18v2"/></svg>',
    cpu: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M9 9h6v6H9z"/><path d="M9 2v2M15 2v2M9 20v2M15 20v2M2 9h2M2 15h2M20 9h2M20 15h2"/></svg>',
    storage: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M21 12c0 1.66-4.03 3-9 3s-9-1.34-9-3"/><path d="M3 5v14c0 1.66 4.03 3 9 3s9-1.34 9-3V5"/></svg>',
    memory: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="4" y="4" width="16" height="16" rx="2"/><rect x="7" y="7" width="3" height="10" rx="1"/><rect x="14" y="7" width="3" height="10" rx="1"/></svg>',
    ai: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 2a7 7 0 0 1 7 7c0 2.4-1.2 4.5-3 5.7V17H8v-2.3C6.2 13.5 5 11.4 5 9a7 7 0 0 1 7-7z"/><path d="M9 17h6"/><path d="M10 20h4"/></svg>',
    lightning: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>',
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

  const moduleColor = (name: string): string => {
    if (name.startsWith('Render')) return 'var(--accent)'
    if (name.startsWith('AI')) return 'var(--yellow)'
    if (name.startsWith('Storage') || name.startsWith('Memory')) return 'var(--red)'
    return 'var(--green)'
  }

  const formatScore = (v: number): string => v.toLocaleString('en-US', { maximumFractionDigits: 0 })

  async function runBenchmark() {
    running = true
    result = null
    progress = 0

    // Start thermal polling
    thermalInt = setInterval(async () => {
      try { thermal = await invoke<ThermalSample>('get_thermal_snapshot') } catch {}
    }, 3000)

    try {
      const res = await invoke<RunResult>('run_benchmark')
      result = res
      progress = 100
    } catch (e) {
      console.error('Benchmark failed:', e)
    }
    running = false
    currentModule = ''
    if (thermalInt) { clearInterval(thermalInt); thermalInt = null }
  }
</script>

<div class="app simple">
  <div class="header">
    <div class="logo">
      <div class="logo-icon">F</div>
      <span class="logo-text">FairyBench</span>
      <span class="version">{version}</span>
    </div>
    <div class="header-actions">
      <button class="btn btn-outline" onclick={() => onSwitch()}>
        {@html svgs.dashboard} Dashboard
      </button>
    </div>
  </div>

  {#if running}
    <div class="running">
      <div class="spinner"></div>
      <div class="running-module">{currentModule}</div>
      <div class="progress-track"><div class="progress-fill" style="width: {progress}%"></div></div>
    </div>
  {/if}

  <div class="score-hero">
    <div class="score-label">FairyScore</div>
    <div class="score-value gradient-text">
      {result ? formatScore(result.overall_raw) : '—'}
    </div>
    {#if result}
      <div class="ci-bar-container">
        <div class="ci-track">
          <div class="ci-fill"></div>
          <div class="ci-marker"></div>
        </div>
      </div>
      <div class="ci-text">
        95% CI: <strong>{formatScore(result.ci_lower)}</strong> – <strong>{formatScore(result.ci_upper)}</strong>
        &nbsp; CV: {(result.cv * 100).toFixed(1)}%
        &nbsp; {result.runs_used} runs
        {#if result.runs_excluded > 0}({result.runs_excluded} excluded){/if}
      </div>
    {:else if !running}
      <div class="ci-text idle-text">Press "Run" to start the benchmark</div>
    {/if}
  </div>

  <div class="sub-scores">
    {#each modules as mod}
      {@const sub = result?.sub_scores?.[mod]}
      <div class="sub-card" style="--card-accent: {moduleColor(mod)}">
        <div class="sub-icon">{@html moduleIcons[mod] || svgs.cpu}</div>
        <div class="sub-name">{mod}</div>
        <div class="sub-value">{sub ? formatScore(sub.raw_score) : '—'}</div>
      </div>
    {/each}
  </div>

  {#if running || thermal?.sensors_available}
    <div class="thermal-strip">
      {#if running}
        <span class="thermal-chip">{@html moduleIcons['Render-Raster'] || svgs.gpu} Running {currentModule}</span>
      {/if}
      {#if thermal?.cpu_temp_avg != null}
        <span class="thermal-chip">{@html svgs.cpu} CPU {thermal.cpu_temp_avg.toFixed(0)}°C / {thermal.cpu_load_pct?.toFixed(0) ?? '?'}%</span>
      {/if}
      {#if thermal?.gpu_temp != null}
        <span class="thermal-chip">{@html moduleIcons['Render-Raster'] || svgs.gpu} GPU {thermal.gpu_temp.toFixed(0)}°C / {thermal.gpu_load_pct?.toFixed(0) ?? '?'}%</span>
      {/if}
      {#if thermal?.power_watts != null}
        <span class="thermal-chip">{@html svgs.lightning} {thermal.power_watts.toFixed(0)}W</span>
      {/if}
    </div>
  {/if}

  <div class="actions">
    <button class="btn btn-primary btn-run" onclick={runBenchmark} disabled={running}>
      {#if running}
        {@html svgs.clock} Running...
      {:else}
        {@html svgs.play} Run All
      {/if}
    </button>
    <button class="btn btn-outline" onclick={() => onSwitch()}>
      {@html svgs.dashboard} Dashboard
    </button>
  </div>
</div>

<style>
  .app.simple {
    max-width: 720px; margin: 0 auto; padding: 40px 32px 32px;
    display: flex; flex-direction: column; height: 100vh;
  }

  .header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 48px; }
  .logo { display: flex; align-items: center; gap: 10px; }
  .logo-icon {
    width: 34px; height: 34px;
    background: linear-gradient(135deg, var(--accent) 0%, #6366F1 100%);
    border-radius: 8px; display: flex; align-items: center; justify-content: center;
    font-size: 17px; font-weight: 700; color: var(--bg-primary);
  }
  .logo-text { font-size: 19px; font-weight: 600; letter-spacing: -0.3px; }
  .version { font-size: 11px; color: var(--text-muted); }
  .header-actions :global(.btn) :global(svg) { width: 14px; height: 14px; }

  .running { text-align: center; margin-bottom: 24px; }
  .spinner {
    width: 40px; height: 40px; border: 3px solid var(--bg-tertiary);
    border-top-color: var(--accent); border-radius: 50%;
    animation: spin 1s linear infinite; margin: 0 auto 12px;
  }
  @keyframes spin { to { transform: rotate(360deg); } }
  .running-module {
    display: inline-block; padding: 4px 12px;
    background: rgba(129,140,248,0.1); border: 1px solid rgba(129,140,248,0.2);
    border-radius: 4px; font-size: 13px; color: var(--accent); margin-bottom: 10px;
  }
  .progress-track { width: 200px; height: 3px; background: var(--bg-tertiary); border-radius: 2px; margin: 0 auto; overflow: hidden; }
  .progress-fill { height: 100%; background: var(--accent); border-radius: 2px; transition: width 0.3s; }

  .score-hero { text-align: center; margin-bottom: 40px; flex-shrink: 0; }
  .score-label {
    font-size: 13px; font-weight: 500; color: var(--text-muted);
    text-transform: uppercase; letter-spacing: 2px; margin-bottom: 8px;
  }
  .score-value {
    font-size: 72px; font-weight: 300; line-height: 1; letter-spacing: -3px;
    color: #F0F0F8; margin-bottom: 14px;
  }
  .ci-bar-container { display: flex; align-items: center; justify-content: center; gap: 14px; margin-bottom: 6px; }
  .ci-track { width: 180px; height: 4px; background: var(--bg-tertiary); border-radius: 2px; position: relative; }
  .ci-fill { position: absolute; left: 15%; width: 70%; height: 100%; background: linear-gradient(90deg, var(--accent) 0%, #A78BFA 100%); border-radius: 2px; }
  .ci-marker { position: absolute; left: 50%; top: -4px; width: 10px; height: 10px; background: var(--accent); border-radius: 50%; transform: translateX(-50%); box-shadow: 0 0 10px rgba(129,140,248,0.5); }
  .ci-text { font-size: 12px; color: var(--text-secondary); }
  .ci-text strong { color: var(--text-primary); font-weight: 500; }
  .idle-text { margin-top: 8px; font-size: 13px; color: var(--text-muted); }

  .sub-scores { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; margin-bottom: auto; padding: 12px 0; }
  .sub-card {
    background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius-lg);
    padding: 18px 12px; text-align: center; transition: border-color 0.2s;
  }
  .sub-card:hover { border-color: var(--card-accent); }
  .sub-icon :global(svg) { width: 22px; height: 22px; margin-bottom: 8px; color: var(--card-accent); }
  .sub-name { font-size: 11px; font-weight: 500; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.8px; margin-bottom: 4px; }
  .sub-value { font-size: 22px; font-weight: 600; color: var(--text-primary); letter-spacing: -0.5px; }

  .thermal-strip {
    display: flex; gap: 6px; justify-content: center; flex-wrap: wrap;
    padding: 8px 0; margin: 4px 0;
  }
  .thermal-chip {
    display: inline-flex; align-items: center; gap: 4px;
    padding: 4px 10px; background: var(--bg-secondary); border: 1px solid var(--border);
    border-radius: 4px; font-size: 11px; color: var(--text-secondary);
  }
  .thermal-chip :global(svg) { width: 12px; height: 12px; }

  .actions { display: flex; gap: 10px; justify-content: center; padding: 16px 0; flex-shrink: 0; }
  .actions :global(.btn) :global(svg) { width: 15px; height: 15px; }
  .btn-run { padding: 12px 44px; font-size: 15px; }
  .btn-run:disabled { opacity: 0.5; cursor: not-allowed; }

  @media (max-width: 640px) {
    .app.simple { padding: 24px 16px; }
    .score-value { font-size: 52px; }
    .sub-scores { grid-template-columns: repeat(2, 1fr); }
  }
</style>
