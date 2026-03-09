# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 4812 LoC |
| **Peso Total del Proyecto** | 626.18KB |
| **Estado de Sintonía** | Activa |

### Mapa de Arquitectura y Pesos
```text
├── .biomeignore (0 LoC | 23.00B)
├── .cursorrules-back.md (0 LoC | 3.69KB)
├── .cursorrules.md (0 LoC | 2.47KB)
├── .env.example (0 LoC | 504.00B)
├── .pre-commit-config.yaml (36 LoC | 1.05KB)
├── .sqlx/ [3.17KB]
│   ├── query-d210c01bfa9798e7ce178f0ae12077cac8203e6607883a8dffed40f3e70977c5.json (0 LoC | 1.19KB)
│   ├── query-d82cf1da92d2cb100f72bac4f77238c964d4aabeb70b26ea55c5b89f620787d1.json (0 LoC | 1.19KB)
│   └── query-e20e8388eb12c09c1121cf86d730e498226a5de22f49e9b3e0f921e3442b728c.json (0 LoC | 815.00B)
├── Cargo.lock (0 LoC | 65.57KB)
├── Cargo.toml (26 LoC | 1005.00B)
├── Justfile (0 LoC | 1.69KB)
├── apps/ [283.31KB]
│   └── frontend_astro/ [283.31KB]
│       ├── README.md (0 LoC | 1.60KB)
│       ├── astro.config.mjs (0 LoC | 478.00B)
│       ├── bun.lock (0 LoC | 187.44KB)
│       ├── components.json (0 LoC | 523.00B)
│       ├── package.json (0 LoC | 964.00B)
│       ├── postcss.config.cjs (0 LoC | 72.00B)
│       ├── public/ [1.37KB]
│       │   ├── favicon.ico (0 LoC | 655.00B)
│       │   └── favicon.svg (0 LoC | 749.00B)
│       ├── src/ [90.66KB]
│       │   ├── application/ [0.00B]
│       │   │   ├── stores/ [0.00B]
│       │   │   │   └── auth.ts (0 LoC | 0.00B)
│       │   │   └── use-cases/ [0.00B]
│       │   ├── assets/ [4.27KB]
│       │   │   ├── astro.svg (0 LoC | 2.85KB)
│       │   │   └── background.svg (0 LoC | 1.42KB)
│       │   ├── components/ [7.17KB]
│       │   │   └── ui/ [7.17KB]
│       │   │       ├── button.tsx (54 LoC | 3.08KB)
│       │   │       ├── card.tsx (94 LoC | 2.57KB)
│       │   │       ├── input.tsx (17 LoC | 1.02KB)
│       │   │       └── label.tsx (16 LoC | 518.00B)
│       │   ├── domain/ [669.00B]
│       │   │   ├── entities/ [280.00B]
│       │   │   │   ├── .gitkeep (0 LoC | 0.00B)
│       │   │   │   └── auth.ts (17 LoC | 280.00B)
│       │   │   ├── interfaces/ [0.00B]
│       │   │   │   └── .gitkeep (0 LoC | 0.00B)
│       │   │   └── schemas/ [389.00B]
│       │   │       ├── .gitkeep (0 LoC | 0.00B)
│       │   │       └── auth.ts (12 LoC | 389.00B)
│       │   ├── infrastructure/ [3.49KB]
│       │   │   ├── api/ [3.49KB]
│       │   │   │   ├── auth-client.ts (71 LoC | 2.14KB)
│       │   │   │   ├── htmx-bridge.ts (0 LoC | 0.00B)
│       │   │   │   └── hx-bridge.ts (34 LoC | 1.34KB)
│       │   │   └── storage/ [0.00B]
│       │   ├── lib/ [3.37KB]
│       │   │   ├── alpine.ts (69 LoC | 1.93KB)
│       │   │   ├── db.ts (42 LoC | 1.29KB)
│       │   │   └── utils.ts (5 LoC | 166.00B)
│       │   ├── middleware.ts (16 LoC | 647.00B)
│       │   ├── pages/ [44.63KB]
│       │   │   ├── api/ [16.38KB]
│       │   │   │   ├── auth/ [5.77KB]
│       │   │   │   │   ├── login.ts (38 LoC | 1.20KB)
│       │   │   │   │   ├── logout.ts (19 LoC | 544.00B)
│       │   │   │   │   ├── me.ts (45 LoC | 1.42KB)
│       │   │   │   │   └── register.ts (67 LoC | 2.61KB)
│       │   │   │   ├── health.ts (9 LoC | 254.00B)
│       │   │   │   └── users/ [10.36KB]
│       │   │   │       ├── create.ts (76 LoC | 2.98KB)
│       │   │   │       ├── delete.ts (23 LoC | 808.00B)
│       │   │   │       ├── edit.ts (86 LoC | 3.26KB)
│       │   │   │       └── index.ts (84 LoC | 3.33KB)
│       │   │   ├── dashboard.astro (164 LoC | 8.29KB)
│       │   │   ├── index.astro (12 LoC | 436.00B)
│       │   │   ├── login.astro (93 LoC | 3.40KB)
│       │   │   ├── logout.astro (13 LoC | 233.00B)
│       │   │   ├── register.astro (112 LoC | 4.03KB)
│       │   │   └── users.astro (228 LoC | 11.89KB)
│       │   ├── presentation/ [20.03KB]
│       │   │   ├── components/ [15.86KB]
│       │   │   │   ├── auth/ [444.00B]
│       │   │   │   │   └── Guard.astro (12 LoC | 444.00B)
│       │   │   │   ├── command/ [5.95KB]
│       │   │   │   │   └── CommandPalette.astro (93 LoC | 5.95KB)
│       │   │   │   ├── shared/ [6.78KB]
│       │   │   │   │   ├── Navbar.astro (60 LoC | 4.29KB)
│       │   │   │   │   └── Sidebar.astro (55 LoC | 2.49KB)
│       │   │   │   └── ui/ [2.69KB]
│       │   │   │       ├── Button.astro (57 LoC | 1.51KB)
│       │   │   │       ├── Card.astro (14 LoC | 220.00B)
│       │   │   │       └── Input.astro (41 LoC | 992.00B)
│       │   │   ├── layouts/ [4.17KB]
│       │   │   │   └── MainLayout.astro (105 LoC | 4.17KB)
│       │   │   └── pages/ [0.00B]
│       │   │       └── index.astro (0 LoC | 0.00B)
│       │   ├── styles/ [5.87KB]
│       │   │   └── global.css (178 LoC | 5.87KB)
│       │   └── types/ [567.00B]
│       │       └── htmx.d.ts (24 LoC | 567.00B)
│       └── tsconfig.json (0 LoC | 259.00B)
├── biome.json (0 LoC | 788.00B)
├── crates/ [92.47KB]
│   ├── api_server/ [37.77KB]
│   │   ├── Cargo.toml (42 LoC | 1.67KB)
│   │   ├── src/ [25.48KB]
│   │   │   ├── config/ [3.80KB]
│   │   │   │   ├── di.rs (61 LoC | 2.93KB)
│   │   │   │   ├── env.rs (21 LoC | 792.00B)
│   │   │   │   └── mod.rs (4 LoC | 100.00B)
│   │   │   ├── entry_points/ [13.05KB]
│   │   │   │   ├── api/ [10.02KB]
│   │   │   │   │   ├── mod.rs (3 LoC | 104.00B)
│   │   │   │   │   └── v1/ [9.92KB]
│   │   │   │   │       ├── errors.rs (58 LoC | 2.56KB)
│   │   │   │   │       ├── mod.rs (12 LoC | 559.00B)
│   │   │   │   │       └── user_handlers.rs (203 LoC | 6.81KB)
│   │   │   │   ├── auth.rs (80 LoC | 2.93KB)
│   │   │   │   └── mod.rs (4 LoC | 107.00B)
│   │   │   ├── errors.rs (41 LoC | 2.00KB)
│   │   │   ├── lib.rs (20 LoC | 886.00B)
│   │   │   ├── main.rs (109 LoC | 4.76KB)
│   │   │   └── routes.rs (26 LoC | 1.01KB)
│   │   └── tests/ [10.62KB]
│   │       └── integration_tests.rs (281 LoC | 10.62KB)
│   ├── core_logic/ [30.74KB]
│   │   ├── Cargo.toml (29 LoC | 1.06KB)
│   │   └── src/ [29.69KB]
│   │       ├── application/ [11.88KB]
│   │       │   ├── mod.rs (14 LoC | 577.00B)
│   │       │   └── use_cases/ [11.32KB]
│   │       │       ├── mod.rs (9 LoC | 371.00B)
│   │       │       └── user/ [10.95KB]
│   │       │           ├── create_session.rs (47 LoC | 1.62KB)
│   │       │           ├── delete.rs (16 LoC | 544.00B)
│   │       │           ├── get_user_by_id.rs (27 LoC | 963.00B)
│   │       │           ├── list.rs (15 LoC | 467.00B)
│   │       │           ├── login.rs (58 LoC | 2.07KB)
│   │       │           ├── mod.rs (16 LoC | 513.00B)
│   │       │           ├── register.rs (74 LoC | 2.93KB)
│   │       │           └── update.rs (57 LoC | 1.90KB)
│   │       ├── domain/ [17.39KB]
│   │       │   ├── entities/ [4.80KB]
│   │       │   │   ├── mod.rs (17 LoC | 612.00B)
│   │       │   │   ├── session.rs (17 LoC | 488.00B)
│   │       │   │   └── user.rs (116 LoC | 3.73KB)
│   │       │   ├── errors.rs (29 LoC | 1.06KB)
│   │       │   ├── interfaces/ [4.25KB]
│   │       │   │   ├── hasher.rs (24 LoC | 1.14KB)
│   │       │   │   ├── mod.rs (20 LoC | 816.00B)
│   │       │   │   ├── session_repo.rs (16 LoC | 694.00B)
│   │       │   │   └── user_repo.rs (36 LoC | 1.64KB)
│   │       │   ├── mod.rs (16 LoC | 679.00B)
│   │       │   └── value_objects/ [6.60KB]
│   │       │       ├── email.rs (50 LoC | 1.84KB)
│   │       │       ├── mod.rs (19 LoC | 855.00B)
│   │       │       ├── password_hash.rs (36 LoC | 1.29KB)
│   │       │       ├── session_token.rs (31 LoC | 1.05KB)
│   │       │       └── user_id.rs (45 LoC | 1.59KB)
│   │       └── lib.rs (11 LoC | 430.00B)
│   └── infra_db/ [23.95KB]
│       ├── Cargo.toml (29 LoC | 1.24KB)
│       ├── migrations/ [5.04KB]
│       │   ├── 20260305135148_create_users_table.sql (43 LoC | 2.15KB)
│       │   ├── 20260305135149_create_rbac.sql (19 LoC | 643.00B)
│       │   ├── 20260305135150_create_tokens.sql (11 LoC | 420.00B)
│       │   ├── 20260305135151_create_audit.sql (14 LoC | 530.00B)
│       │   ├── 20260305135152_seed_system_data.sql (9 LoC | 512.00B)
│       │   └── 20260305135153_create_sessions.sql (21 LoC | 852.00B)
│       └── src/ [17.67KB]
│           ├── external_services/ [2.92KB]
│           │   ├── hashing.rs (56 LoC | 2.43KB)
│           │   └── mod.rs (12 LoC | 502.00B)
│           ├── lib.rs (19 LoC | 1.13KB)
│           └── persistence/ [13.63KB]
│               ├── mod.rs (13 LoC | 537.00B)
│               └── sqlite/ [13.10KB]
│                   ├── mod.rs (16 LoC | 736.00B)
│                   ├── models.rs (61 LoC | 2.45KB)
│                   └── repositories/ [9.93KB]
│                       ├── mod.rs (15 LoC | 639.00B)
│                       ├── sqlite_session_repo.rs (112 LoC | 4.09KB)
│                       └── sqlite_user_repo.rs (139 LoC | 5.22KB)
├── deploy/ [4.21KB]
│   ├── Caddyfile (0 LoC | 785.00B)
│   ├── Dockerfile (0 LoC | 1.84KB)
│   └── podman-compose.yml (53 LoC | 1.61KB)
├── docs/ [136.93KB]
│   ├── DECISIONS.md (0 LoC | 17.20KB)
│   ├── RUST_STANDARDS.md (0 LoC | 3.56KB)
│   ├── TESTING.md (0 LoC | 4.67KB)
│   ├── TROUBLESHOOTING.md (0 LoC | 16.80KB)
│   ├── core-iu.md (0 LoC | 5.28KB)
│   ├── docs-fases/ [74.67KB]
│   │   ├── docs-backend.md (0 LoC | 7.40KB)
│   │   ├── docs-backend1.md (0 LoC | 33.01KB)
│   │   ├── docs-frontend.md (0 LoC | 19.42KB)
│   │   └── docs-genesis.md (0 LoC | 14.82KB)
│   ├── promps-back.md (0 LoC | 2.90KB)
│   ├── promps-front.md (0 LoC | 1.66KB)
│   └── testing-front.md (0 LoC | 10.21KB)
├── logs/ [4.06KB]
│   └── backend.log.2026-03-09 (0 LoC | 4.06KB)
├── proto/ [2.24KB]
│   ├── auth/ [1.05KB]
│   │   └── v1/ [1.05KB]
│   │       └── auth.proto (0 LoC | 1.05KB)
│   ├── buf.gen.yaml (14 LoC | 662.00B)
│   ├── buf.yaml (8 LoC | 181.00B)
│   └── common/ [373.00B]
│       └── v1/ [373.00B]
│           └── common.proto (0 LoC | 373.00B)
├── roadmaps/ [17.50KB]
│   ├── backend.md (0 LoC | 9.43KB)
│   ├── frontend.md (0 LoC | 4.08KB)
│   ├── genesis.md (0 LoC | 3.39KB)
│   └── master.md (0 LoC | 605.00B)
├── test-api.bat (0 LoC | 649.00B)
├── test-api.sh (0 LoC | 663.00B)
└── ver-proyecto.py (121 LoC | 4.26KB)
```
