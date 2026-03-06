# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 771 LoC |
| **Peso Total del Proyecto** | 179.23KB |
| **Estado de Sintonía** | Activa |

### Mapa de Arquitectura y Pesos
```text
├── .cursorrules.md (0 LoC | 3.69KB)
├── .env.example (0 LoC | 451.00B)
├── .pre-commit-config.yaml (34 LoC | 1.06KB)
├── .sqlx/ [3.29KB]
│   ├── query-d210c01bfa9798e7ce178f0ae12077cac8203e6607883a8dffed40f3e70977c5.json (0 LoC | 1.25KB)
│   ├── query-d82cf1da92d2cb100f72bac4f77238c964d4aabeb70b26ea55c5b89f620787d1.json (0 LoC | 1.25KB)
│   └── query-e20e8388eb12c09c1121cf86d730e498226a5de22f49e9b3e0f921e3442b728c.json (0 LoC | 815.00B)
├── Cargo.lock (0 LoC | 61.83KB)
├── Cargo.toml (14 LoC | 522.00B)
├── Justfile (0 LoC | 1.69KB)
├── biome.json (0 LoC | 413.00B)
├── crates/ [22.74KB]
│   ├── api_server/ [625.00B]
│   │   ├── Cargo.toml (9 LoC | 284.00B)
│   │   └── src/ [341.00B]
│   │       ├── lib.rs (3 LoC | 176.00B)
│   │       └── main.rs (5 LoC | 165.00B)
│   ├── core_logic/ [12.40KB]
│   │   ├── Cargo.toml (12 LoC | 320.00B)
│   │   └── src/ [12.08KB]
│   │       ├── application/ [2.58KB]
│   │       │   └── use_cases/ [2.58KB]
│   │       │       └── user/ [2.58KB]
│   │       │           ├── mod.rs (2 LoC | 77.00B)
│   │       │           └── register.rs (57 LoC | 2.51KB)
│   │       ├── domain/ [9.08KB]
│   │       │   ├── entities/ [3.04KB]
│   │       │   │   ├── mod.rs (1 LoC | 15.00B)
│   │       │   │   └── user.rs (81 LoC | 3.03KB)
│   │       │   ├── errors.rs (8 LoC | 249.00B)
│   │       │   ├── interfaces/ [2.11KB]
│   │       │   │   ├── hasher.rs (24 LoC | 1.13KB)
│   │       │   │   ├── mod.rs (6 LoC | 216.00B)
│   │       │   │   └── user_repo.rs (17 LoC | 782.00B)
│   │       │   ├── mod.rs (4 LoC | 77.00B)
│   │       │   └── value_objects/ [3.61KB]
│   │       │       ├── email.rs (25 LoC | 894.00B)
│   │       │       ├── mod.rs (17 LoC | 812.00B)
│   │       │       ├── password_hash.rs (34 LoC | 1.26KB)
│   │       │       └── user_id.rs (23 LoC | 706.00B)
│   │       └── lib.rs (11 LoC | 429.00B)
│   └── infra_db/ [9.74KB]
│       ├── Cargo.toml (14 LoC | 547.00B)
│       ├── migrations/ [984.00B]
│       │   └── 20260305135148_create_users_table.sql (23 LoC | 984.00B)
│       └── src/ [8.24KB]
│           ├── external_services/ [2.41KB]
│           │   ├── hashing.rs (55 LoC | 2.36KB)
│           │   └── mod.rs (2 LoC | 54.00B)
│           ├── lib.rs (7 LoC | 429.00B)
│           └── persistence/ [5.41KB]
│               ├── mod.rs (2 LoC | 58.00B)
│               └── sqlite/ [5.36KB]
│                   ├── mod.rs (3 LoC | 88.00B)
│                   ├── models.rs (34 LoC | 1.33KB)
│                   └── repositories/ [3.94KB]
│                       ├── mod.rs (2 LoC | 88.00B)
│                       └── sqlite_user_repo.rs (99 LoC | 3.85KB)
├── deploy/ [1.35KB]
│   └── Caddyfile (0 LoC | 1.35KB)
├── docs/ [65.99KB]
│   ├── DECISIONS.md (0 LoC | 17.34KB)
│   ├── RUST_STANDARDS.md (0 LoC | 3.42KB)
│   ├── TROUBLESHOOTING.md (0 LoC | 6.80KB)
│   ├── core-iu.md (0 LoC | 5.28KB)
│   └── docs-fases/ [33.16KB]
│       ├── docs-backend.md (0 LoC | 18.33KB)
│       └── docs-genesis.md (0 LoC | 14.83KB)
├── proto/ [2.36KB]
│   ├── auth.proto (0 LoC | 1.28KB)
│   ├── buf.gen.yaml (14 LoC | 661.00B)
│   ├── buf.yaml (8 LoC | 175.00B)
│   └── common.proto (0 LoC | 268.00B)
├── roadmaps/ [9.61KB]
│   ├── backend.md (0 LoC | 5.62KB)
│   ├── genesis.md (0 LoC | 3.39KB)
│   └── master.md (0 LoC | 604.00B)
└── ver-proyecto.py (121 LoC | 4.26KB)
```
