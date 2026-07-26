<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'

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

  // A/B compare state
  let compareMode = $state(false)
  let selectedRuns = $state<SavedRun[]>([])
  let compareDetail = $state<[RunResult|null, RunResult|null]>([null, null])

  function toggleCompare(run: SavedRun) {
    const idx = selectedRuns.findIndex(r => r.run_id === run.run_id)
    if (idx >= 0) {
      selectedRuns.splice(idx, 1)
    } else {
      if (selectedRuns.length >= 2) selectedRuns.shift()
      selectedRuns = [...selectedRuns, run]
    }
    if (selectedRuns.length === 2) loadCompareDetails()
  }

  async function loadCompareDetails() {
    try {
      const [a, b] = await Promise.all([
        invoke<RunResult|null>('get_run_detail', { runId: selectedRuns[0].run_id }),
        invoke<RunResult|null>('get_run_detail', { runId: selectedRuns[1].run_id }),
      ])
      compareDetail = [a, b]
    } catch {}
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
    <span class="panel-title">
      {#if compareMode}
        <button class="btn btn-xs btn-outline" onclick={() => { compareMode = false; selectedRuns = []; compareDetail = [null, null] }}>
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" style="width:12px;height:12px"><path d="M10 12l-4-4 4-4"/></svg>
          戻る
        </button>
        A/B 比較
      {:else}
        統計分析
      {/if}
    </span>
    <span class="panel-badge">{result ? '95% CI' : history.length > 0 ? `${history.length} runs` : ''}</span>
  </div>
  <div class="panel-body">

    {#if compareMode && selectedRuns.length === 2 && compareDetail[0] && compareDetail[1]}
      <!-- A/B Comparison -->
      <div class="ab-grid">
        <div class="ab-col">
          <div class="ab-header">{formatLocalDate(selectedRuns[0].executed_at)}</div>
          <div class="score-value compare">{formatScore(selectedRuns[0].overall_raw)}</div>
          {#each Object.entries(compareDetail[0].sub_scores) as [name, sub]}
            <div class="ab-row">
              <span class="ab-name">{name}</span>
              <span class="ab-val">{formatScore(sub.raw_score)}</span>
            </div>
          {/each}
        </div>
        <div class="ab-vs">
          <span class="vs-text">VS</span>
          {#if selectedRuns[0].overall_raw !== selectedRuns[1].overall_raw}
            <span class="vs-delta" class:green={selectedRuns[0].overall_raw > selectedRuns[1].overall_raw} class:red={selectedRuns[0].overall_raw < selectedRuns[1].overall_raw}>
              {((selectedRuns[0].overall_raw / selectedRuns[1].overall_raw - 1) * 100).toFixed(1)}%
            </span>
          {/if}
        </div>
        <div class="ab-col">
          <div class="ab-header">{formatLocalDate(selectedRuns[1].executed_at)}</div>
          <div class="score-value compare">{formatScore(selectedRuns[1].overall_raw)}</div>
          {#each Object.entries(compareDetail[1].sub_scores) as [name, sub]}
            <div class="ab-row">
              <span class="ab-name">{name}</span>
              <span class="ab-val">{formatScore(sub.raw_score)}</span>
            </div>
          {/each}
        </div>
      </div>

    {:else}
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
                    fill="var(--accent)" fill-opacity="0.5"
                    onmouseenter={() => {}}
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
        <div class="history-title">
          <span>実行履歴</span>
          {#if history.length >= 2}
            <button class="btn btn-xs btn-outline" onclick={() => { compareMode = true; selectedRuns = [] }}>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width:12px;height:12px">
                <path d="M5 12h14M12 5l-7 7 7 7"/>
              </svg>
              A/B比較
            </button>
          {/if}
        </div>
        {#if history.length === 0}
          <div class="empty-state">まだ実行結果がありません</div>
        {:else}
          <div class="history-list">
            {#each history as run, i}
              <div class="history-item" class:selected={selectedRuns.find(r => r.run_id === run.run_id)}
                   onclick={() => compareMode ? toggleCompare(run) : null}
                   onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); if (compareMode) toggleCompare(run) } }}
                   role="button" tabindex="-1"
              >
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
    {/if}
  </div>
</div>

<style>
  .stat-details { display: flex; flex-direction: column; gap: 8px; }
  .stat-row { display: flex; justify-content: space-between; align-items: center; font-size: 12px; }
  .stat-row span { color: var(--text-muted); }
  .stat-row strong { color: var(--text-primary); font-weight: 500; }

  .divider { height: 1px; background: var(--bg-tertiary); margin: 12px 0; }

  .chart-section { margin: 4px 0 8px; }
  .chart-title { font-size: 11px; font-weight: 500; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 8px; }
  .chart-svg { display: block; }
  .chart-bar { transition: height 0.3s; }
  .chart-bar:hover { fill-opacity: 0.8; }
  .chart-line { filter: drop-shadow(0 0 4px rgba(129,140,248,0.4)); }
  .chart-dot { cursor: pointer; transition: r 0.15s; }
  .chart-dot:hover { r: 5; }

  .history-title { display: flex; align-items: center; justify-content: space-between; font-size: 11px; font-weight: 500; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 8px; }
  .empty-state { font-size: 12px; color: var(--text-muted); padding: 12px 0; text-align: center; }
  .history-list { display: flex; flex-direction: column; gap: 4px; max-height: 180px; overflow-y: auto; }
  .history-item {
    display: flex; align-items: center; justify-content: space-between;
    padding: 6px 8px; border-radius: 5px; background: var(--bg-primary);
    border: 1px solid var(--border); cursor: default; transition: border-color 0.15s, background 0.15s;
  }
  .history-item.selected { border-color: var(--accent); background: rgba(129,140,248,0.06); }
  .history-item[role=button] { cursor: pointer; }
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

  /* A/B Compare */
  .ab-grid { display: grid; grid-template-columns: 1fr auto 1fr; gap: 12px; }
  .ab-col { display: flex; flex-direction: column; gap: 4px; }
  .ab-header { font-size: 10px; color: var(--text-muted); text-align: center; font-weight: 500; }
  .ab-col .score-value.compare { font-size: 18px; font-weight: 700; text-align: center; padding: 8px 0; }
  .ab-row { display: flex; justify-content: space-between; font-size: 11px; padding: 3px 4px; border-radius: 3px; background: var(--bg-primary); }
  .ab-name { color: var(--text-muted); }
  .ab-val { color: var(--text-primary); font-weight: 500; }
  .ab-vs { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 4px; }
  .vs-text { font-size: 11px; font-weight: 600; color: var(--text-secondary); }
  .vs-delta { font-size: 13px; font-weight: 700; }
  .vs-delta.green { color: var(--green); }
  .vs-delta.red { color: var(--red); }
</style>
