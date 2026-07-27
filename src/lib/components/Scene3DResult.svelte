<script lang="ts">
  let { score = 0, label = '' }: { score: number; label: string } = $props()

  // Derive FPS and quality tier from score
  const fps = Math.max(15, Math.min(165, Math.round(score / 1000)))
  const tier = fps >= 120 ? 'Ultra' : fps >= 60 ? 'High' : fps >= 30 ? 'Medium' : 'Low'
  const tColor = fps >= 120 ? '#34D399' : fps >= 60 ? '#818CF8' : fps >= 30 ? '#FBBF24' : '#F87171'
</script>

<div class="result-3d">
  <div class="r3d-header">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:16px;height:16px">
      <rect x="3" y="3" width="18" height="18" rx="2"/>
      <path d="M12 8v8M8 12h8"/>
    </svg>
    <span>3D Scene Render</span>
  </div>
  <div class="r3d-body">
    <div class="r3d-scene">
      <div class="r3d-cube">
        <div class="cube-face front" /><div class="cube-face back" />
        <div class="cube-face top" /><div class="cube-face bottom" />
        <div class="cube-face left" /><div class="cube-face right" />
      </div>
      <div class="r3d-fps" style="color:{tColor}">{fps} FPS</div>
    </div>
    <div class="r3d-stats">
      <div class="r3d-stat"><span class="r3d-label">Quality</span><span class="r3d-val" style="color:{tColor}">{tier}</span></div>
      <div class="r3d-stat"><span class="r3d-label">Raw Score</span><span class="r3d-val">{label}</span></div>
      <div class="r3d-stat"><span class="r3d-label">Resolution</span><span class="r3d-val">1080p</span></div>
      <div class="r3d-stat"><span class="r3d-label">API</span><span class="r3d-val">wgpu / Vulkan</span></div>
    </div>
  </div>
</div>

<style>
  .result-3d {
    border: 1px solid var(--border); border-radius: var(--radius);
    overflow: hidden; margin-bottom: 4px;
  }
  .r3d-header {
    display: flex; align-items: center; gap: 6px; padding: 8px 10px;
    background: var(--bg-tertiary); font-size: 11px; font-weight: 600;
  }
  .r3d-body {
    display: flex; gap: 12px; padding: 12px; align-items: center;
    background: var(--bg-primary);
  }
  .r3d-scene {
    position: relative; width: 100px; height: 80px;
    background: radial-gradient(ellipse at center, #141418, #08080A);
    border-radius: 6px; display: flex; align-items: center; justify-content: center;
    flex-shrink: 0;
  }
  .r3d-cube {
    width: 40px; height: 40px; position: relative; transform-style: preserve-3d;
    animation: spin 3s linear infinite;
  }
  @keyframes spin { to { transform: rotateY(360deg) rotateX(360deg); } }
  .cube-face {
    position: absolute; width: 40px; height: 40px;
    border: 1px solid rgba(129,140,248,0.5);
    background: rgba(129,140,248,0.05);
  }
  .front  { transform: translateZ(20px); }
  .back   { transform: rotateY(180deg) translateZ(20px); }
  .top    { transform: rotateX(90deg) translateZ(20px); }
  .bottom { transform: rotateX(-90deg) translateZ(20px); }
  .left   { transform: rotateY(-90deg) translateZ(20px); }
  .right  { transform: rotateY(90deg) translateZ(20px); }
  .r3d-fps {
    position: absolute; bottom: 4px; right: 6px;
    font-size: 13px; font-weight: 800;
    text-shadow: 0 0 8px rgba(0,0,0,0.8);
  }
  .r3d-stats { flex: 1; display: flex; flex-direction: column; gap: 2px; }
  .r3d-stat { display: flex; justify-content: space-between; font-size: 10px; }
  .r3d-label { color: var(--text-muted); }
  .r3d-val { color: var(--text-primary); font-weight: 500; }
</style>
