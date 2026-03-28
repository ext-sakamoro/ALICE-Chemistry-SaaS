# ALICE-Chemistry-SaaS

Molecular dynamics and computational chemistry API — simulate reactions, compute thermodynamic properties, and query element data via the ALICE SaaS architecture.

## Architecture

```
Client
  └─ API Gateway (:8142) — JWT auth, rate limiting, proxy
       └─ Core Engine (:9142) — MD simulation, reaction engine, element DB
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Health check |
| POST | /api/v1/chem/simulate | Run molecular dynamics simulation |
| POST | /api/v1/chem/reaction | Compute reaction pathway and energy |
| GET | /api/v1/chem/elements | Query periodic table element data |
| POST | /api/v1/chem/thermodynamics | Calculate thermodynamic properties |
| GET | /api/v1/chem/stats | Request statistics |

## Quick Start

```bash
cd services/core-engine && cargo run
cd services/api-gateway && cargo run
```

## License

AGPL-3.0-or-later
