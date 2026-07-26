<script lang="ts">
  import './app.css'
  import SimpleMode from './lib/SimpleMode.svelte'
  import DashboardMode from './lib/DashboardMode.svelte'
  import { invoke } from '@tauri-apps/api/core'

  let mode = $state<'simple' | 'dashboard'>('simple')
  let version = $state('')

  // 初期化時にバージョン取得
  $effect(() => {
    invoke<string>('get_version').then(v => version = v).catch(() => {})
  })
</script>

<div id="app">
  {#if mode === 'simple'}
    <SimpleMode onSwitch={() => mode = 'dashboard'} {version} />
  {:else}
    <DashboardMode onSwitch={() => mode = 'simple'} {version} />
  {/if}
</div>
