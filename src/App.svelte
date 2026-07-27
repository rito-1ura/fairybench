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

  // Landing page scroll state
  let scrolled = $state(0)
  let visibleSections = $state<Set<string>>(new Set())

  $effect(() => {
    if (isTauri) return
    document.documentElement.style.overflow = 'auto'
    document.documentElement.style.height = 'auto'
    document.body.style.overflow = 'auto'
    document.body.style.height = 'auto'
    const appEl = document.getElementById('app')
    if (appEl) { appEl.style.height = 'auto'; appEl.style.display = 'block' }

    const onScroll = () => scrolled = window.scrollY
    window.addEventListener('scroll', onScroll, { passive: true })

    const observer = new IntersectionObserver(
      (entries) => {
        for (const e of entries) {
          if (e.isIntersecting) {
            visibleSections.add(e.target.id)
            visibleSections = new Set(visibleSections)
          }
        }
      },
      { threshold: 0.15 }
    )
    // Observe all sections
    setTimeout(() => {
      document.querySelectorAll('[data-observe]').forEach(el => observer.observe(el))
    }, 100)

    return () => {
      window.removeEventListener('scroll', onScroll)
      observer.disconnect()
    }
  })

  const modules = [
    { name: '3D Raster', cat: 'GPU', desc: 'Direct GPU rasterization pipeline via wgpu. Measures vertex shading, fragment processing, and draw-call throughput.', color: '#818CF8' },
    { name: '3D Ray Trace', cat: 'GPU', desc: 'Hardware-accelerated ray tracing. BVH traversal, intersection, and shading performance.', color: '#818CF8' },
    { name: '3D Procedural', cat: 'GPU', desc: 'GPU compute shader terrain generation. Noise algorithms and mesh construction throughput.', color: '#818CF8' },
    { name: '3D Scene', cat: 'GPU', desc: 'Full 3D scene rendering with lighting, shadows, and post-processing effects.', color: '#818CF8' },
    { name: 'Storage IO', cat: 'Storage', desc: 'NVMe sequential/random read & write. IOPS, latency, and queue-depth scaling.', color: '#34D399' },
    { name: 'Memory BW', cat: 'RAM', desc: 'Memory read/write/copy bandwidth. Cache hierarchy-aware benchmarks (L1/L2/L3/RAM).', color: '#FBBF24' },
    { name: 'AI Inference', cat: 'AI', desc: 'Transformer inference pipeline. Token generation throughput and per-layer latency.', color: '#F472B6' },
    { name: 'AI Generative', cat: 'AI', desc: 'Generative model batch inference. Parallel prompt processing and decode speed.', color: '#F472B6' },
    { name: 'CPU Hash', cat: 'CPU', desc: 'SHA-256 hashing throughput. Measures integer ALU and memory pipeline performance.', color: '#FB923C' },
    { name: 'CPU Compress', cat: 'CPU', desc: 'LZ4-style run-length compression. Data transformation and branch prediction throughput.', color: '#FB923C' },
    { name: 'CPU Sort', cat: 'CPU', desc: 'Quicksort 2M elements. Memory access pattern and comparison throughput.', color: '#FB923C' },
    { name: 'CPU Float', cat: 'CPU', desc: '1024×128 FP matrix multiply. FMA pipeline utilization and GFLOPS measurement.', color: '#FB923C' },
  ]

  const categories = ['GPU', 'Storage', 'RAM', 'AI', 'CPU']

  const faqs = [
    { q: 'Is FairyBench free?', a: 'Yes. Fully open source under MIT license. No ads, no telemetry, no paywalls.' },
    { q: 'What hardware does it support?', a: 'Windows 10/11 with any modern GPU supporting wgpu (D3D12/Vulkan/Metal). NVMe SSDs recommended for storage tests.' },
    { q: 'How long does a benchmark take?', a: 'Full suite runs in 3-5 minutes. Individual modules run in 15-60 seconds each.' },
    { q: 'Can I compare results online?', a: 'Yes! Results are automatically submitted to the global leaderboard. View rankings at fairybench.vercel.app.' },
    { q: 'Does it affect system performance?', a: 'Benchmarks are read-only — they measure, they don\'t modify. Thermal monitoring is passive observation.' },
  ]
</script>

{#if !isTauri}
<!-- ═══════════════════════════════════════ LANDING PAGE ═══════════════════════════════════════ -->
<div class="lp">

<!-- ── HERO ── -->
<section class="lp-hero" id="hero">
  <div class="lp-hero-bg">
    <div class="lp-glow"></div>
    <div class="lp-grid"></div>
    {#each Array(20) as _, i}
      <div class="lp-particle" style="
        left: {Math.random() * 100}%;
        top: {Math.random() * 100}%;
        animation-delay: {Math.random() * 6}s;
        animation-duration: {6 + Math.random() * 8}s;
        width: {2 + Math.random() * 4}px;
        height: {2 + Math.random() * 4}px;
        opacity: {0.15 + Math.random() * 0.35};
      "></div>
    {/each}
  </div>

  <nav class="lp-nav" class:lp-nav-scrolled={scrolled > 60}>
    <div class="lp-nav-inner">
      <div class="lp-nav-logo">
        <svg viewBox="0 0 32 32" fill="none" style="width:22px;height:22px"><rect x="2" y="2" width="28" height="28" rx="7" fill="var(--accent)" fill-opacity=".2"/><text x="16" y="23" text-anchor="middle" fill="var(--accent)" font-size="20" font-weight="800">F</text></svg>
        <span>FairyBench</span>
      </div>
      <div class="lp-nav-links">
        <a href="#modules">Modules</a>
        <a href="#specs">Specs</a>
        <a href="#faq">FAQ</a>
        <a href="https://github.com/rito-1ura/fairybench" target="_blank">GitHub</a>
        <a class="lp-nav-dl" href="https://github.com/rito-1ura/fairybench/releases" target="_blank">Download</a>
      </div>
    </div>
  </nav>

  <div class="lp-hero-body" data-observe="hero">
    <div class="lp-hero-badge">v0.2.0 &mdash; Desktop Benchmark Suite</div>
    <h1 class="lp-hero-title">
      <span class="lp-ht">Measure Your Machine</span>
      <span class="lp-ht lp-ht-accent">Without Compromise</span>
    </h1>
    <p class="lp-hero-sub">12 benchmark modules spanning GPU, storage, memory, AI, and CPU &mdash;<br/>powered by Rust, wgpu, and Tauri 2. Real results, not estimates.</p>
    <div class="lp-hero-actions">
      <a class="lp-btn lp-btn-primary" href="https://github.com/rito-1ura/fairybench/releases" target="_blank">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:16px;height:16px"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        Download for Windows
      </a>
      <a class="lp-btn lp-btn-secondary" href="https://github.com/rito-1ura/fairybench" target="_blank">
        <svg viewBox="0 0 24 24" fill="currentColor" style="width:16px;height:16px"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/></svg>
        Source Code
      </a>
    </div>
  </div>
  <div class="lp-scroll-hint" aria-hidden="true">
    <svg viewBox="0 0 16 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:10px;height:16px;opacity:.3"><rect x="1.5" y="1.5" width="13" height="21" rx="6.5"/><circle cx="8" cy="8" r="1.5" fill="currentColor"/></svg>
  </div>
</section>

<!-- ── STATS ── -->
<section class="lp-stats" id="stats" data-observe="stats">
  {#each [
    { n: 12, l: 'Benchmark Modules', s: 'Across 5 categories' },
    { n: '5', l: 'Categories', s: 'GPU / Storage / RAM / AI / CPU' },
    { n: '3-5', l: 'Minutes per Run', s: 'Full suite benchmark' },
    { n: '1', l: 'Desktop App', s: 'No web. No cloud. Just Tauri.' },
  ] as stat}
    <div class="lp-stat-card" class:lp-visible={visibleSections.has('stats')}>
      <div class="lp-stat-n">{stat.n}</div>
      <div class="lp-stat-l">{stat.l}</div>
      <div class="lp-stat-s">{stat.s}</div>
    </div>
  {/each}
</section>

<!-- ── MODULES TABLE ── -->
<section class="lp-modules" id="modules" data-observe="modules">
  <div class="lp-sec-label">Benchmark Modules</div>
  <h2 class="lp-sec-title">12 modules, 5 categories</h2>
  <p class="lp-sec-sub">Every module runs in isolation. Results are scored, normalized, and logged to local SQLite with full history tracking.</p>

  {#each categories as cat}
    <div class="lp-cat-block" class:lp-visible={visibleSections.has('modules')}>
      <div class="lp-cat-head">
        <div class="lp-cat-dot" style="background: {modules.find(m => m.cat === cat)?.color || '#818CF8'}"></div>
        <span>{cat}</span>
        <span class="lp-cat-count">{modules.filter(m => m.cat === cat).length}</span>
      </div>
      {#each modules.filter(m => m.cat === cat) as mod, i}
        <div class="lp-mod-row" style="transition-delay: {i * 0.04}s">
          <div class="lp-mod-name">{mod.name}</div>
          <div class="lp-mod-desc">{mod.desc}</div>
          <div class="lp-mod-tag" style="background: {mod.color}15; color: {mod.color}; border-color: {mod.color}30">{mod.cat}</div>
        </div>
      {/each}
    </div>
  {/each}
</section>

<!-- ── TECH SPECS ── -->
<section class="lp-specs" id="specs" data-observe="specs">
  <div class="lp-sec-label">Technical Specifications</div>
  <h2 class="lp-sec-title">Built for performance, measured in precision</h2>

  <div class="lp-specs-grid">
    <div class="lp-spec-card" class:lp-visible={visibleSections.has('specs')}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M8 21h8M12 17v4"/></svg>
      <h3>Rendering Engine</h3>
      <p>wgpu (WebGPU) with D3D12/Vulkan/Metal backends. GPU compute shaders for procedural generation and post-processing.</p>
    </div>
    <div class="lp-spec-card" class:lp-visible={visibleSections.has('specs')} style="transition-delay:0.1s">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><path d="M4 7V4h16v3"/><path d="M9 20h6"/><path d="M12 4v16"/></svg>
      <h3>Desktop Native</h3>
      <p>Tauri 2 framework. Single ~8MB executable. No Electron overhead. Native Windows 10/11 API access for thermal monitoring.</p>
    </div>
    <div class="lp-spec-card" class:lp-visible={visibleSections.has('specs')} style="transition-delay:0.2s">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
      <h3>Real-time Analysis</h3>
      <p>Live scoring with animated transitions. Confidence intervals, CV, outlier detection. Per-module breakdown and historical trends.</p>
    </div>
    <div class="lp-spec-card" class:lp-visible={visibleSections.has('specs')} style="transition-delay:0.3s">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
      <h3>Privacy First</h3>
      <p>All data stored locally in SQLite. Online leaderboard is opt-in. No telemetry, no analytics, no data collection.</p>
    </div>
    <div class="lp-spec-card" class:lp-visible={visibleSections.has('specs')} style="transition-delay:0.4s">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
      <h3>Fast Runs</h3>
      <p>Full 12-module suite completes in 3-5 minutes. Each module 15-60s. Concurrent GPU + CPU execution where possible.</p>
    </div>
    <div class="lp-spec-card" class:lp-visible={visibleSections.has('specs')} style="transition-delay:0.5s">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><path d="M12 2v4M12 18v4"/><path d="M4 12H2M22 12h-2"/><circle cx="12" cy="12" r="4"/></svg>
      <h3>Thermal Monitoring</h3>
      <p>Real-time CPU/GPU temperature, power draw, and clock frequency sampled every 2 seconds during benchmarks.</p>
    </div>
  </div>
</section>

<!-- ── DOWNLOAD ── -->
<section class="lp-download" id="download" data-observe="download">
  <div class="lp-dl-card" class:lp-visible={visibleSections.has('download')}>
    <h2>Ready to benchmark?</h2>
    <p>Download the latest release for Windows. No installer required &mdash; portable executable.</p>
    <div class="lp-dl-actions">
      <a class="lp-btn lp-btn-primary lp-btn-lg" href="https://github.com/rito-1ura/fairybench/releases" target="_blank">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:18px;height:18px"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        Download v0.2.0
      </a>
      <a class="lp-btn lp-btn-ghost" href="https://github.com/rito-1ura/fairybench" target="_blank">View on GitHub</a>
    </div>
    <div class="lp-dl-meta">Windows 10/11 &middot; ~8 MB &middot; Open Source (MIT)</div>
  </div>
</section>

<!-- ── FAQ ── -->
<section class="lp-faq" id="faq" data-observe="faq">
  <div class="lp-sec-label">FAQ</div>
  <h2 class="lp-sec-title">Common questions</h2>
  <div class="lp-faq-list" class:lp-visible={visibleSections.has('faq')}>
    {#each faqs as item}
      <details class="lp-faq-item">
        <summary><span>{item.q}</span><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:14px;height:14px"><polyline points="6 9 12 15 18 9"/></svg></summary>
        <p>{item.a}</p>
      </details>
    {/each}
  </div>
</section>

<!-- ── FOOTER ── -->
<footer class="lp-footer">
  <div class="lp-footer-inner">
    <div class="lp-fcol">
      <div class="lp-footer-logo">
        <svg viewBox="0 0 32 32" fill="none" style="width:18px;height:18px"><rect x="2" y="2" width="28" height="28" rx="7" fill="var(--accent)" fill-opacity=".15"/><text x="16" y="23" text-anchor="middle" fill="var(--accent)" font-size="20" font-weight="800">F</text></svg>
        <span>FairyBench</span>
      </div>
      <p class="lp-footer-desc">Open-source PC benchmark suite. Built with Rust, Tauri 2, Svelte 5, and wgpu.</p>
    </div>
    <div class="lp-fcol">
      <span class="lp-fh">Links</span>
      <a href="https://github.com/rito-1ura/fairybench" target="_blank">GitHub</a>
      <a href="https://github.com/rito-1ura/fairybench/releases" target="_blank">Releases</a>
    </div>
    <div class="lp-fcol">
      <span class="lp-fh">Tech</span>
      <a href="https://tauri.app" target="_blank">Tauri 2</a>
      <a href="https://svelte.dev" target="_blank">Svelte 5</a>
      <a href="https://wgpu.rs" target="_blank">wgpu</a>
    </div>
  </div>
  <div class="lp-footer-copy">&copy; {new Date().getFullYear()} rito-1ura &middot; MIT License</div>
</footer>

</div>

<style>
/* ═══ LANDING PAGE STYLES ═══ */
.lp { background: #06060A; color: #E4E4E7; font-family: 'Inter', system-ui, sans-serif; -webkit-font-smoothing: antialiased; overflow-x: hidden; }

/* ── Animations ── */
@keyframes lpFadeUp {
  from { opacity: 0; translate: 0 30px; }
  to { opacity: 1; translate: 0 0; }
}
@keyframes lpFadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
@keyframes lpDrift {
  0%, 100% { translate: 0 0; }
  50% { translate: 0 -20px; }
}
@keyframes lpBounce {
  0%, 100% { translate: -50% 0; }
  50% { translate: -50% 8px; }
}
[data-observe] { opacity: 0; translate: 0 20px; transition: opacity .6s ease-out, translate .6s ease-out; }
[data-observe].lp-visible { opacity: 1; translate: 0 0; }

/* ── Nav ── */
.lp-nav { position: fixed; top: 0; left: 0; right: 0; z-index: 100; padding: 12px 24px; transition: background .3s, backdrop-filter .3s; }
.lp-nav-scrolled { background: rgba(6,6,10,.85); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); }
.lp-nav-inner { max-width: 960px; margin: 0 auto; display: flex; align-items: center; justify-content: space-between; }
.lp-nav-logo { display: flex; align-items: center; gap: 8px; font-size: 14px; font-weight: 700; color: #fff; }
.lp-nav-links { display: flex; align-items: center; gap: 16px; font-size: 12px; }
.lp-nav-links a { color: #A1A1AA; text-decoration: none; transition: color .2s; }
.lp-nav-links a:hover { color: #fff; }
.lp-nav-dl {
  padding: 5px 14px; border-radius: 6px; background: var(--accent); color: #fff !important; font-weight: 600; transition: opacity .2s;
}
.lp-nav-dl:hover { opacity: .8; }

/* ── Hero ── */
.lp-hero { position: relative; min-height: 100vh; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center; padding: 80px 24px 60px; overflow: hidden; }
.lp-hero-bg { position: absolute; inset: 0; pointer-events: none; z-index: 0; }
.lp-glow { position: absolute; top: -25%; left: 50%; translate: -50%; width: 800px; height: 800px; background: radial-gradient(circle, rgba(129,140,248,.1) 0%, transparent 65%); }
.lp-grid { position: absolute; inset: 0; background-image: linear-gradient(rgba(255,255,255,.025) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,.025) 1px, transparent 1px); background-size: 40px 40px; mask-image: radial-gradient(ellipse at center, black 25%, transparent 70%); -webkit-mask-image: radial-gradient(ellipse at center, black 25%, transparent 70%); }
.lp-particle { position: absolute; border-radius: 50%; background: var(--accent); pointer-events: none; animation: lpDrift linear infinite; }

.lp-hero-body { position: relative; z-index: 1; max-width: 640px; animation: lpFadeIn .8s ease-out; }
.lp-hero-badge { display: inline-block; padding: 4px 12px; border-radius: 100px; font-size: 10px; font-weight: 500; background: rgba(129,140,248,.12); color: var(--accent); border: 1px solid rgba(129,140,248,.2); margin-bottom: 20px; letter-spacing: .02em; }
.lp-hero-title { margin: 0 0 14px; }
.lp-ht { display: block; font-size: clamp(32px, 6vw, 56px); font-weight: 800; line-height: 1.1; letter-spacing: -0.03em; color: #fff; }
.lp-ht-accent { background: linear-gradient(135deg, #818CF8, #6366F1); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text; }
.lp-hero-sub { font-size: 14px; line-height: 1.7; color: #A1A1AA; margin: 0 0 28px; }
.lp-hero-actions { display: flex; gap: 10px; justify-content: center; flex-wrap: wrap; }

.lp-btn { display: inline-flex; align-items: center; gap: 8px; padding: 9px 20px; border-radius: 8px; font-size: 13px; font-weight: 600; text-decoration: none; transition: all .2s; cursor: pointer; }
.lp-btn-primary { background: var(--accent); color: #fff; }
.lp-btn-primary:hover { translate: 0 -1px; box-shadow: 0 8px 24px rgba(129,140,248,.25); }
.lp-btn-secondary { background: rgba(255,255,255,.06); color: #E4E4E7; border: 1px solid rgba(255,255,255,.1); }
.lp-btn-secondary:hover { background: rgba(255,255,255,.1); translate: 0 -1px; }
.lp-btn-lg { padding: 12px 28px; font-size: 15px; }
.lp-btn-ghost { background: transparent; color: #A1A1AA; border: 1px solid rgba(255,255,255,.08); }
.lp-btn-ghost:hover { color: #fff; border-color: rgba(255,255,255,.2); }

.lp-scroll-hint { position: absolute; bottom: 28px; left: 50%; translate: -50%; animation: lpBounce 2s infinite; }

/* ── Stats ── */
.lp-stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; max-width: 800px; margin: -40px auto 0; padding: 0 24px 60px; position: relative; z-index: 2; }
@media (max-width: 600px) { .lp-stats { grid-template-columns: repeat(2, 1fr); } }
.lp-stat-card {
  padding: 20px 16px; border-radius: 10px; text-align: center;
  background: rgba(255,255,255,.03); border: 1px solid rgba(255,255,255,.06);
  transition: all .35s ease-out; transition-delay: inherit;
}
.lp-stat-card.lp-visible { animation: lpFadeUp .5s ease-out forwards; }
.lp-stat-n { font-size: 28px; font-weight: 800; color: #fff; margin-bottom: 4px; }
.lp-stat-l { font-size: 12px; font-weight: 600; color: #E4E4E7; margin-bottom: 2px; }
.lp-stat-s { font-size: 11px; color: #52525B; }

/* ── Section Common ── */
.lp-sec-label {
  display: inline-block; padding: 3px 10px; border-radius: 4px;
  font-size: 10px; font-weight: 600; letter-spacing: .06em; text-transform: uppercase;
  background: rgba(129,140,248,.1); color: var(--accent); margin-bottom: 10px;
}
.lp-sec-title { font-size: 26px; font-weight: 700; color: #fff; margin: 0 0 8px; }
.lp-sec-sub { font-size: 14px; color: #A1A1AA; margin: 0 0 36px; line-height: 1.5; }

/* ── Modules ── */
.lp-modules { max-width: 800px; margin: 0 auto; padding: 60px 24px; text-align: center; }
.lp-cat-block { text-align: left; margin-bottom: 20px; }
.lp-cat-block.lp-visible .lp-mod-row { animation: lpFadeUp .4s ease-out forwards; opacity: 0; }
.lp-cat-head { display: flex; align-items: center; gap: 8px; font-size: 13px; font-weight: 600; color: #fff; margin-bottom: 8px; padding-left: 2px; }
.lp-cat-dot { width: 8px; height: 8px; border-radius: 50%; }
.lp-cat-count { margin-left: auto; font-size: 10px; color: #52525B; background: rgba(255,255,255,.04); padding: 1px 8px; border-radius: 4px; }
.lp-mod-row {
  display: flex; align-items: center; gap: 12px;
  padding: 10px 14px; margin-bottom: 4px;
  background: rgba(255,255,255,.02); border-radius: 8px;
  border: 1px solid rgba(255,255,255,.04);
  transition: all .2s;
}
.lp-mod-row:hover { background: rgba(255,255,255,.04); border-color: rgba(255,255,255,.08); }
.lp-mod-name { font-size: 12px; font-weight: 600; color: #fff; min-width: 100px; }
.lp-mod-desc { font-size: 11px; color: #A1A1AA; flex: 1; line-height: 1.4; }
.lp-mod-tag { font-size: 9px; font-weight: 600; padding: 2px 8px; border-radius: 4px; border: 1px solid; white-space: nowrap; }
@media (max-width: 640px) { .lp-mod-row { flex-wrap: wrap; gap: 4px; } .lp-mod-desc { width: 100%; order: 3; } }

/* ── Specs ── */
.lp-specs { max-width: 800px; margin: 0 auto; padding: 60px 24px; text-align: center; }
.lp-specs-grid { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px; text-align: left; }
@media (max-width: 660px) { .lp-specs-grid { grid-template-columns: 1fr 1fr; } }
@media (max-width: 440px) { .lp-specs-grid { grid-template-columns: 1fr; } }
.lp-spec-card {
  padding: 20px; border-radius: 10px;
  background: rgba(255,255,255,.02); border: 1px solid rgba(255,255,255,.06);
  transition: all .35s ease-out;
}
.lp-spec-card:hover { background: rgba(255,255,255,.05); border-color: var(--accent); translate: 0 -2px; }
.lp-spec-card.lp-visible { animation: lpFadeUp .5s ease-out forwards; }
.lp-spec-card :global(svg) { color: var(--accent); margin-bottom: 10px; }
.lp-spec-card h3 { font-size: 13px; font-weight: 600; color: #fff; margin: 0 0 6px; }
.lp-spec-card p { font-size: 11px; line-height: 1.5; color: #A1A1AA; margin: 0; }

/* ── Download CTA ── */
.lp-download { max-width: 800px; margin: 0 auto; padding: 60px 24px; }
.lp-dl-card {
  padding: 40px 32px; border-radius: 12px; text-align: center;
  background: linear-gradient(135deg, rgba(129,140,248,.06), rgba(99,102,241,.04));
  border: 1px solid rgba(129,140,248,.12);
  transition: all .5s ease-out;
}
.lp-dl-card.lp-visible { animation: lpFadeUp .5s ease-out forwards; }
.lp-dl-card h2 { font-size: 24px; font-weight: 700; color: #fff; margin: 0 0 8px; }
.lp-dl-card > p { font-size: 13px; color: #A1A1AA; margin: 0 0 24px; }
.lp-dl-actions { display: flex; gap: 10px; justify-content: center; flex-wrap: wrap; }
.lp-dl-meta { font-size: 11px; color: #52525B; margin-top: 16px; }

/* ── FAQ ── */
.lp-faq { max-width: 800px; margin: 0 auto; padding: 60px 24px; text-align: center; }
.lp-faq-list { text-align: left; max-width: 560px; margin: 0 auto; }
.lp-faq-list.lp-visible .lp-faq-item { animation: lpFadeUp .4s ease-out forwards; }
.lp-faq-item {
  border-bottom: 1px solid rgba(255,255,255,.06); padding: 14px 0;
  opacity: 0; transition-delay: inherit;
}
.lp-faq-item summary {
  display: flex; justify-content: space-between; align-items: center;
  font-size: 13px; font-weight: 500; color: #E4E4E7; cursor: pointer; padding: 2px 0;
  list-style: none;
}
.lp-faq-item summary::-webkit-details-marker { display: none; }
.lp-faq-item[open] summary svg { rotate: 180deg; }
.lp-faq-item summary svg { transition: rotate .2s; color: #52525B; }
.lp-faq-item p { font-size: 12px; color: #A1A1AA; line-height: 1.6; margin: 8px 0 0; }

/* ── Footer ── */
.lp-footer { border-top: 1px solid rgba(255,255,255,.06); padding: 40px 24px 24px; }
.lp-footer-inner { max-width: 800px; margin: 0 auto; display: flex; gap: 40px; flex-wrap: wrap; }
.lp-fcol { display: flex; flex-direction: column; gap: 6px; }
.lp-fcol:first-child { flex: 2; min-width: 200px; }
.lp-footer-logo { display: flex; align-items: center; gap: 6px; font-size: 13px; font-weight: 600; color: #E4E4E7; }
.lp-footer-desc { font-size: 11px; color: #52525B; margin: 4px 0 0; line-height: 1.5; }
.lp-fh { font-size: 11px; font-weight: 600; color: #A1A1AA; margin-bottom: 4px; }
.lp-fcol a { font-size: 12px; color: #52525B; text-decoration: none; transition: color .2s; }
.lp-fcol a:hover { color: #E4E4E7; }
.lp-footer-copy { max-width: 800px; margin: 24px auto 0; text-align: center; font-size: 11px; color: #3F3F46; }

@keyframes bounce { 0%,100% { translate: -50% 0; } 50% { translate: -50% 8px; } }
[data-observe] { opacity: 0; translate: 0 20px; transition: opacity .6s ease-out, translate .6s ease-out; }
[data-observe].lp-visible { opacity: 1; translate: 0 0; }
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