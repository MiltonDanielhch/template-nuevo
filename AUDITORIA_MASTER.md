# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 2264 LoC |
| **Peso Total del Proyecto** | 248.73KB |
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
├── Cargo.lock (0 LoC | 61.63KB)
├── Cargo.toml (26 LoC | 1005.00B)
├── Justfile (0 LoC | 1.69KB)
├── biome.json (0 LoC | 413.00B)
├── crates/ [78.54KB]
│   ├── api_server/ [28.78KB]
│   │   ├── Cargo.toml (37 LoC | 1.45KB)
│   │   ├── src/ [17.06KB]
│   │   │   ├── config/ [3.26KB]
│   │   │   │   ├── di.rs (62 LoC | 3.18KB)
│   │   │   │   └── mod.rs (3 LoC | 86.00B)
│   │   │   ├── entry_points/ [10.47KB]
│   │   │   │   ├── api/ [7.25KB]
│   │   │   │   │   ├── mod.rs (3 LoC | 104.00B)
│   │   │   │   │   └── v1/ [7.15KB]
│   │   │   │   │       ├── errors.rs (60 LoC | 2.49KB)
│   │   │   │   │       ├── mod.rs (12 LoC | 559.00B)
│   │   │   │   │       └── user_handlers.rs (132 LoC | 4.11KB)
│   │   │   │   ├── auth.rs (95 LoC | 3.11KB)
│   │   │   │   └── mod.rs (4 LoC | 107.00B)
│   │   │   ├── lib.rs (20 LoC | 886.00B)
│   │   │   ├── main.rs (42 LoC | 1.81KB)
│   │   │   └── routes.rs (18 LoC | 668.00B)
│   │   └── tests/ [10.26KB]
│   │       └── integration_tests.rs (278 LoC | 10.26KB)
│   ├── core_logic/ [26.60KB]
│   │   ├── Cargo.toml (28 LoC | 1022.00B)
│   │   └── src/ [25.60KB]
│   │       ├── application/ [8.67KB]
│   │       │   ├── mod.rs (14 LoC | 577.00B)
│   │       │   └── use_cases/ [8.11KB]
│   │       │       ├── mod.rs (9 LoC | 371.00B)
│   │       │       └── user/ [7.74KB]
│   │       │           ├── create_session.rs (42 LoC | 1.53KB)
│   │       │           ├── get_user_by_id.rs (31 LoC | 971.00B)
│   │       │           ├── login.rs (59 LoC | 2.02KB)
│   │       │           ├── mod.rs (13 LoC | 467.00B)
│   │       │           └── register.rs (70 LoC | 2.79KB)
│   │       ├── domain/ [16.51KB]
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
│   │       │   └── value_objects/ [6.54KB]
│   │       │       ├── email.rs (50 LoC | 1.84KB)
│   │       │       ├── mod.rs (19 LoC | 855.00B)
│   │       │       ├── password_hash.rs (36 LoC | 1.29KB)
│   │       │       ├── session_token.rs (29 LoC | 1010.00B)
│   │       │       └── user_id.rs (45 LoC | 1.59KB)
│   │       └── lib.rs (11 LoC | 430.00B)
│   └── infra_db/ [23.17KB]
│       ├── Cargo.toml (29 LoC | 1.24KB)
│       ├── migrations/ [5.04KB]
│       │   ├── 20260305135148_create_users_table.sql (43 LoC | 2.15KB)
│       │   ├── 20260305135149_create_rbac.sql (19 LoC | 643.00B)
│       │   ├── 20260305135150_create_tokens.sql (11 LoC | 420.00B)
│       │   ├── 20260305135151_create_audit.sql (14 LoC | 530.00B)
│       │   ├── 20260305135152_seed_system_data.sql (9 LoC | 512.00B)
│       │   └── 20260305135153_create_sessions.sql (21 LoC | 852.00B)
│       └── src/ [16.89KB]
│           ├── external_services/ [2.92KB]
│           │   ├── hashing.rs (56 LoC | 2.43KB)
│           │   └── mod.rs (12 LoC | 502.00B)
│           ├── lib.rs (19 LoC | 1.13KB)
│           └── persistence/ [12.84KB]
│               ├── mod.rs (13 LoC | 537.00B)
│               └── sqlite/ [12.32KB]
│                   ├── mod.rs (16 LoC | 736.00B)
│                   ├── models.rs (61 LoC | 2.45KB)
│                   └── repositories/ [9.15KB]
│                       ├── mod.rs (15 LoC | 639.00B)
│                       ├── sqlite_session_repo.rs (112 LoC | 3.97KB)
│                       └── sqlite_user_repo.rs (117 LoC | 4.55KB)
├── deploy/ [1.93KB]
│   └── Caddyfile (0 LoC | 1.93KB)
├── docs/ [75.42KB]
│   ├── DECISIONS.md (0 LoC | 17.34KB)
│   ├── RUST_STANDARDS.md (0 LoC | 3.42KB)
│   ├── TESTING.md (0 LoC | 4.40KB)
│   ├── TROUBLESHOOTING.md (0 LoC | 16.80KB)
│   ├── core-iu.md (0 LoC | 5.28KB)
│   ├── docs-fases/ [25.97KB]
│   │   ├── docs-backend.md (0 LoC | 11.15KB)
│   │   └── docs-genesis.md (0 LoC | 14.82KB)
│   └── promps.md (0 LoC | 2.22KB)
├── proto/ [2.24KB]
│   ├── auth/ [1.05KB]
│   │   └── v1/ [1.05KB]
│   │       └── auth.proto (0 LoC | 1.05KB)
│   ├── buf.gen.yaml (14 LoC | 662.00B)
│   ├── buf.yaml (8 LoC | 181.00B)
│   └── common/ [373.00B]
│       └── v1/ [373.00B]
│           └── common.proto (0 LoC | 373.00B)
├── roadmaps/ [11.98KB]
│   ├── backend.md (0 LoC | 7.99KB)
│   ├── genesis.md (0 LoC | 3.39KB)
│   └── master.md (0 LoC | 605.00B)
├── test-api.bat (0 LoC | 649.00B)
├── test-api.sh (0 LoC | 663.00B)
└── ver-proyecto.py (121 LoC | 4.26KB)
```
