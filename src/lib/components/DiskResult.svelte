<script lang="ts">
  let { score = 0, label = '' }: { score: number; label: string } = $props()

  const iops = Math.max(10, Math.min(500, Math.round(score / 20000)))
  const readSpd = Math.max(100, Math.min(7000, Math.round(score / 1000 * 3)))
  const writeSpd = Math.max(80, Math.min(5000, Math.round(score / 1000 * 2.5)))
</script>

<div class="result-disk">
  <div class="rd-header">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="width:16px;height:16px">
      <ellipse cx="12" cy="12" rx="10" ry="4"/><path d="M4 12v5c0 2.2 3.6 4 8 4s8-1.8 8-4v-5"/>
    </svg>
    <span>Disk Throughput</span>
  </div>
  <div class="rd-body">
    <div class="rd-chart">
      <div class="rd-bar-group">
        <span class="rd-bar-label">R</span>
        <div class="rd-bar-track"><div class="rd-bar read" style="width:{Math.min(readSpd / 7000 * 100,100)}%"></div></div>
        <span class="rd-bar-val">{readSpd} MB/s</span>
      </div>
      <div class="rd-bar-group">
        <span class="rd-bar-label">W</span>
        <div class="rd-bar-track"><div class="rd-bar write" style="width:{Math.min(writeSpd / 5000 * 100,100)}%"></div></div>
        <span class="rd-bar-val">{writeSpd} MB/s</span>
      </div>
      <div class="rd-metric">
        <span class="rd-big">{iops}K</span>
        <span class="rd-unit">IOPS</span>
      </div>
    </div>
    <div class="rd-info">
      <div class="rd-stat"><span class="rd-label">Raw Score</span><span class="rd-val">{label}</span></div>
      <div class="rd-stat"><span class="rd-label">Type</span><span class="rd-val">NVMe SSD</span></div>
      <div class="rd-stat"><span class="rd-label">Access</span><span class="rd-val">Random 4K</span></div>
    </div>
  </div>
</div>

<style>
  .result-disk {
    border: 1px solid var(--border); border-radius: var(--radius);
    overflow: hidden; margin-bottom: 4px;
  }
  .rd-header {
    display: flex; align-items: center; gap: 6px; padding: 8px 10px;
    background: var(--bg-tertiary); font-size: 11px; font-weight: 600;
  }
  .rd-body {
    display: flex; gap: 12px; padding: 12px; align-items: center;
    background: var(--bg-primary);
  }
  .rd-chart { flex: 1; display: flex; flex-direction: column; gap: 6px; min-width: 140px; }
  .rd-bar-group { display: grid; grid-template-columns: 14px 1fr 50px; gap: 6px; align-items: center; }
  .rd-bar-label { font-size: 10px; color: var(--text-muted); font-weight: 600; text-align: right; }
  .rd-bar-track { height: 14px; background: var(--bg-secondary); border-radius: 7px; overflow: hidden; }
  .rd-bar { height: 100%; border-radius: 7px; transition: width 0.5s ease; }
  .rd-bar.read { background: linear-gradient(90deg, #818CF8, #6366F1); }
  .rd-bar.write { background: linear-gradient(90deg, #34D399, #10B981); }
  .rd-bar-val { font-size: 11px; font-weight: 500; font-variant-numeric: tabular-nums; }
  .rd-metric { text-align: center; padding: 6px 0 2px; }
  .rd-big { font-size: 22px; font-weight: 700; color: var(--accent); }
  .rd-unit { font-size: 9px; color: var(--text-muted); margin-left: 4px; }
  .rd-info { display: flex; flex-direction: column; gap: 2px; min-width: 90px; }
  .rd-stat { display: flex; justify-content: space-between; font-size: 10px; }
  .rd-label { color: var(--text-muted); }
  .rd-val { color: var(--text-primary); font-weight: 500; }
</style>
