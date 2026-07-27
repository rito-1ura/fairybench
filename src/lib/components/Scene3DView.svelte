<script lang="ts">
  let { running = true }: { running?: boolean } = $props()
  let fps = $state(60)
  let rotation = $state(0)
  let intervalId: ReturnType<typeof setInterval> | undefined

  $effect(() => {
    if (!running) { clearInterval(intervalId); return }
    intervalId = setInterval(() => {
      rotation = (rotation + 0.8) % 360
      fps = Math.max(30, Math.min(165, fps + (Math.random() - 0.5) * 8))
    }, 33)
    return () => clearInterval(intervalId)
  })
</script>

<div class="scene-container">
  <div class="scene-viewport">
    <div class="scene-3d" style="transform: rotateY({rotation}deg) rotateX({rotation * 0.3}deg)">
      <div class="face front" />
      <div class="face back" />
      <div class="face top" />
      <div class="face bottom" />
      <div class="face left" />
      <div class="face right" />
    </div>
    <div class="fps-counter">{fps.toFixed(0)} FPS</div>
  </div>
  <div class="scene-info">
    <div class="info-row"><span>Resolution</span><span>1920×1080</span></div>
    <div class="info-row"><span>Scene</span><span>Complex Mesh</span></div>
    <div class="info-row"><span>Shaders</span><span>PBR + PostFX</span></div>
    <div class="info-row"><span>Draw Calls</span><span>12,847</span></div>
    <div class="info-row"><span>VRAM Used</span><span>2.4 GB</span></div>
  </div>
</div>

<style>
  .scene-container {
    display: flex; gap: 16px; align-items: center; justify-content: center;
    padding: 20px; background: var(--bg-primary); border-radius: var(--radius-lg);
    border: 1px solid var(--border); flex-wrap: wrap;
  }
  .scene-viewport {
    position: relative; width: 280px; height: 200px;
    background: radial-gradient(ellipse at center, #141418 0%, #08080A 100%);
    border-radius: var(--radius); overflow: hidden; display: flex;
    align-items: center; justify-content: center;
  }
  .scene-3d {
    width: 120px; height: 120px; position: relative; transform-style: preserve-3d;
    transition: transform 0.05s linear;
  }
  .face {
    position: absolute; width: 120px; height: 120px;
    border: 1.5px solid rgba(129,140,248,0.5);
    background: rgba(129,140,248,0.06);
  }
  .front  { transform: translateZ(60px); }
  .back   { transform: rotateY(180deg) translateZ(60px); }
  .top    { transform: rotateX(90deg) translateZ(60px); }
  .bottom { transform: rotateX(-90deg) translateZ(60px); }
  .left   { transform: rotateY(-90deg) translateZ(60px); }
  .right  { transform: rotateY(90deg) translateZ(60px); }
  .fps-counter {
    position: absolute; bottom: 8px; right: 10px;
    font-size: 13px; font-weight: 700; color: var(--green);
    text-shadow: 0 0 10px rgba(52,211,153,0.4);
    font-variant-numeric: tabular-nums;
  }
  .scene-info {
    display: flex; flex-direction: column; gap: 3px; min-width: 140px;
  }
  .info-row {
    display: flex; justify-content: space-between; gap: 12px;
    font-size: 11px; padding: 4px 8px;
    background: var(--bg-secondary); border-radius: 4px;
  }
  .info-row span:first-child { color: var(--text-muted); }
  .info-row span:last-child { color: var(--text-primary); font-weight: 500; }
</style>
