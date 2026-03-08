# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 2418 LoC |
| **Peso Total del Proyecto** | 296.74KB |
| **Estado de Sintonía** | Activa |

### Mapa de Arquitectura y Pesos
```text
├── .cursorrules-back.md (0 LoC | 3.69KB)
├── .cursorrules.md (0 LoC | 2.46KB)
├── .env.example (0 LoC | 504.00B)
├── .pre-commit-config.yaml (36 LoC | 1.05KB)
├── .sqlx/ [3.17KB]
│   ├── query-d210c01bfa9798e7ce178f0ae12077cac8203e6607883a8dffed40f3e70977c5.json (0 LoC | 1.19KB)
│   ├── query-d82cf1da92d2cb100f72bac4f77238c964d4aabeb70b26ea55c5b89f620787d1.json (0 LoC | 1.19KB)
│   └── query-e20e8388eb12c09c1121cf86d730e498226a5de22f49e9b3e0f921e3442b728c.json (0 LoC | 815.00B)
├── Cargo.lock (0 LoC | 61.63KB)
├── Cargo.toml (26 LoC | 1005.00B)
├── Justfile (0 LoC | 1.69KB)
├── README.md (0 LoC | 3.19KB)
├── apps/ [0.00B]
│   └── frontend_astro/ [0.00B]
├── biome.json (0 LoC | 413.00B)
├── crates/ [84.03KB]
│   ├── api_server/ [33.95KB]
│   │   ├── Cargo.toml (37 LoC | 1.45KB)
│   │   ├── src/ [21.88KB]
│   │   │   ├── config/ [3.81KB]
│   │   │   │   ├── di.rs (57 LoC | 2.94KB)
│   │   │   │   ├── env.rs (21 LoC | 792.00B)
│   │   │   │   └── mod.rs (4 LoC | 100.00B)
│   │   │   ├── entry_points/ [10.48KB]
│   │   │   │   ├── api/ [7.45KB]
│   │   │   │   │   ├── mod.rs (3 LoC | 104.00B)
│   │   │   │   │   └── v1/ [7.35KB]
│   │   │   │   │       ├── errors.rs (57 LoC | 2.54KB)
│   │   │   │   │       ├── mod.rs (12 LoC | 559.00B)
│   │   │   │   │       └── user_handlers.rs (131 LoC | 4.26KB)
│   │   │   │   ├── auth.rs (80 LoC | 2.93KB)
│   │   │   │   └── mod.rs (4 LoC | 107.00B)
│   │   │   ├── errors.rs (41 LoC | 2.00KB)
│   │   │   ├── lib.rs (20 LoC | 886.00B)
│   │   │   ├── main.rs (97 LoC | 4.03KB)
│   │   │   └── routes.rs (20 LoC | 705.00B)
│   │   └── tests/ [10.62KB]
│   │       └── integration_tests.rs (281 LoC | 10.62KB)
│   ├── core_logic/ [26.79KB]
│   │   ├── Cargo.toml (28 LoC | 1022.00B)
│   │   └── src/ [25.80KB]
│   │       ├── application/ [8.80KB]
│   │       │   ├── mod.rs (14 LoC | 577.00B)
│   │       │   └── use_cases/ [8.23KB]
│   │       │       ├── mod.rs (9 LoC | 371.00B)
│   │       │       └── user/ [7.87KB]
│   │       │           ├── create_session.rs (47 LoC | 1.62KB)
│   │       │           ├── get_user_by_id.rs (27 LoC | 963.00B)
│   │       │           ├── login.rs (58 LoC | 2.07KB)
│   │       │           ├── mod.rs (13 LoC | 467.00B)
│   │       │           └── register.rs (70 LoC | 2.79KB)
│   │       ├── domain/ [16.58KB]
│   │       │   ├── entities/ [4.21KB]
│   │       │   │   ├── mod.rs (17 LoC | 612.00B)
│   │       │   │   ├── session.rs (17 LoC | 488.00B)
│   │       │   │   └── user.rs (99 LoC | 3.13KB)
│   │       │   ├── errors.rs (29 LoC | 1.06KB)
│   │       │   ├── interfaces/ [4.04KB]
│   │       │   │   ├── hasher.rs (24 LoC | 1.14KB)
│   │       │   │   ├── mod.rs (20 LoC | 816.00B)
│   │       │   │   ├── session_repo.rs (16 LoC | 694.00B)
│   │       │   │   └── user_repo.rs (32 LoC | 1.43KB)
│   │       │   ├── mod.rs (16 LoC | 679.00B)
│   │       │   └── value_objects/ [6.60KB]
│   │       │       ├── email.rs (50 LoC | 1.84KB)
│   │       │       ├── mod.rs (19 LoC | 855.00B)
│   │       │       ├── password_hash.rs (36 LoC | 1.29KB)
│   │       │       ├── session_token.rs (31 LoC | 1.05KB)
│   │       │       └── user_id.rs (45 LoC | 1.59KB)
│   │       └── lib.rs (11 LoC | 430.00B)
│   └── infra_db/ [23.29KB]
│       ├── Cargo.toml (29 LoC | 1.24KB)
│       ├── migrations/ [5.04KB]
│       │   ├── 20260305135148_create_users_table.sql (43 LoC | 2.15KB)
│       │   ├── 20260305135149_create_rbac.sql (19 LoC | 643.00B)
│       │   ├── 20260305135150_create_tokens.sql (11 LoC | 420.00B)
│       │   ├── 20260305135151_create_audit.sql (14 LoC | 530.00B)
│       │   ├── 20260305135152_seed_system_data.sql (9 LoC | 512.00B)
│       │   └── 20260305135153_create_sessions.sql (21 LoC | 852.00B)
│       └── src/ [17.01KB]
│           ├── external_services/ [2.92KB]
│           │   ├── hashing.rs (56 LoC | 2.43KB)
│           │   └── mod.rs (12 LoC | 502.00B)
│           ├── lib.rs (19 LoC | 1.13KB)
│           └── persistence/ [12.96KB]
│               ├── mod.rs (13 LoC | 537.00B)
│               └── sqlite/ [12.44KB]
│                   ├── mod.rs (16 LoC | 736.00B)
│                   ├── models.rs (61 LoC | 2.45KB)
│                   └── repositories/ [9.27KB]
│                       ├── mod.rs (15 LoC | 639.00B)
│                       ├── sqlite_session_repo.rs (112 LoC | 4.09KB)
│                       └── sqlite_user_repo.rs (117 LoC | 4.55KB)
├── deploy/ [4.21KB]
│   ├── Caddyfile (0 LoC | 785.00B)
│   ├── Dockerfile (0 LoC | 1.84KB)
│   └── podman-compose.yml (53 LoC | 1.61KB)
├── docs/ [105.65KB]
│   ├── DECISIONS.md (0 LoC | 17.30KB)
│   ├── RUST_STANDARDS.md (0 LoC | 3.56KB)
│   ├── TESTING.md (0 LoC | 4.67KB)
│   ├── TROUBLESHOOTING.md (0 LoC | 16.80KB)
│   ├── core-iu.md (0 LoC | 5.28KB)
│   ├── docs-fases/ [53.44KB]
│   │   ├── docs-backend.md (0 LoC | 5.61KB)
│   │   ├── docs-backend1.md (0 LoC | 33.01KB)
│   │   ├── docs-frontend.md (0 LoC | 0.00B)
│   │   └── docs-genesis.md (0 LoC | 14.82KB)
│   ├── promps-front.md (0 LoC | 1.78KB)
│   └── promps.md (0 LoC | 2.82KB)
├── proto/ [2.24KB]
│   ├── auth/ [1.05KB]
│   │   └── v1/ [1.05KB]
│   │       └── auth.proto (0 LoC | 1.05KB)
│   ├── buf.gen.yaml (14 LoC | 662.00B)
│   ├── buf.yaml (8 LoC | 181.00B)
│   └── common/ [373.00B]
│       └── v1/ [373.00B]
│           └── common.proto (0 LoC | 373.00B)
├── roadmaps/ [16.30KB]
│   ├── backend.md (0 LoC | 8.83KB)
│   ├── frontend.md (0 LoC | 3.49KB)
│   ├── genesis.md (0 LoC | 3.39KB)
│   └── master.md (0 LoC | 605.00B)
├── test-api.bat (0 LoC | 649.00B)
├── test-api.sh (0 LoC | 663.00B)
└── ver-proyecto.py (121 LoC | 4.26KB)
```
