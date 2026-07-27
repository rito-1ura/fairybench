<script lang="ts">
  import './app.css'
  import SimpleMode from './lib/SimpleMode.svelte'
  import DashboardMode from './lib/DashboardMode.svelte'
  import { t, setLang, getLang } from './lib/i18n'

  let mode = $state<'simple' | 'dashboard'>(
    (typeof localStorage !== 'undefined' ? localStorage.getItem('mode') as 'simple' | 'dashboard' : null) || 'simple'
  )
  let version = $state('')
  let isTauri = $state(false)
  let lpLang = $state<'en' | 'ja'>('en')

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
    // Restore landing lang
    const saved = localStorage.getItem('landing_lang') as 'en' | 'ja' | null
    if (saved === 'ja' || saved === 'en') lpLang = saved
  })

  function switchLpLang(l: 'en' | 'ja') {
    lpLang = l
    localStorage.setItem('landing_lang', l)
  }

  function lp(key: string): string {
    const v = i18n_lp[lpLang]?.[key]
    return v || i18n_lp.en[key] || key
  }

  // Scroll observer for animations
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
    setTimeout(() => {
      document.querySelectorAll('[data-observe]').forEach(el => observer.observe(el))
    }, 100)
    return () => {
      window.removeEventListener('scroll', onScroll)
      observer.disconnect()
    }
  })

  const modules = [
    { name: '3D Raster', cat: 'GPU', desc: 'Direct GPU rasterization pipeline via wgpu.', color: '#818CF8' },
    { name: '3D Ray Trace', cat: 'GPU', desc: 'Hardware-accelerated ray tracing. BVH traversal.', color: '#818CF8' },
    { name: '3D Procedural', cat: 'GPU', desc: 'GPU compute shader terrain generation.', color: '#818CF8' },
    { name: '3D Scene', cat: 'GPU', desc: 'Full 3D scene with lighting, shadows, post-processing.', color: '#818CF8' },
    { name: 'Storage IO', cat: 'Storage', desc: 'NVMe sequential/random read & write. IOPS, latency.', color: '#34D399' },
    { name: 'Memory BW', cat: 'RAM', desc: 'Memory read/write/copy. Cache hierarchy-aware.', color: '#FBBF24' },
    { name: 'AI Inference', cat: 'AI', desc: 'Transformer inference pipeline.', color: '#F472B6' },
    { name: 'AI Generative', cat: 'AI', desc: 'Generative model batch inference.', color: '#F472B6' },
    { name: 'CPU Hash', cat: 'CPU', desc: 'SHA-256 hashing throughput.', color: '#FB923C' },
    { name: 'CPU Compress', cat: 'CPU', desc: 'LZ4 run-length compression.', color: '#FB923C' },
    { name: 'CPU Sort', cat: 'CPU', desc: 'Quicksort 2M elements.', color: '#FB923C' },
    { name: 'CPU Float', cat: 'CPU', desc: '1024×128 FP matrix multiply.', color: '#FB923C' },
  ]

  const i18n_lp: Record<string, Record<string, string>> = {
    en: {
      badge: 'v0.2.0 — Desktop Benchmark Suite',
      title1: 'Measure Your Machine',
      title2: 'Without Compromise',
      sub: "12 benchmark modules spanning GPU, storage, memory, AI, and CPU — powered by Rust, wgpu, and Tauri 2.",
      dl_btn: 'Download for Windows',
      src_btn: 'Source Code',
      nav_mod: 'Modules', nav_spec: 'Specs', nav_faq: 'FAQ', nav_gh: 'GitHub', nav_dl: 'Download',
      s1n: '12', s1l: 'Benchmark Modules', s1s: 'Across 5 categories',
      s2n: '5', s2l: 'Categories', s2s: 'GPU / Storage / RAM / AI / CPU',
      s3n: '3-5', s3l: 'Minutes per Run', s3s: 'Full suite benchmark',
      s4n: '1', s4l: 'Desktop App', s4s: 'Native. No web. No cloud.',
      mod_l: 'Benchmark Modules', mod_t: '12 modules, 5 categories',
      mod_s: 'Every module runs in isolation. Results are scored, normalized, and logged to local SQLite.',
      spec_l: 'Technical Specifications', spec_t: 'Built for performance, measured in precision',
      spec: [
        {t:'Rendering Engine',d:'wgpu (WebGPU) with D3D12/Vulkan/Metal backends. GPU compute shaders for procedural generation.'},
        {t:'Desktop Native',d:'Tauri 2 framework. Single ~8MB executable. No Electron overhead. Native Windows API access.'},
        {t:'Real-time Analysis',d:'Live scoring with CI, CV, outlier detection. Per-module breakdown and historical trends.'},
        {t:'Privacy First',d:'All data stored locally in SQLite. Online leaderboard is opt-in. No telemetry.'},
        {t:'Fast Runs',d:'Full 12-module suite completes in 3-5 minutes. Concurrent GPU + CPU execution.'},
        {t:'Thermal Monitoring',d:'Real-time CPU/GPU temperature, power, and clock sampled every 2 seconds.'},
      ],
      dl_t: 'Ready to benchmark?', dl_s: 'Download the latest release for Windows. Portable, no installer.',
      dl_b: 'Download v0.2.0', dl_gh: 'View on GitHub', dl_m: 'Windows 10/11 · ~8 MB · Open Source (MIT)',
      faq_l: 'FAQ', faq_t: 'Common questions',
      faqs: [
        {q:'Is FairyBench free?',a:'Yes. Fully open source under MIT. No ads, no telemetry, no paywalls.'},
        {q:'What hardware does it support?',a:'Windows 10/11 with wgpu-capable GPU (D3D12/Vulkan/Metal). NVMe SSDs recommended.'},
        {q:'How long does a benchmark take?',a:'Full suite 3-5 min. Individual modules 15-60 sec.'},
        {q:'Can I compare results online?',a:'Yes! Auto-submits to the global leaderboard at fairybench.vercel.app.'},
        {q:'Does it affect system performance?',a:"Benchmarks are read-only. Thermal monitoring is passive observation."},
      ],
      show_l: 'Desktop App Preview', show_t: 'See it in action', show_s: 'Real-time dashboard with live scoring, thermal monitoring, and module telemetry.',
      ft: 'Open-source PC benchmark suite. Rust + Tauri 2 + Svelte 5 + wgpu.',
    },
    ja: {
      badge: 'v0.2.0 — デスクトップベンチマーク',
      title1: 'あなたのマシンを',
      title2: '徹底測定',
      sub: "GPU、ストレージ、メモリ、AI、CPUの12モジュール。Rust、wgpu、Tauri 2で構築。",
      dl_btn: 'Windows版をダウンロード',
      src_btn: 'ソースコード',
      nav_mod: 'モジュール', nav_spec: 'スペック', nav_faq: 'よくある質問', nav_gh: 'GitHub', nav_dl: 'ダウンロード',
      s1n: '12', s1l: 'ベンチマークモジュール', s1s: '5カテゴリ',
      s2n: '5', s2l: 'カテゴリ', s2s: 'GPU / ストレージ / RAM / AI / CPU',
      s3n: '3-5', s3l: '分で完了', s3s: 'フルスイート',
      s4n: '1', s4l: 'デスクトップアプリ', s4s: 'ネイティブ。Web不要。',
      mod_l: 'ベンチマークモジュール', mod_t: '12モジュール、5カテゴリ',
      mod_s: '各モジュールは独立実行。結果はSQLiteに履歴保存。',
      spec_l: '技術仕様', spec_t: '精密測定のために設計',
      spec: [
        {t:'レンダリングエンジン',d:'wgpu（WebGPU）採用。GPUコンピュートシェーダーでプロシージャル生成。'},
        {t:'デスクトップネイティブ',d:'Tauri 2。約8MB。Electron不要。ネイティブWindows API。'},
        {t:'リアルタイム解析',d:'信頼区間、CV、外れ値検出。モジュール別内訳と履歴トレンド。'},
        {t:'プライバシー最優先',d:'全データはローカル保存。テレメトリなし。'},
        {t:'高速実行',d:'フルスイート3-5分。GPU+CPU同時実行可能。'},
        {t:'温度監視',d:'CPU/GPU温度、消費電力、クロックを2秒間隔でサンプリング。'},
      ],
      dl_t: 'ベンチマークを始めましょう', dl_s: 'Windows向け最新版。インストーラー不要のポータブル。',
      dl_b: 'v0.2.0 をダウンロード', dl_gh: 'GitHubで見る', dl_m: 'Windows 10/11 · ~8 MB · オープンソース (MIT)',
      faq_l: 'よくある質問', faq_t: 'よくある質問',
      faqs: [
        {q:'FairyBenchは無料ですか？',a:'はい。MITライセンスの完全オープンソース。'},
        {q:'対応ハードウェアは？',a:'Windows 10/11、wgpu対応GPU。NVMe SSD推奨。'},
        {q:'所要時間は？',a:'フルスイート3-5分。個別15-60秒。'},
        {q:'オンラインで比較できますか？',a:'はい。fairybench.vercel.appでランキングを確認。'},
        {q:'システムに影響は？',a:'ベンチマークは読み取り専用。変更は加えません。'},
      ],
      show_l: 'デスクトップアプリプレビュー', show_t: '実際の動作画面', show_s: 'リアルタイムダッシュボード。ライブスコアリング、温度監視。',
      ft: 'オープンソースPCベンチマーク。Rust + Tauri 2 + Svelte 5 + wgpu。',
    },
  }
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
        <a href="#modules">{lp('nav_mod')}</a>
        <a href="#specs">{lp('nav_spec')}</a>
        <a href="#faq">{lp('nav_faq')}</a>
        <a href="https://github.com/rito-1ura/fairybench" target="_blank">{lp('nav_gh')}</a>
        <div class="lp-nav-lang">
          <button class="lp-lang-btn" class:active={lpLang==='en'} onclick={()=>switchLpLang('en')}>EN</button>
          <button class="lp-lang-btn" class:active={lpLang==='ja'} onclick={()=>switchLpLang('ja')}>JA</button>
        </div>
        <a class="lp-nav-dl" href="https://github.com/rito-1ura/fairybench/releases" target="_blank">{lp('nav_dl')}</a>
      </div>
    </div>
  </nav>

  <div class="lp-hero-body">
    <div class="lp-hero-badge">{lp('badge')}</div>
    <h1 class="lp-hero-title">
      <span class="lp-ht">{lp('title1')}</span>
      <span class="lp-ht lp-ht-accent">{lp('title2')}</span>
    </h1>
    <p class="lp-hero-sub">{lp('sub')}</p>
    <div class="lp-hero-actions">
      <a class="lp-btn lp-btn-primary" href="https://github.com/rito-1ura/fairybench/releases" target="_blank">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:16px;height:16px"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        {lp('dl_btn')}
      </a>
      <a class="lp-btn lp-btn-secondary" href="https://github.com/rito-1ura/fairybench" target="_blank">
        <svg viewBox="0 0 24 24" fill="currentColor" style="width:16px;height:16px"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/></svg>
        {lp('src_btn')}
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
    { n: lp('s1n'), l: lp('s1l'), s: lp('s1s') },
    { n: lp('s2n'), l: lp('s2l'), s: lp('s2s') },
    { n: lp('s3n'), l: lp('s3l'), s: lp('s3s') },
    { n: lp('s4n'), l: lp('s4l'), s: lp('s4s') },
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
  <div class="lp-sec-label">{lp('mod_l')}</div>
  <h2 class="lp-sec-title">{lp('mod_t')}</h2>
  <p class="lp-sec-sub">{lp('mod_s')}</p>

  {#each ['GPU', 'Storage', 'RAM', 'AI', 'CPU'] as cat}
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
  <div class="lp-sec-label">{lp('spec_l')}</div>
  <h2 class="lp-sec-title">{lp('spec_t')}</h2>
  <div class="lp-specs-grid">
    {#each ['spec_1','spec_2','spec_3','spec_4','spec_5','spec_6'] as s, i}
      <div class="lp-spec-card" class:lp-visible={visibleSections.has('specs')} style="transition-delay:{i*0.1}s">
        <div class="lp-spec-svg">
          {#if s === 'spec_1'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M8 21h8M12 17v4"/><path d="M5 7l7 4 7-4"/></svg>
          {:else if s === 'spec_2'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M9 9l6 6M15 9l-6 6"/><circle cx="12" cy="12" r="10"/></svg>
          {:else if s === 'spec_3'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
          {:else if s === 'spec_4'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
          {:else if s === 'spec_5'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
          {:else if s === 'spec_6'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:18px;height:18px"><path d="M12 2v4M12 18v4"/><path d="M4 12H2M22 12h-2"/><circle cx="12" cy="12" r="4"/></svg>
          {/if}
        </div>
        <h3>{i18n_lp[lpLang].spec[i]?.t || ''}</h3>
        <p>{i18n_lp[lpLang].spec[i]?.d || ''}</p>
      </div>
    {/each}
  </div>
</section>

<!-- ── APP SHOWCASE ── -->
<section class="lp-showcase" id="showcase" data-observe="showcase">
  <div class="lp-sec-label">{lp('show_l')}</div>
  <h2 class="lp-sec-title">{lp('show_t')}</h2>
  <p class="lp-sec-sub">{lp('show_s')}</p>
  <div class="lp-showcase-screens" class:lp-visible={visibleSections.has('showcase')}>
    <!-- Dashboard mockup -->
    <div class="lp-mockup">
      <div class="lp-mockup-bar">
        <div class="lp-mockup-dots"><span></span><span></span><span></span></div>
        <span class="lp-mockup-title">FairyBench — Dashboard</span>
      </div>
      <div class="lp-mockup-body">
        <div class="lp-mock-col">
          <div class="lp-mock-card"><div class="lp-mock-label">Final Score</div><div class="lp-mock-val" style="color:var(--accent)">125,678</div><div class="lp-mock-bar" style="width:80%"></div></div>
          <div class="lp-mock-card"><div class="lp-mock-label">95% CI</div><div class="lp-mock-val">122,450 – 128,900</div><div class="lp-mock-bar" style="width:70%"></div></div>
        </div>
        <div class="lp-mock-col">
          <div class="lp-mock-card"><div class="lp-mock-label">CPU Hash</div><div class="lp-mock-val">42,315</div><div class="lp-mock-bar" style="width:65%"></div></div>
          <div class="lp-mock-card"><div class="lp-mock-label">3D Scene</div><div class="lp-mock-val">18,240</div><div class="lp-mock-bar" style="width:55%"></div></div>
          <div class="lp-mock-card"><div class="lp-mock-label">Memory BW</div><div class="lp-mock-val">31,500</div><div class="lp-mock-bar" style="width:72%"></div></div>
        </div>
        <div class="lp-mock-col">
          <div class="lp-mock-graph">
            <svg viewBox="0 0 100 40" style="width:100%;height:100%"><polyline points="0,35 15,28 30,30 45,18 60,22 75,10 90,14 100,8" fill="none" stroke="var(--accent)" stroke-width="1.5" opacity=".6"/><polyline points="0,35 15,28 30,30 45,18 60,22 75,10 90,14 100,8" fill="none" stroke="var(--accent)" stroke-width="2"/><circle cx="100" cy="8" r="2" fill="var(--accent)"/></svg>
          </div>
          <div class="lp-mock-temp"><span>CPU 62°C</span><span>GPU 58°C</span></div>
        </div>
      </div>
    </div>
  </div>
</section>

<!-- ── DOWNLOAD ── -->
<section class="lp-download" id="download" data-observe="download">
  <div class="lp-dl-card" class:lp-visible={visibleSections.has('download')}>
    <h2>{lp('dl_t')}</h2>
    <p>{lp('dl_s')}</p>
    <div class="lp-dl-actions">
      <a class="lp-btn lp-btn-primary lp-btn-lg" href="https://github.com/rito-1ura/fairybench/releases" target="_blank">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:18px;height:18px"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        {lp('dl_b')}
      </a>
      <a class="lp-btn lp-btn-ghost" href="https://github.com/rito-1ura/fairybench" target="_blank">{lp('dl_gh')}</a>
    </div>
    <div class="lp-dl-meta">{lp('dl_m')}</div>
  </div>
</section>

<!-- ── FAQ ── -->
<section class="lp-faq" id="faq" data-observe="faq">
  <div class="lp-sec-label">{lp('faq_l')}</div>
  <h2 class="lp-sec-title">{lp('faq_t')}</h2>
  <div class="lp-faq-list" class:lp-visible={visibleSections.has('faq')}>
    {#each i18n_lp[lpLang].faqs as item}
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
      <p class="lp-footer-desc">{lp('ft')}</p>
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
.lp { background: #06060A; color: #E4E4E7; font-family: 'Inter', system-ui, sans-serif; -webkit-font-smoothing: antialiased; overflow-x: hidden; min-height: 100vh; }

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
[data-observe] { transition: opacity .6s ease-out, translate .6s ease-out; }
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
  .lp-nav-lang { display: flex; gap: 2px; }
  .lp-lang-btn {
    padding: 2px 8px; font-size: 10px; font-weight: 600; border-radius: 4px;
    background: transparent; color: #52525B; border: 1px solid rgba(255,255,255,.08); cursor: pointer; transition: all .2s;
  }
  .lp-lang-btn.active { background: var(--accent); color: #fff; border-color: var(--accent); }
  .lp-lang-btn:hover:not(.active) { color: #A1A1AA; border-color: rgba(255,255,255,.2); }

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

/* ── Showcase ── */
.lp-showcase { max-width: 800px; margin: 0 auto; padding: 60px 24px; text-align: center; }
.lp-showcase-screens.lp-visible { animation: lpFadeUp .5s ease-out forwards; }
.lp-mockup {
  background: rgba(255,255,255,.03); border: 1px solid rgba(255,255,255,.08); border-radius: 10px;
  overflow: hidden; text-align: left;
}
.lp-mockup-bar {
  display: flex; align-items: center; gap: 10px; padding: 10px 14px;
  background: rgba(255,255,255,.04); border-bottom: 1px solid rgba(255,255,255,.06);
}
.lp-mockup-dots { display: flex; gap: 5px; }
.lp-mockup-dots span { width: 8px; height: 8px; border-radius: 50%; background: rgba(255,255,255,.15); }
.lp-mockup-title { font-size: 11px; color: #A1A1AA; }
.lp-mockup-body { display: flex; gap: 12px; padding: 14px; }
@media (max-width: 500px) { .lp-mockup-body { flex-direction: column; } }
.lp-mock-col { flex: 1; display: flex; flex-direction: column; gap: 8px; }
.lp-mock-card {
  padding: 10px 12px; border-radius: 6px; background: rgba(255,255,255,.04);
  border: 1px solid rgba(255,255,255,.05);
}
.lp-mock-label { font-size: 10px; color: #A1A1AA; text-transform: uppercase; letter-spacing: .04em; margin-bottom: 4px; }
.lp-mock-val { font-size: 14px; font-weight: 700; color: #E4E4E7; }
.lp-mock-bar { height: 3px; border-radius: 2px; background: linear-gradient(90deg,var(--accent),transparent); margin-top: 6px; }
.lp-mock-graph { padding: 8px; border-radius: 6px; background: rgba(255,255,255,.04); border: 1px solid rgba(255,255,255,.05); height: 60px; }
.lp-mock-temp { display: flex; justify-content: center; gap: 16px; font-size: 10px; color: #A1A1AA; }

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
[data-observe] { transition: opacity .6s ease-out, translate .6s ease-out; }
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