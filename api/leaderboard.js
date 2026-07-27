// Online Leaderboard API for FairyBench
// Deploy on Vercel: `vercel deploy`
// Requires: Vercel KV store (optional, falls back to in-memory)

const KV_PREFIX = 'fairybench_lb_'

let inMemoryStore: Entry[] | null = null

interface Entry {
  score: number;
  run_id: string;
  cpu_name: string;
  gpu_name: string;
  memory_gb: number;
  timestamp: string;
  version: string;
}

// Attempt to use Vercel KV
async function getKv(): Promise<any | null> {
  try {
    const { kv } = await import('@vercel/kv')
    return kv
  } catch {
    return null
  }
}

async function getEntries(): Promise<Entry[]> {
  const kv = await getKv()
  if (kv) {
    const data = await kv.get<Entry[]>(KV_PREFIX + 'entries')
    return data || []
  }
  return inMemoryStore || []
}

async function setEntries(entries: Entry[]): Promise<void> {
  const kv = await getKv()
  if (kv) {
    await kv.set(KV_PREFIX + 'entries', entries)
    return
  }
  inMemoryStore = entries
}

export default async function handler(req: Request): Promise<Response> {
  const headers = {
    'Content-Type': 'application/json',
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
    'Access-Control-Allow-Headers': 'Content-Type',
  }

  if (req.method === 'OPTIONS') {
    return new Response(null, { status: 204, headers })
  }

  try {
    if (req.method === 'GET') {
      const url = new URL(req.url)
      const limit = Math.min(parseInt(url.searchParams.get('limit') || '50'), 100)
      const entries = await getEntries()
      const sorted = entries.sort((a, b) => b.score - a.score).slice(0, limit)
      return new Response(JSON.stringify({ ok: true, entries: sorted, total: entries.length }), { headers })
    }

    if (req.method === 'POST') {
      const body = await req.json()
      if (!body.score || !body.run_id) {
        return new Response(JSON.stringify({ ok: false, error: 'score and run_id required' }), { status: 400, headers })
      }

      const entry: Entry = {
        score: body.score,
        run_id: body.run_id,
        cpu_name: body.cpu_name || '',
        gpu_name: body.gpu_name || '',
        memory_gb: body.memory_gb || 0,
        timestamp: new Date().toISOString(),
        version: body.version || '0.0.0',
      }

      const entries = await getEntries()
      entries.push(entry)
      await setEntries(entries)

      const rank = entries.filter(e => e.score > entry.score).length + 1
      return new Response(JSON.stringify({ ok: true, rank, total: entries.length }), { headers })
    }

    return new Response(JSON.stringify({ ok: false, error: 'Method not allowed' }), { status: 405, headers })
  } catch (err) {
    return new Response(JSON.stringify({ ok: false, error: String(err) }), { status: 500, headers })
  }
}
