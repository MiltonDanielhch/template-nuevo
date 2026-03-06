# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 1430 LoC |
| **Peso Total del Proyecto** | 221.43KB |
| **Estado de Sintonía** | Activa |

### Mapa de Arquitectura y Pesos
```text
├── .cursorrules.md (0 LoC | 3.69KB)
├── .env.example (0 LoC | 452.00B)
├── .pre-commit-config.yaml (36 LoC | 1.05KB)
├── .sqlx/ [3.17KB]
│   ├── query-d210c01bfa9798e7ce178f0ae12077cac8203e6607883a8dffed40f3e70977c5.json (0 LoC | 1.19KB)
│   ├── query-d82cf1da92d2cb100f72bac4f77238c964d4aabeb70b26ea55c5b89f620787d1.json (0 LoC | 1.19KB)
│   └── query-e20e8388eb12c09c1121cf86d730e498226a5de22f49e9b3e0f921e3442b728c.json (0 LoC | 815.00B)
├── Cargo.lock (0 LoC | 69.16KB)
├── Cargo.toml (26 LoC | 1005.00B)
├── Justfile (0 LoC | 1.69KB)
├── biome.json (0 LoC | 413.00B)
├── crates/ [50.81KB]
│   ├── api_server/ [16.63KB]
│   │   ├── Cargo.toml (36 LoC | 1.54KB)
│   │   ├── src/ [10.32KB]
│   │   │   ├── config/ [2.48KB]
│   │   │   │   └── di.rs (46 LoC | 2.48KB)
│   │   │   ├── entry_points/ [4.74KB]
│   │   │   │   └── api/ [4.74KB]
│   │   │   │       └── v1/ [4.74KB]
│   │   │   │           ├── errors.rs (49 LoC | 2.04KB)
│   │   │   │           ├── mod.rs (12 LoC | 557.00B)
│   │   │   │           └── user_handlers.rs (56 LoC | 2.15KB)
│   │   │   ├── lib.rs (20 LoC | 866.00B)
│   │   │   ├── main.rs (42 LoC | 1.81KB)
│   │   │   └── routes.rs (13 LoC | 451.00B)
│   │   └── test/ [4.77KB]
│   │       ├── integration_tests.rs (100 LoC | 4.67KB)
│   │       └── mod.rs (3 LoC | 101.00B)
│   ├── core_logic/ [18.71KB]
│   │   ├── Cargo.toml (29 LoC | 1.09KB)
│   │   └── src/ [17.63KB]
│   │       ├── application/ [3.73KB]
│   │       │   ├── mod.rs (14 LoC | 577.00B)
│   │       │   └── use_cases/ [3.17KB]
│   │       │       ├── mod.rs (9 LoC | 371.00B)
│   │       │       └── user/ [2.81KB]
│   │       │           ├── mod.rs (6 LoC | 269.00B)
│   │       │           └── register.rs (65 LoC | 2.54KB)
│   │       ├── domain/ [13.47KB]
│   │       │   ├── entities/ [3.67KB]
│   │       │   │   ├── mod.rs (14 LoC | 548.00B)
│   │       │   │   └── user.rs (99 LoC | 3.13KB)
│   │       │   ├── errors.rs (21 LoC | 820.00B)
│   │       │   ├── interfaces/ [3.29KB]
│   │       │   │   ├── hasher.rs (24 LoC | 1.14KB)
│   │       │   │   ├── mod.rs (18 LoC | 746.00B)
│   │       │   │   └── user_repo.rs (32 LoC | 1.43KB)
│   │       │   ├── mod.rs (16 LoC | 679.00B)
│   │       │   └── value_objects/ [5.05KB]
│   │       │       ├── email.rs (41 LoC | 1.59KB)
│   │       │       ├── mod.rs (17 LoC | 795.00B)
│   │       │       ├── password_hash.rs (36 LoC | 1.25KB)
│   │       │       └── user_id.rs (39 LoC | 1.43KB)
│   │       └── lib.rs (11 LoC | 430.00B)
│   └── infra_db/ [15.46KB]
│       ├── Cargo.toml (29 LoC | 1.25KB)
│       ├── migrations/ [1.78KB]
│       │   └── 20260305135148_create_users_table.sql (34 LoC | 1.78KB)
│       └── src/ [12.43KB]
│           ├── external_services/ [2.92KB]
│           │   ├── hashing.rs (56 LoC | 2.43KB)
│           │   └── mod.rs (12 LoC | 502.00B)
│           ├── lib.rs (19 LoC | 1.12KB)
│           └── persistence/ [8.39KB]
│               ├── mod.rs (13 LoC | 537.00B)
│               └── sqlite/ [7.86KB]
│                   ├── mod.rs (16 LoC | 736.00B)
│                   ├── models.rs (49 LoC | 2.10KB)
│                   └── repositories/ [5.05KB]
│                       ├── mod.rs (12 LoC | 507.00B)
│                       └── sqlite_user_repo.rs (117 LoC | 4.55KB)
├── deploy/ [1.93KB]
│   └── Caddyfile (0 LoC | 1.93KB)
├── docs/ [71.96KB]
│   ├── DECISIONS.md (0 LoC | 17.34KB)
│   ├── RUST_STANDARDS.md (0 LoC | 3.42KB)
│   ├── TROUBLESHOOTING.md (0 LoC | 6.80KB)
│   ├── core-iu.md (0 LoC | 5.28KB)
│   ├── docs-fases/ [38.01KB]
│   │   ├── docs-backend.md (0 LoC | 23.19KB)
│   │   └── docs-genesis.md (0 LoC | 14.82KB)
│   └── promps.md (0 LoC | 1.11KB)
├── proto/ [2.24KB]
│   ├── auth/ [1.05KB]
│   │   └── v1/ [1.05KB]
│   │       └── auth.proto (0 LoC | 1.05KB)
│   ├── buf.gen.yaml (14 LoC | 662.00B)
│   ├── buf.yaml (8 LoC | 181.00B)
│   └── common/ [373.00B]
│       └── v1/ [373.00B]
│           └── common.proto (0 LoC | 373.00B)
├── roadmaps/ [9.64KB]
│   ├── backend.md (0 LoC | 5.65KB)
│   ├── genesis.md (0 LoC | 3.39KB)
│   └── master.md (0 LoC | 605.00B)
└── ver-proyecto.py (121 LoC | 4.26KB)
```
