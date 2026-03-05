# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 446 LoC |
| **Peso Total del Proyecto** | 133.74KB |
| **Estado de Sintonía** | Activa |

### Mapa de Arquitectura y Pesos
```text
├── .cursorrules.md (0 LoC | 3.69KB)
├── .env.example (0 LoC | 451.00B)
├── .pre-commit-config.yaml (34 LoC | 1.06KB)
├── Cargo.lock (0 LoC | 60.77KB)
├── Cargo.toml (14 LoC | 522.00B)
├── Justfile (0 LoC | 1.47KB)
├── biome.json (0 LoC | 413.00B)
├── crates/ [8.97KB]
│   ├── api_server/ [308.00B]
│   │   ├── Cargo.toml (6 LoC | 132.00B)
│   │   └── src/ [176.00B]
│   │       └── lib.rs (3 LoC | 176.00B)
│   ├── core_logic/ [3.43KB]
│   │   ├── Cargo.toml (12 LoC | 320.00B)
│   │   └── src/ [3.12KB]
│   │       ├── domain/ [2.88KB]
│   │       │   ├── entities/ [652.00B]
│   │       │   │   ├── mod.rs (1 LoC | 15.00B)
│   │       │   │   └── user.rs (25 LoC | 637.00B)
│   │       │   ├── errors.rs (10 LoC | 300.00B)
│   │       │   ├── interfaces/ [1005.00B]
│   │       │   │   ├── mod.rs (5 LoC | 229.00B)
│   │       │   │   └── user_repo.rs (18 LoC | 776.00B)
│   │       │   ├── mod.rs (4 LoC | 77.00B)
│   │       │   └── value_objects/ [910.00B]
│   │       │       ├── email.rs (25 LoC | 894.00B)
│   │       │       └── mod.rs (1 LoC | 16.00B)
│   │       └── lib.rs (4 LoC | 247.00B)
│   └── infra_db/ [5.24KB]
│       ├── Cargo.toml (12 LoC | 475.00B)
│       ├── migrations/ [396.00B]
│       │   └── 20260305135148_create_users_table.sql (9 LoC | 396.00B)
│       └── src/ [4.39KB]
│           ├── lib.rs (4 LoC | 250.00B)
│           └── persistence/ [4.15KB]
│               ├── mod.rs (2 LoC | 58.00B)
│               └── sqlite/ [4.09KB]
│                   ├── mod.rs (3 LoC | 88.00B)
│                   └── repositories/ [4.00KB]
│                       ├── mod.rs (2 LoC | 88.00B)
│                       ├── models.rs (27 LoC | 979.00B)
│                       └── sqlite_user_repo.rs (82 LoC | 2.96KB)
├── deploy/ [1.35KB]
│   └── Caddyfile (0 LoC | 1.35KB)
├── docs/ [40.86KB]
│   ├── DECISIONS.md (0 LoC | 17.34KB)
│   ├── RUST_STANDARDS.md (0 LoC | 3.42KB)
│   ├── TROUBLESHOOTING.md (0 LoC | 0.00B)
│   ├── core-iu.md (0 LoC | 5.28KB)
│   └── docs-fases/ [14.83KB]
│       └── docs-genesis.md (0 LoC | 14.83KB)
├── proto/ [2.36KB]
│   ├── auth.proto (0 LoC | 1.28KB)
│   ├── buf.gen.yaml (14 LoC | 661.00B)
│   ├── buf.yaml (8 LoC | 175.00B)
│   └── common.proto (0 LoC | 268.00B)
├── roadmaps/ [7.60KB]
│   ├── backend.md (0 LoC | 3.61KB)
│   ├── genesis.md (0 LoC | 3.39KB)
│   └── master.md (0 LoC | 604.00B)
└── ver-proyecto.py (121 LoC | 4.26KB)
```
