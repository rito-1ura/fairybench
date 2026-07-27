// Online Leaderboard API for FairyBench
// Vercel Serverless Function (Node.js)

const KV_PREFIX = 'fairybench_lb_'
let inMemoryStore = []

export default async function handler(req, res) {
  res.setHeader('Access-Control-Allow-Origin', '*')
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type')

  if (req.method === 'OPTIONS') return res.status(204).end()

  try {
    let entries = inMemoryStore

    if (req.method === 'GET') {
      const limit = Math.min(parseInt(req.query.limit || '50'), 100)
      const sorted = [...entries].sort((a, b) => b.score - a.score).slice(0, limit)
      return res.status(200).json({ ok: true, entries: sorted, total: entries.length })
    }

    if (req.method === 'POST') {
      const { score, run_id, cpu_name, gpu_name, memory_gb, version } = req.body
      if (!score || !run_id) {
        return res.status(400).json({ ok: false, error: 'score and run_id required' })
      }

      const entry = {
        score, run_id,
        cpu_name: cpu_name || '',
        gpu_name: gpu_name || '',
        memory_gb: memory_gb || 0,
        timestamp: new Date().toISOString(),
        version: version || '0.0.0',
      }

      entries.push(entry)
      inMemoryStore = entries

      const rank = entries.filter(e => e.score > entry.score).length + 1
      return res.status(200).json({ ok: true, rank, total: entries.length })
    }

    return res.status(405).json({ ok: false, error: 'Method not allowed' })
  } catch (err) {
    return res.status(500).json({ ok: false, error: String(err) })
  }
}
