# PAUGERAN

PAUGERAN adalah Supreme Legal Reasoning Engine untuk analisis hukum Indonesia yang dapat dilacak.

## Status implementasi

Vertical slice awal tersedia:

- Dashboard perkara SolidJS dengan sidebar, chat, mode penalaran, dan case graph.
- Tipe domain bersama untuk `CaseState`, `ReasoningMode`, dan `CaseSummary`.
- Server Axum dengan `GET /health` dan `GET /api/cases`.
- SQLite persistence untuk perkara dan pesan analisis.
- Case State Machine dan Reasoning Mode router.
- Endpoint perubahan state/mode dan analisis pesan terstruktur.
- Workspace pnpm dan Cargo sebagai fondasi single-binary.

## Pengembangan

```bash
pnpm install
pnpm --filter @paugeran/web dev
cargo run --manifest-path apps/server/Cargo.toml
```

Rust 1.75+ dan Node.js 20+ diperlukan. Aplikasi web tersedia di `http://localhost:5173`; server API di `http://localhost:3000`.

## API inti

- `GET /health`
- `GET /api/cases`
- `POST /api/cases` dengan `{ "title": "..." }`
- `PATCH /api/cases/:id/state` dengan `{ "state": "exploration", "reason": "..." }`
- `PATCH /api/cases/:id/mode` dengan `{ "mode": "adversarial" }`
- `POST /api/cases/:id/messages` dengan `{ "content": "..." }`

Respons analisis menyertakan `certaintyScore`, `factors`, dan `clarifyingQuestions` sesuai mode aktif.
