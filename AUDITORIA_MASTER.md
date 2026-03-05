# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 305 LoC |
| **Peso Total del Proyecto** | 63.68KB |
| **Estado de Sintonía** | Activa |

### Mapa de Arquitectura y Pesos
```text
├── .cursorrules.md (0 LoC | 3.69KB)
├── .pre-commit-config.yaml (34 LoC | 1.06KB)
├── Cargo.toml (14 LoC | 524.00B)
├── Justfile (0 LoC | 621.00B)
├── biome.json (0 LoC | 412.00B)
├── crates/ [3.82KB]
│   ├── api_server/ [309.00B]
│   │   ├── Cargo.toml (6 LoC | 132.00B)
│   │   └── src/ [177.00B]
│   │       └── lib.rs (3 LoC | 177.00B)
│   └── core_logic/ [3.52KB]
│       ├── Cargo.toml (12 LoC | 320.00B)
│       └── src/ [3.21KB]
│           ├── domain/ [2.97KB]
│           │   ├── entities/ [677.00B]
│           │   │   ├── mod.rs (1 LoC | 15.00B)
│           │   │   └── user.rs (25 LoC | 662.00B)
│           │   ├── errors.rs (10 LoC | 309.00B)
│           │   ├── interfaces/ [1.00KB]
│           │   │   ├── mod.rs (5 LoC | 232.00B)
│           │   │   └── user_repo.rs (18 LoC | 793.00B)
│           │   ├── mod.rs (4 LoC | 79.00B)
│           │   └── value_objects/ [947.00B]
│           │       ├── email.rs (25 LoC | 931.00B)
│           │       └── mod.rs (1 LoC | 16.00B)
│           └── lib.rs (4 LoC | 247.00B)
├── deploy/ [1.35KB]
│   └── Caddyfile (0 LoC | 1.35KB)
├── docs/ [38.50KB]
│   ├── DECISIONS.md (0 LoC | 17.34KB)
│   ├── RUST_STANDARDS.md (0 LoC | 3.42KB)
│   ├── TROUBLESHOOTING.md (0 LoC | 0.00B)
│   ├── core-iu.md (0 LoC | 5.28KB)
│   └── docs-fases/ [12.47KB]
│       └── docs-genesis.md (0 LoC | 12.47KB)
├── proto/ [2.36KB]
│   ├── auth.proto (0 LoC | 1.28KB)
│   ├── buf.gen.yaml (14 LoC | 661.00B)
│   ├── buf.yaml (8 LoC | 175.00B)
│   └── common.proto (0 LoC | 268.00B)
├── roadmaps/ [7.12KB]
│   ├── backend.md (0 LoC | 3.61KB)
│   ├── genesis.md (0 LoC | 2.92KB)
│   └── master.md (0 LoC | 604.00B)
└── ver-proyecto.py (121 LoC | 4.26KB)
```
