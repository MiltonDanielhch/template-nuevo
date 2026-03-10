# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 7206 LoC |
| **Peso Total del Proyecto** | 2.12MB |
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
├── Cargo.lock (0 LoC | 61.65KB)
├── Cargo.toml (26 LoC | 1005.00B)
├── Justfile (0 LoC | 1.69KB)
├── apps/ [336.18KB]
│   └── frontend_astro/ [336.18KB]
│       ├── README.md (0 LoC | 1.60KB)
│       ├── astro.config.mjs (0 LoC | 478.00B)
│       ├── bun.lock (0 LoC | 188.62KB)
│       ├── components.json (0 LoC | 523.00B)
│       ├── package.json (0 LoC | 1.13KB)
│       ├── postcss.config.cjs (0 LoC | 72.00B)
│       ├── public/ [1.37KB]
│       │   ├── favicon.ico (0 LoC | 655.00B)
│       │   └── favicon.svg (0 LoC | 749.00B)
│       ├── src/ [142.16KB]
│       │   ├── application/ [0.00B]
│       │   │   ├── stores/ [0.00B]
│       │   │   │   └── auth.ts (0 LoC | 0.00B)
│       │   │   └── use-cases/ [0.00B]
│       │   ├── assets/ [4.27KB]
│       │   │   ├── astro.svg (0 LoC | 2.85KB)
│       │   │   └── background.svg (0 LoC | 1.42KB)
│       │   ├── components/ [7.18KB]
│       │   │   └── ui/ [7.18KB]
│       │   │       ├── button.tsx (54 LoC | 3.09KB)
│       │   │       ├── card.tsx (83 LoC | 2.55KB)
│       │   │       ├── input.tsx (17 LoC | 1.03KB)
│       │   │       └── label.tsx (16 LoC | 529.00B)
│       │   ├── domain/ [669.00B]
│       │   │   ├── entities/ [280.00B]
│       │   │   │   ├── .gitkeep (0 LoC | 0.00B)
│       │   │   │   └── auth.ts (17 LoC | 280.00B)
│       │   │   ├── interfaces/ [0.00B]
│       │   │   │   └── .gitkeep (0 LoC | 0.00B)
│       │   │   └── schemas/ [389.00B]
│       │   │       ├── .gitkeep (0 LoC | 0.00B)
│       │   │       └── auth.ts (12 LoC | 389.00B)
│       │   ├── infrastructure/ [3.48KB]
│       │   │   ├── api/ [3.48KB]
│       │   │   │   ├── auth-client.ts (71 LoC | 2.14KB)
│       │   │   │   ├── htmx-bridge.ts (0 LoC | 0.00B)
│       │   │   │   └── hx-bridge.ts (34 LoC | 1.34KB)
│       │   │   └── storage/ [0.00B]
│       │   ├── lib/ [5.60KB]
│       │   │   ├── alpine.ts (125 LoC | 4.15KB)
│       │   │   ├── db.ts (42 LoC | 1.29KB)
│       │   │   └── utils.ts (5 LoC | 169.00B)
│       │   ├── middleware.ts (5 LoC | 180.00B)
│       │   ├── pages/ [76.53KB]
│       │   │   ├── api/ [29.28KB]
│       │   │   │   ├── auth/ [5.73KB]
│       │   │   │   │   ├── login.ts (39 LoC | 1.31KB)
│       │   │   │   │   ├── logout.ts (14 LoC | 396.00B)
│       │   │   │   │   ├── me.ts (45 LoC | 1.42KB)
│       │   │   │   │   └── register.ts (67 LoC | 2.61KB)
│       │   │   │   ├── health.ts (16 LoC | 321.00B)
│       │   │   │   ├── roles/ [4.43KB]
│       │   │   │   │   ├── [id].ts (57 LoC | 1.78KB)
│       │   │   │   │   ├── index.ts (62 LoC | 1.99KB)
│       │   │   │   │   └── permissions.ts (20 LoC | 680.00B)
│       │   │   │   └── users/ [18.80KB]
│       │   │   │       ├── create.ts (66 LoC | 2.89KB)
│       │   │   │       ├── delete.ts (23 LoC | 808.00B)
│       │   │   │       ├── edit.ts (71 LoC | 3.03KB)
│       │   │   │       ├── index.ts (139 LoC | 5.88KB)
│       │   │   │       ├── list.ts (101 LoC | 4.77KB)
│       │   │   │       └── me.ts (48 LoC | 1.43KB)
│       │   │   ├── dashboard.astro (246 LoC | 13.50KB)
│       │   │   ├── index.astro (18 LoC | 699.00B)
│       │   │   ├── login.astro (95 LoC | 3.52KB)
│       │   │   ├── logout.astro (13 LoC | 233.00B)
│       │   │   ├── register.astro (102 LoC | 3.71KB)
│       │   │   ├── roles.astro (191 LoC | 8.22KB)
│       │   │   ├── settings.astro (136 LoC | 5.80KB)
│       │   │   └── users.astro (257 LoC | 11.59KB)
│       │   ├── presentation/ [37.77KB]
│       │   │   ├── components/ [32.05KB]
│       │   │   │   ├── auth/ [449.00B]
│       │   │   │   │   └── Guard.astro (12 LoC | 449.00B)
│       │   │   │   ├── command/ [4.81KB]
│       │   │   │   │   └── CommandPalette.astro (93 LoC | 4.81KB)
│       │   │   │   ├── shared/ [24.11KB]
│       │   │   │   │   ├── LocalFsCard.astro (90 LoC | 3.97KB)
│       │   │   │   │   ├── Navbar.astro (226 LoC | 14.64KB)
│       │   │   │   │   ├── Sidebar.astro (79 LoC | 3.79KB)
│       │   │   │   │   └── TauriGreeter.astro (31 LoC | 1.71KB)
│       │   │   │   └── ui/ [2.69KB]
│       │   │   │       ├── Button.astro (56 LoC | 1.51KB)
│       │   │   │       ├── Card.astro (14 LoC | 220.00B)
│       │   │   │       └── Input.astro (41 LoC | 992.00B)
│       │   │   ├── layouts/ [5.72KB]
│       │   │   │   └── MainLayout.astro (147 LoC | 5.72KB)
│       │   │   └── pages/ [0.00B]
│       │   │       └── index.astro (0 LoC | 0.00B)
│       │   ├── styles/ [5.95KB]
│       │   │   └── global.css (182 LoC | 5.95KB)
│       │   └── types/ [567.00B]
│       │       └── htmx.d.ts (24 LoC | 567.00B)
│       └── tsconfig.json (0 LoC | 259.00B)
├── biome.json (0 LoC | 748.00B)
├── bun.lock (0 LoC | 4.20KB)
├── crates/ [131.33KB]
│   ├── api_server/ [49.49KB]
│   │   ├── Cargo.toml (39 LoC | 1.53KB)
│   │   ├── src/ [37.34KB]
│   │   │   ├── config/ [4.11KB]
│   │   │   │   ├── di.rs (82 LoC | 3.24KB)
│   │   │   │   ├── env.rs (21 LoC | 792.00B)
│   │   │   │   └── mod.rs (4 LoC | 100.00B)
│   │   │   ├── entry_points/ [24.40KB]
│   │   │   │   ├── api/ [19.24KB]
│   │   │   │   │   ├── mod.rs (3 LoC | 104.00B)
│   │   │   │   │   └── v1/ [19.14KB]
│   │   │   │   │       ├── errors.rs (58 LoC | 2.56KB)
│   │   │   │   │       ├── mod.rs (13 LoC | 583.00B)
│   │   │   │   │       ├── role_handlers.rs (180 LoC | 5.82KB)
│   │   │   │   │       └── user_handlers.rs (315 LoC | 10.18KB)
│   │   │   │   ├── auth.rs (80 LoC | 2.93KB)
│   │   │   │   ├── middleware/ [2.11KB]
│   │   │   │   │   ├── mod.rs (2 LoC | 70.00B)
│   │   │   │   │   └── rbac.rs (61 LoC | 2.05KB)
│   │   │   │   └── mod.rs (5 LoC | 127.00B)
│   │   │   ├── errors.rs (41 LoC | 2.00KB)
│   │   │   ├── lib.rs (20 LoC | 886.00B)
│   │   │   ├── main.rs (97 LoC | 4.03KB)
│   │   │   └── routes.rs (44 LoC | 1.93KB)
│   │   └── tests/ [10.62KB]
│   │       └── integration_tests.rs (281 LoC | 10.62KB)
│   ├── core_logic/ [44.94KB]
│   │   ├── Cargo.toml (29 LoC | 1.06KB)
│   │   └── src/ [43.88KB]
│   │       ├── application/ [19.34KB]
│   │       │   ├── mod.rs (14 LoC | 577.00B)
│   │       │   └── use_cases/ [18.78KB]
│   │       │       ├── mod.rs (10 LoC | 385.00B)
│   │       │       ├── role/ [6.71KB]
│   │       │       │   ├── assign.rs (28 LoC | 1.02KB)
│   │       │       │   ├── create.rs (47 LoC | 1.71KB)
│   │       │       │   ├── delete.rs (26 LoC | 911.00B)
│   │       │       │   ├── list.rs (18 LoC | 547.00B)
│   │       │       │   ├── list_permissions.rs (15 LoC | 500.00B)
│   │       │       │   ├── mod.rs (14 LoC | 409.00B)
│   │       │       │   └── update.rs (50 LoC | 1.66KB)
│   │       │       └── user/ [11.69KB]
│   │       │           ├── create_session.rs (47 LoC | 1.62KB)
│   │       │           ├── delete.rs (18 LoC | 557.00B)
│   │       │           ├── get_user_by_id.rs (27 LoC | 963.00B)
│   │       │           ├── list.rs (18 LoC | 542.00B)
│   │       │           ├── login.rs (58 LoC | 2.07KB)
│   │       │           ├── mod.rs (19 LoC | 588.00B)
│   │       │           ├── register.rs (84 LoC | 3.42KB)
│   │       │           └── update.rs (59 LoC | 1.99KB)
│   │       ├── domain/ [24.12KB]
│   │       │   ├── entities/ [8.88KB]
│   │       │   │   ├── mod.rs (19 LoC | 682.00B)
│   │       │   │   ├── role.rs (147 LoC | 3.99KB)
│   │       │   │   ├── session.rs (17 LoC | 488.00B)
│   │       │   │   └── user.rs (116 LoC | 3.75KB)
│   │       │   ├── errors.rs (29 LoC | 1.06KB)
│   │       │   ├── interfaces/ [6.91KB]
│   │       │   │   ├── hasher.rs (24 LoC | 1.14KB)
│   │       │   │   ├── mod.rs (22 LoC | 889.00B)
│   │       │   │   ├── role_repository.rs (52 LoC | 2.58KB)
│   │       │   │   ├── session_repo.rs (16 LoC | 694.00B)
│   │       │   │   └── user_repo.rs (36 LoC | 1.65KB)
│   │       │   ├── mod.rs (16 LoC | 679.00B)
│   │       │   └── value_objects/ [6.60KB]
│   │       │       ├── email.rs (50 LoC | 1.84KB)
│   │       │       ├── mod.rs (19 LoC | 855.00B)
│   │       │       ├── password_hash.rs (36 LoC | 1.29KB)
│   │       │       ├── session_token.rs (31 LoC | 1.05KB)
│   │       │       └── user_id.rs (45 LoC | 1.59KB)
│   │       └── lib.rs (11 LoC | 430.00B)
│   └── infra_db/ [36.91KB]
│       ├── Cargo.toml (29 LoC | 1.24KB)
│       ├── examples/ [347.00B]
│       │   └── generate_hash.rs (11 LoC | 347.00B)
│       ├── migrations/ [6.25KB]
│       │   ├── 20260305135148_create_users_table.sql (43 LoC | 2.15KB)
│       │   ├── 20260305135149_create_rbac.sql (19 LoC | 643.00B)
│       │   ├── 20260305135150_create_tokens.sql (11 LoC | 420.00B)
│       │   ├── 20260305135151_create_audit.sql (14 LoC | 530.00B)
│       │   ├── 20260305135152_seed_system_data.sql (24 LoC | 1.36KB)
│       │   ├── 20260305135153_create_sessions.sql (21 LoC | 852.00B)
│       │   └── 20260305135154_create_user_roles.sql (9 LoC | 360.00B)
│       └── src/ [29.08KB]
│           ├── external_services/ [2.92KB]
│           │   ├── hashing.rs (56 LoC | 2.43KB)
│           │   └── mod.rs (12 LoC | 502.00B)
│           ├── lib.rs (21 LoC | 1.15KB)
│           └── persistence/ [25.01KB]
│               ├── mod.rs (13 LoC | 537.00B)
│               └── sqlite/ [24.49KB]
│                   ├── mod.rs (16 LoC | 736.00B)
│                   ├── models.rs (83 LoC | 3.09KB)
│                   └── repositories/ [20.68KB]
│                       ├── mod.rs (17 LoC | 713.00B)
│                       ├── sqlite_role_repo.rs (327 LoC | 10.70KB)
│                       ├── sqlite_session_repo.rs (112 LoC | 4.09KB)
│                       └── sqlite_user_repo.rs (135 LoC | 5.20KB)
├── deploy/ [4.21KB]
│   ├── Caddyfile (0 LoC | 785.00B)
│   ├── Dockerfile (0 LoC | 1.84KB)
│   └── podman-compose.yml (53 LoC | 1.61KB)
├── docs/ [161.00KB]
│   ├── DECISIONS.md (0 LoC | 17.20KB)
│   ├── RUST_STANDARDS.md (0 LoC | 3.56KB)
│   ├── TESTING.md (0 LoC | 4.67KB)
│   ├── TROUBLESHOOTING.md (0 LoC | 16.80KB)
│   ├── core-iu.md (0 LoC | 5.28KB)
│   ├── docs-fases/ [88.02KB]
│   │   ├── docs-backend.md (0 LoC | 13.24KB)
│   │   ├── docs-backend1.md (0 LoC | 33.01KB)
│   │   ├── docs-capacitor.md (0 LoC | 4.74KB)
│   │   ├── docs-frontend.md (0 LoC | 19.48KB)
│   │   ├── docs-genesis.md (0 LoC | 14.82KB)
│   │   └── docs-tauri.md (0 LoC | 2.72KB)
│   ├── promps-back.md (0 LoC | 2.44KB)
│   ├── promps-capacitor.md (0 LoC | 1.84KB)
│   ├── promps-front.md (0 LoC | 2.15KB)
│   ├── promps-tauri.md (0 LoC | 1.64KB)
│   ├── testing-capacitor.md (0 LoC | 5.22KB)
│   ├── testing-front.md (0 LoC | 7.12KB)
│   └── testing-tauri.md (0 LoC | 5.06KB)
├── logs/ [0.00B]
│   └── backend.log.2026-03-09 (0 LoC | 0.00B)
├── package.json (0 LoC | 200.00B)
├── proto/ [2.24KB]
│   ├── auth/ [1.05KB]
│   │   └── v1/ [1.05KB]
│   │       └── auth.proto (0 LoC | 1.05KB)
│   ├── buf.gen.yaml (14 LoC | 662.00B)
│   ├── buf.yaml (8 LoC | 181.00B)
│   └── common/ [373.00B]
│       └── v1/ [373.00B]
│           └── common.proto (0 LoC | 373.00B)
├── roadmaps/ [23.44KB]
│   ├── backend.md (0 LoC | 10.16KB)
│   ├── capacitor.md (0 LoC | 2.96KB)
│   ├── frontend.md (0 LoC | 4.29KB)
│   ├── genesis.md (0 LoC | 3.39KB)
│   ├── master.md (0 LoC | 605.00B)
│   └── tauri.md (0 LoC | 2.04KB)
├── src-tauri/ [1.39MB]
│   ├── Cargo.toml (25 LoC | 670.00B)
│   ├── build.rs (3 LoC | 39.00B)
│   ├── capabilities/ [240.00B]
│   │   └── default.json (0 LoC | 240.00B)
│   ├── gen/ [923.19KB]
│   │   └── schemas/ [923.19KB]
│   │       ├── acl-manifests.json (0 LoC | 137.62KB)
│   │       ├── capabilities.json (0 LoC | 193.00B)
│   │       ├── desktop-schema.json (0 LoC | 392.69KB)
│   │       └── windows-schema.json (0 LoC | 392.69KB)
│   ├── icons/ [501.34KB]
│   │   ├── 128x128.png (0 LoC | 10.80KB)
│   │   ├── 128x128@2x.png (0 LoC | 22.59KB)
│   │   ├── 32x32.png (0 LoC | 2.17KB)
│   │   ├── Square107x107Logo.png (0 LoC | 8.99KB)
│   │   ├── Square142x142Logo.png (0 LoC | 12.24KB)
│   │   ├── Square150x150Logo.png (0 LoC | 12.73KB)
│   │   ├── Square284x284Logo.png (0 LoC | 25.33KB)
│   │   ├── Square30x30Logo.png (0 LoC | 2.03KB)
│   │   ├── Square310x310Logo.png (0 LoC | 27.84KB)
│   │   ├── Square44x44Logo.png (0 LoC | 3.34KB)
│   │   ├── Square71x71Logo.png (0 LoC | 5.89KB)
│   │   ├── Square89x89Logo.png (0 LoC | 7.37KB)
│   │   ├── StoreLogo.png (0 LoC | 3.88KB)
│   │   ├── icon.icns (0 LoC | 270.51KB)
│   │   ├── icon.ico (0 LoC | 36.83KB)
│   │   └── icon.png (0 LoC | 48.81KB)
│   ├── src/ [1.15KB]
│   │   ├── commands.rs (4 LoC | 124.00B)
│   │   ├── lib.rs (23 LoC | 871.00B)
│   │   └── main.rs (5 LoC | 179.00B)
│   └── tauri.conf.json (0 LoC | 1.24KB)
├── test-api.bat (0 LoC | 649.00B)
├── test-api.sh (0 LoC | 663.00B)
└── ver-proyecto.py (121 LoC | 4.26KB)
```
