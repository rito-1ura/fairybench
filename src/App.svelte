<script lang="ts">
  import './app.css'
  import SimpleMode from './lib/SimpleMode.svelte'
  import DashboardMode from './lib/DashboardMode.svelte'

  let mode = $state<'simple' | 'dashboard'>(
    (typeof localStorage !== 'undefined' ? localStorage.getItem('mode') as 'simple' | 'dashboard' : null) || 'simple'
  )
  let version = $state('')
  let isTauri = $state(false)
  let showLanding = $state(true)

  $effect(() => {
    // Preserve mode on switch
    localStorage.setItem('mode', mode)
  })

  $effect(() => {
    // Detect Tauri context
    if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
      isTauri = true
      showLanding = false
      import('@tauri-apps/api/core').then(({ invoke }) => {
        invoke<string>('get_version').then(v => version = v).catch(() => {})
      })
    }
  })
</script>

{#if !isTauri && showLanding}
  <!-- Landing page for web/Vercel -->
  <div class="landing">
    <div class="landing-content">
      <div class="landing-logo">
        <svg viewBox="0 0 48 48" fill="none" style="width:64px;height:64px">
          <rect x="4" y="4" width="40" height="40" rx="10" fill="var(--accent)" fill-opacity=".15"/>
          <text x="24" y="33" text-anchor="middle" fill="var(--accent)" font-size="28" font-weight="800" font-family="Inter,sans-serif">F</text>
        </svg>
      </div>
      <h1>FairyBench</h1>
      <p class="landing-subtitle">Ultimate PC Benchmark Suite — 3D Graphics, Storage, AI, CPU & more</p>
      <div class="landing-features">
        <div class="lf-card"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:20px;height:20px"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M8 21h8M12 17v4"/></svg><span>3D Graphics (wgpu)</span></div>
        <div class="lf-card"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:20px;height:20px"><ellipse cx="12" cy="12" rx="10" ry="4"/><path d="M4 12v5c0 2.2 3.6 4 8 4s8-1.8 8-4v-5"/></svg><span>Storage Throughput</span></div>
        <div class="lf-card"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:20px;height:20px"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M9 9h6v6H9z"/></svg><span>AI Inference</span></div>
        <div class="lf-card"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:20px;height:20px"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg><span>Memory Bandwidth</span></div>
        <div class="lf-card"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:20px;height:20px"><path d="M4 7V4h16v3"/><path d="M9 20h6"/><path d="M12 4v16"/></svg><span>CPU Benchmark</span></div>
        <div class="lf-card"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:20px;height:20px"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="3"/></svg><span>Real-time Thermal Monitor</span></div>
      </div>
      <div class="landing-actions">
        <a class="btn btn-primary" href="https://github.com/rito-1ura/fairybench" target="_blank">View on GitHub</a>
        <button class="btn btn-outline" onclick={() => { showLanding = false; try { import('@tauri-apps/api/core') } catch {} }}>Enter App</button>
      </div>
      <p class="landing-footer">Built with Rust + Tauri 2 + Svelte 5 + wgpu</p>
    </div>
  </div>
{:else}
  <div id="app">
    {#if mode === 'simple'}
      <SimpleMode onSwitch={() => mode = 'dashboard'} {version} />
    {:else}
      <DashboardMode onSwitch={() => mode = 'simple'} {version} />
    {/if}
  </div>
{/if}

<style>
  .landing {
    min-height: 100vh; display: flex; align-items: center; justify-content: center;
    background: var(--bg-primary); padding: 24px;
  }
  .landing-content { max-width: 500px; text-align: center; }
  .landing-logo { margin-bottom: 16px; }
  .landing h1 { font-size: 36px; font-weight: 800; margin: 0 0 8px; background: linear-gradient(135deg,var(--accent),#6366F1); -webkit-background-clip: text; -webkit-text-fill-color: transparent; }
  .landing-subtitle { font-size: 14px; color: var(--text-secondary); margin: 0 0 32px; }
  .landing-features { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 32px; text-align: left; }
  .lf-card { display: flex; align-items: center; gap: 8px; padding: 10px 12px; background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius); font-size: 12px; color: var(--text-primary); }
  .lf-card svg { flex-shrink: 0; color: var(--accent); }
  .landing-actions { display: flex; gap: 12px; justify-content: center; }
  .landing-actions .btn { padding: 10px 24px; }
  .landing-footer { font-size: 11px; color: var(--text-muted); margin-top: 32px; }
</style>
