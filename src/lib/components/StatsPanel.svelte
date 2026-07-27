<script lang="ts">
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

  let { result, history, deleteRun, formatScore }: {
    result: RunResult | null
    history: SavedRun[]
    deleteRun: (id: string) => void
    formatScore: (v: number) => string
  } = $props()

  const formatLocalDate = (str: string): string => {
    const d = new Date(str);
    if (isNaN(d.getTime())) return str;
    const pad = (n: number) => String(n).padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth()+1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
  }

  // History chart — inline SVG chart with bars + line
  const maxHistory = 20
  const chartHeight = 80
  const chartWidth = 280

  function chartData(): { score: number; label: string; run_id: string }[] {
    const raw = [...history].slice(0, maxHistory)
    const max = Math.max(...raw.map(r => r.overall_raw), 1)
    return raw.map(r => ({ score: r.overall_raw / max, label: formatScore(r.overall_raw), run_id: r.run_id }))
  }
</script>

<div class="panel">
  <div class="panel-header">
    <span class="panel-title">統計分析</span>
    <span class="panel-badge">{result ? '95% CI' : history.length > 0 ? `${history.length} runs` : ''}</span>
  </div>
  <div class="panel-body">

    <!-- Current result -->
    {#if result}
      <div class="stat-details">
        <div class="stat-row"><span>最終スコア</span><strong class="gradient-text">{formatScore(result.overall_raw)}</strong></div>
        <div class="stat-row"><span>95% CI</span><strong>{formatScore(result.ci_lower)} – {formatScore(result.ci_upper)}</strong></div>
        <div class="stat-row"><span>CV</span><strong>{(result.cv * 100).toFixed(1)}%</strong></div>
        <div class="stat-row"><span>採用/除外</span><strong>{result.runs_used}/{result.runs_excluded}</strong></div>
      </div>
      <div class="divider"></div>
    {/if}

    <!-- History chart: SVG bars + line overlay -->
    {#if history.length > 1}
      <div class="chart-section">
        <div class="chart-title">スコア推移</div>
        <svg class="chart-svg" viewBox="0 0 {chartWidth} {chartHeight}" style="width:100%;height:{chartHeight}px">
          <!-- Bars -->
          {#each chartData() as item, i (item.run_id)}
            <rect class="chart-bar" x={(i / Math.max(chartData().length - 1, 1)) * chartWidth + 2}
                  y={chartHeight - item.score * chartHeight * 0.85}
                  width={Math.max(3, chartWidth / chartData().length - 4)}
                  height={item.score * chartHeight * 0.85}
                  rx="2" ry="2"
                  role="presentation"
                  fill="var(--accent)" fill-opacity="0.5"
            />
          {/each}
          <!-- Line overlay -->
          <polyline class="chart-line"
            points={chartData().map((item, i) =>
              `${(i / Math.max(chartData().length - 1, 1)) * chartWidth + chartWidth / (chartData().length * 2)},${chartHeight - item.score * chartHeight * 0.85}`
            ).join(' ')}
            fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
          />
          <!-- Dots on line -->
          {#each chartData() as item, i (item.run_id)}
            <circle class="chart-dot"
              cx={(i / Math.max(chartData().length - 1, 1)) * chartWidth + chartWidth / (chartData().length * 2)}
              cy={chartHeight - item.score * chartHeight * 0.85}
              r="3" fill="var(--accent)" stroke="var(--bg-primary)" stroke-width="1.5"
            />
          {/each}
        </svg>
      </div>
      <div class="divider"></div>
    {/if}

    <!-- History list -->
    <div class="history-section">
      <div class="history-title">実行履歴</div>
      {#if history.length === 0}
        <div class="empty-state">まだ実行結果がありません</div>
      {:else}
        <div class="history-list">
          {#each history as run, i}
            <div class="history-item">
              <div class="hist-left">
                <span class="hist-rank">#{i+1}</span>
                <span class="hist-score">{formatScore(run.overall_raw)}</span>
                <span class="hist-ci">±{formatScore((run.ci_upper - run.ci_lower) / 2)}</span>
              </div>
              <div class="hist-mid">
                <span class="hist-cv">CV: {(run.cv * 100).toFixed(1)}%</span>
                <span class="hist-date">{formatLocalDate(run.executed_at)}</span>
              </div>
              <button class="hist-del" onclick={(e) => { e.stopPropagation(); deleteRun(run.run_id) }} title="削除">
                <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" style="width:12px;height:12px"><path d="M4 4l8 8M12 4l-8 8"/></svg>
              </button>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .stat-details { display: flex; flex-direction: column; gap: 8px; }
  .stat-row { display: flex; justify-content: space-between; align-items: center; font-size: 12px; }
  .stat-row span { color: var(--text-muted); }
  .stat-row strong { color: var(--text-primary); font-weight: 500; }

  .divider { height: 1px; background: var(--bg-tertiary); margin: 12px 0; }

  .chart-section { margin: 4px 0 8px; }
  .chart-title { font-size: 11px; font-weight: 500; color: var(--text-muted); text-transform: uppercase; margin-bottom: 8px; }
  .chart-svg { display: block; }
  .chart-bar { transition: height 0.3s; }
  .chart-bar:hover { fill-opacity: 0.8; }
  .chart-line { filter: drop-shadow(0 0 4px rgba(129,140,248,0.4)); }
  .chart-dot { cursor: pointer; transition: r 0.15s; }
  .chart-dot:hover { r: 5; }

  .history-title { font-size: 11px; font-weight: 500; color: var(--text-muted); text-transform: uppercase; margin-bottom: 8px; }
  .empty-state { font-size: 12px; color: var(--text-muted); padding: 12px 0; text-align: center; }
  .history-list { display: flex; flex-direction: column; gap: 4px; max-height: 180px; overflow-y: auto; }
  .history-item {
    display: flex; align-items: center; justify-content: space-between;
    padding: 6px 8px; border-radius: 5px; background: var(--bg-primary);
    border: 1px solid var(--border); transition: border-color 0.15s;
  }
  .history-item:hover { border-color: var(--text-muted); }
  .hist-left { display: flex; align-items: baseline; gap: 6px; }
  .hist-rank { font-size: 9px; color: var(--text-muted); min-width: 18px; }
  .hist-score { font-size: 14px; font-weight: 600; color: var(--text-primary); }
  .hist-ci { font-size: 10px; color: var(--text-muted); }
  .hist-mid { display: flex; flex-direction: column; align-items: flex-end; gap: 1px; }
  .hist-cv { font-size: 10px; color: var(--text-secondary); }
  .hist-date { font-size: 9px; color: var(--text-muted); }
  .hist-del {
    background: none; border: none; color: var(--text-muted); cursor: pointer;
    font-size: 11px; padding: 2px 4px; border-radius: 3px; transition: all 0.15s;
  }
  .hist-del:hover { background: rgba(248,113,113,0.15); color: var(--red); }
</style>
