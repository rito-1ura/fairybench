<script lang="ts">
  import './app.css'
  import SimpleMode from './lib/SimpleMode.svelte'
  import DashboardMode from './lib/DashboardMode.svelte'

  let mode = $state<'simple' | 'dashboard'>(
    (typeof localStorage !== 'undefined' ? localStorage.getItem('mode') as 'simple' | 'dashboard' : null) || 'simple'
  )
  let version = $state('')
  let isTauri = $state(false)

  $effect(() => {
    localStorage.setItem('mode', mode)
  })

  $effect(() => {
    if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
      isTauri = true
      import('@tauri-apps/api/core').then(({ invoke }) => {
        invoke<string>('get_version').then(v => version = v).catch(() => {})
      })
    }
  })

  // Landing page state
  let scrolled = $state(0)
  $effect(() => {
    if (isTauri) return
    // Allow page scrolling for landing page
    document.documentElement.style.overflow = 'auto'
    document.documentElement.style.height = 'auto'
    document.body.style.overflow = 'auto'
    document.body.style.height = 'auto'
    const appEl = document.getElementById('app')
    if (appEl) { appEl.style.height = 'auto'; appEl.style.display = 'block' }
    const onScroll = () => scrolled = window.scrollY
    window.addEventListener('scroll', onScroll, { passive: true })
    return () => window.removeEventListener('scroll', onScroll)
  })
</script>

{#if !isTauri}
<div class="landing-wrap">
  <!-- Hero -->
  <section class="hero">
    <div class="hero-bg">
      <div class="hero-glow"></div>
      <div class="hero-grid"></div>
    </div>
    <nav class="nav">
      <div class="nav-inner">
        <div class="nav-logo">
          <svg viewBox="0 0 32 32" fill="none" style="width:24px;height:24px">
            <rect x="2" y="2" width="28" height="28" rx="7" fill="var(--accent)" fill-opacity=".2"/>
            <text x="16" y="23" text-anchor="middle" fill="var(--accent)" font-size="20" font-weight="800" font-family="Inter,sans-serif">F</text>
          </svg>
          <span>FairyBench</span>
        </div>
        <div class="nav-links">
          <a href="#features">Features</a>
          <a href="https://github.com/rito-1ura/fairybench" target="_blank">GitHub</a>
          <a class="nav-dl" href="https://github.com/rito-1ura/fairybench/releases" target="_blank">Download</a>
        </div>
      </div>
    </nav>
    <div class="hero-body">
      <div class="hero-badge">v0.1.0 — Tauri Desktop</div>
      <h1 class="hero-title">
        <span class="ht-line">Measure Your Machine</span>
        <span class="ht-line ht-accent">Without Compromise</span>
      </h1>
      <p class="hero-sub">3D graphics, storage throughput, AI inference, memory bandwidth &amp; CPU —<br/>a full-spectrum PC benchmark powered by Rust, wgpu, and Tauri 2.</p>
      <div class="hero-actions">
        <a class="btn-primary" href="https://github.com/rito-1ura/fairybench/releases" target="_blank">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:16px;height:16px"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
          Download for Windows
        </a>
        <a class="btn-secondary" href="https://github.com/rito-1ura/fairybench" target="_blank">
          <svg viewBox="0 0 24 24" fill="currentColor" style="width:16px;height:16px"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/></svg>
          Source Code
        </a>
      </div>
    </div>
    <div class="hero-scroll" aria-hidden="true">
      <svg viewBox="0 0 16 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:12px;height:18px;opacity:.4"><rect x="1.5" y="1.5" width="13" height="21" rx="6.5"/><circle cx="8" cy="8" r="1.5" fill="currentColor"/></svg>
    </div>
  </section>

  <!-- Features -->
  <section class="features" id="features">
    <div class="sec-label">Benchmark Modules</div>
    <h2 class="sec-title">Everything your hardware has to offer</h2>
    <p class="sec-sub">Six meticulously measured dimensions. Real results, not synthetic estimates.</p>
    <div class="feat-grid">
      {#each [
        { icon: '🎮', title: '3D Graphics', desc: 'Rasterization, ray tracing, and procedural scenes via wgpu. Direct GPU compute.', color: '#818CF8' },
        { icon: '💾', title: 'Storage Throughput', desc: 'Sequential &amp; random I/O, IOPS, and latency. Measures real NVMe/SSD performance.', color: '#34D399' },
        { icon: '🧠', title: 'AI Inference', desc: 'Transformer &amp; generative model inference on CPU. Token throughput and latency.', color: '#F472B6' },
        { icon: '📊', title: 'Memory Bandwidth', desc: 'Read, write, and copy throughput. Cache-aware benchmarks for each level.', color: '#FBBF24' },
        { icon: '⚡', title: 'CPU Benchmarks', desc: 'Hash throughput, compression, sorting, and FP matrix operations. All cores utilized.', color: '#FB923C' },
        { icon: '🌡️', title: 'Thermal Monitor', desc: 'Real-time CPU/GPU temperature, power, and clock tracking during every test.', color: '#A78BFA' },
      ] as feat}
        <div class="feat-card" style="--card-accent: {feat.color}">
          <div class="feat-icon">{feat.icon}</div>
          <h3>{feat.title}</h3>
          <p>{@html feat.desc}</p>
        </div>
      {/each}
    </div>
  </section>

  <!-- Tech Stack -->
  <section class="tech">
    <div class="sec-label">Built With</div>
    <div class="tech-row">
      {#each ['Rust', 'Tauri 2', 'Svelte 5', 'wgpu', 'TypeScript', 'SQLite'] as t}
        <div class="tech-pill">{t}</div>
      {/each}
    </div>
  </section>

  <!-- Footer -->
  <footer class="footer">
    <div class="footer-inner">
      <div class="footer-logo">
        <svg viewBox="0 0 32 32" fill="none" style="width:20px;height:20px"><rect x="2" y="2" width="28" height="28" rx="7" fill="var(--accent)" fill-opacity=".15"/><text x="16" y="23" text-anchor="middle" fill="var(--accent)" font-size="20" font-weight="800" font-family="Inter,sans-serif">F</text></svg>
        <span>FairyBench</span>
      </div>
      <div class="footer-links">
        <a href="https://github.com/rito-1ura/fairybench" target="_blank">GitHub</a>
        <a href="https://github.com/rito-1ura/fairybench/releases" target="_blank">Releases</a>
      </div>
      <p class="footer-copy">© {new Date().getFullYear()} rito-1ura &middot; Open source under MIT</p>
    </div>
  </footer>
</div>

<style>
  /* ── Reset base ── */
  .landing-wrap :global(body) { margin: 0; background: #06060A; color: #E4E4E7; font-family: 'Inter', system-ui, -apple-system, sans-serif; -webkit-font-smoothing: antialiased; }
  .landing-wrap { background: #06060A; color: #E4E4E7; }

  /* ── Nav ── */
  .nav { position: fixed; top: 0; left: 0; right: 0; z-index: 100; padding: 16px 24px; }
  .nav-inner { max-width: 960px; margin: 0 auto; display: flex; align-items: center; justify-content: space-between; }
  .nav-logo { display: flex; align-items: center; gap: 8px; font-size: 15px; font-weight: 700; color: #fff; }
  .nav-links { display: flex; align-items: center; gap: 20px; font-size: 13px; }
  .nav-links a { color: #A1A1AA; text-decoration: none; transition: color .2s; }
  .nav-links a:hover { color: #fff; }
  .nav-dl {
    padding: 6px 14px; border-radius: 6px; background: var(--accent); color: #fff !important; font-weight: 600;
    transition: opacity .2s;
  }
  .nav-dl:hover { opacity: .85; }

  /* ── Hero ── */
  .hero { position: relative; min-height: 100vh; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center; padding: 80px 24px 60px; overflow: hidden; }
  .hero-bg { position: absolute; inset: 0; pointer-events: none; z-index: 0; }
  .hero-glow {
    position: absolute; top: -30%; left: 50%; translate: -50%; width: 700px; height: 700px;
    background: radial-gradient(circle, rgba(129,140,248,.12) 0%, transparent 70%);
  }
  .hero-grid {
    position: absolute; inset: 0;
    background-image: linear-gradient(rgba(255,255,255,.03) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,.03) 1px, transparent 1px);
    background-size: 40px 40px;
    mask-image: radial-gradient(ellipse at center, black 30%, transparent 70%);
    -webkit-mask-image: radial-gradient(ellipse at center, black 30%, transparent 70%);
  }
  .hero-body { position: relative; z-index: 1; max-width: 640px; }
  .hero-badge {
    display: inline-block; padding: 4px 12px; border-radius: 100px; font-size: 11px; font-weight: 500;
    background: rgba(129,140,248,.12); color: var(--accent); border: 1px solid rgba(129,140,248,.2); margin-bottom: 24px;
  }
  .hero-title { margin: 0 0 16px; }
  .ht-line { display: block; font-size: clamp(32px, 6vw, 56px); font-weight: 800; line-height: 1.1; letter-spacing: -0.03em; color: #fff; }
  .ht-accent { background: linear-gradient(135deg, #818CF8, #6366F1); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text; }
  .hero-sub { font-size: 15px; line-height: 1.6; color: #A1A1AA; margin: 0 0 32px; }
  .hero-actions { display: flex; gap: 12px; justify-content: center; flex-wrap: wrap; }
  .hero-actions a {
    display: inline-flex; align-items: center; gap: 8px; padding: 10px 22px; border-radius: 8px;
    font-size: 13px; font-weight: 600; text-decoration: none; transition: all .2s;
  }
  .btn-primary { background: var(--accent); color: #fff; }
  .btn-primary:hover { opacity: .85; }
  .btn-secondary { background: rgba(255,255,255,.06); color: #E4E4E7; border: 1px solid rgba(255,255,255,.1); }
  .btn-secondary:hover { background: rgba(255,255,255,.1); }
  .hero-scroll { position: absolute; bottom: 32px; left: 50%; translate: -50%; animation: bounce 2s infinite; }

  /* ── Sections ── */
  .features, .tech { max-width: 800px; margin: 0 auto; padding: 60px 24px; text-align: center; }
  .sec-label {
    display: inline-block; padding: 3px 10px; border-radius: 4px;
    font-size: 10px; font-weight: 600; letter-spacing: 0.08em; text-transform: uppercase;
    background: rgba(129,140,248,.1); color: var(--accent); margin-bottom: 12px;
  }
  .sec-title { font-size: 26px; font-weight: 700; color: #fff; margin: 0 0 8px; }
  .sec-sub { font-size: 14px; color: #A1A1AA; margin: 0 0 36px; }

  /* ── Feature Grid ── */
  .feat-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; text-align: left; }
  @media (max-width: 560px) { .feat-grid { grid-template-columns: 1fr; } }
  .feat-card {
    padding: 20px; border-radius: 10px;
    background: rgba(255,255,255,.03); border: 1px solid rgba(255,255,255,.06);
    transition: all .25s;
  }
  .feat-card:hover { background: rgba(255,255,255,.06); border-color: var(--card-accent); translate: 0 -2px; }
  .feat-icon { font-size: 24px; margin-bottom: 10px; }
  .feat-card h3 { font-size: 14px; font-weight: 600; color: #fff; margin: 0 0 6px; }
  .feat-card p { font-size: 12px; line-height: 1.5; color: #A1A1AA; margin: 0; }

  /* ── Tech ── */
  .tech-row { display: flex; gap: 10px; justify-content: center; flex-wrap: wrap; }
  .tech-pill {
    padding: 6px 16px; border-radius: 100px; font-size: 12px; font-weight: 500;
    background: rgba(255,255,255,.04); border: 1px solid rgba(255,255,255,.08); color: #A1A1AA;
  }

  /* ── Footer ── */
  .footer { border-top: 1px solid rgba(255,255,255,.06); padding: 32px 24px; }
  .footer-inner { max-width: 800px; margin: 0 auto; display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 8px; }
  .footer-logo { display: flex; align-items: center; gap: 6px; font-size: 13px; font-weight: 600; color: #E4E4E7; }
  .footer-links { display: flex; gap: 16px; font-size: 12px; }
  .footer-links a { color: #A1A1AA; text-decoration: none; transition: color .2s; }
  .footer-links a:hover { color: #fff; }
  .footer-copy { width: 100%; text-align: center; font-size: 11px; color: #52525B; margin: 12px 0 0; }

  @keyframes bounce { 0%,100% { translate: -50% 0; } 50% { translate: -50% 6px; } }
</style>
{:else}
  <div id="app">
    {#if mode === 'simple'}
      <SimpleMode onSwitch={() => mode = 'dashboard'} {version} />
    {:else}
      <DashboardMode onSwitch={() => mode = 'simple'} {version} />
    {/if}
  </div>
{/if}
