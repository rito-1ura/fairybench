<script lang="ts">
  let { running = true }: { running?: boolean } = $props()
  let iops = $state(185000)
  let readSpeed = $state(3200)
  let writeSpeed = $state(2800)
  let intervalId: ReturnType<typeof setInterval> | undefined

  $effect(() => {
    if (!running) { clearInterval(intervalId); return }
    intervalId = setInterval(() => {
      iops = Math.max(120000, Math.min(350000, iops + (Math.random() - 0.5) * 12000))
      readSpeed = Math.max(800, Math.min(5500, readSpeed + (Math.random() - 0.5) * 200))
      writeSpeed = Math.max(600, Math.min(4500, writeSpeed + (Math.random() - 0.5) * 180))
    }, 200)
    return () => clearInterval(intervalId)
  })
</script>

<div class="disk-container">
  <div class="disk-vis">
    <div class="disk-bar-group">
      <div class="disk-label">Read</div>
      <div class="disk-track">
        <div class="disk-fill read-fill" style="width: {Math.min(readSpeed / 5500 * 100, 100)}%" />
      </div>
      <div class="disk-val">{readSpeed.toFixed(0)} MB/s</div>
    </div>
    <div class="disk-bar-group">
      <div class="disk-label">Write</div>
      <div class="disk-track">
        <div class="disk-fill write-fill" style="width: {Math.min(writeSpeed / 4500 * 100, 100)}%" />
      </div>
      <div class="disk-val">{writeSpeed.toFixed(0)} MB/s</div>
    </div>
    <div class="disk-divider" />
    <div class="disk-metrics">
      <div class="metric">
        <div class="metric-val">{(iops / 1000).toFixed(1)}K</div>
        <div class="metric-label">IOPS</div>
      </div>
      <div class="metric">
        <div class="metric-val">{(readSpeed + writeSpeed).toFixed(0)}</div>
        <div class="metric-label">Total MB/s</div>
      </div>
    </div>
  </div>
  <div class="disk-details">
    <div class="info-row"><span>Drive</span><span>NVMe SSD</span></div>
    <div class="info-row"><span>Queue Depth</span><span>32</span></div>
    <div class="info-row"><span>Block Size</span><span>4KB (Random)</span></div>
    <div class="info-row"><span>Latency</span><span>{((Math.random() * 0.3 + 0.05)).toFixed(2)} ms</span></div>
    <div class="info-row"><span>Fragmentation</span><span>{(Math.random() * 8).toFixed(1)}%</span></div>
  </div>
</div>

<style>
  .disk-container {
    display: flex; gap: 16px; align-items: stretch; justify-content: center;
    padding: 20px; background: var(--bg-primary); border-radius: var(--radius-lg);
    border: 1px solid var(--border); flex-wrap: wrap;
  }
  .disk-vis {
    display: flex; flex-direction: column; gap: 10px; min-width: 200px;
  }
  .disk-bar-group {
    display: grid; grid-template-columns: 36px 1fr 50px; gap: 8px;
    align-items: center;
  }
  .disk-label { font-size: 10px; color: var(--text-muted); font-weight: 500; text-align: right; }
  .disk-track { height: 18px; background: var(--bg-secondary); border-radius: 9px; overflow: hidden; }
  .disk-fill { height: 100%; border-radius: 9px; transition: width 0.15s ease; }
  .read-fill { background: linear-gradient(90deg, #818CF8, #6366F1); }
  .write-fill { background: linear-gradient(90deg, #34D399, #10B981); }
  .disk-val { font-size: 12px; font-weight: 600; font-variant-numeric: tabular-nums; color: var(--text-primary); }
  .disk-divider { height: 1px; background: var(--border); margin: 4px 0; }
  .disk-metrics { display: flex; gap: 16px; justify-content: center; }
  .metric { text-align: center; }
  .metric-val { font-size: 18px; font-weight: 700; color: var(--accent); }
  .metric-label { font-size: 9px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0; margin-top: 2px; }
  .disk-details {
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
