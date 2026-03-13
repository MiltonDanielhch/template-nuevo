# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 8816 LoC |
| **Peso Total del Proyecto** | 3.67MB |
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
├── Cargo.lock (0 LoC | 155.69KB)
├── Cargo.toml (27 LoC | 1023.00B)
├── Justfile (0 LoC | 1.69KB)
├── apps/ [1.74MB]
│   └── frontend_astro/ [1.74MB]
│       ├── README.md (0 LoC | 1.60KB)
│       ├── android/ [1.14MB]
│       │   ├── app/ [1.08MB]
│       │   │   ├── build/ [0.00B]
│       │   │   │   └── .npmkeep (0 LoC | 0.00B)
│       │   │   ├── build.gradle (0 LoC | 2.08KB)
│       │   │   ├── capacitor.build.gradle (0 LoC | 422.00B)
│       │   │   ├── proguard-rules.pro (0 LoC | 751.00B)
│       │   │   └── src/ [1.08MB]
│       │   │       ├── androidTest/ [774.00B]
│       │   │       │   └── java/ [774.00B]
│       │   │       │       └── com/ [774.00B]
│       │   │       │           └── getcapacitor/ [774.00B]
│       │   │       │               └── myapp/ [774.00B]
│       │   │       │                   └── ExampleInstrumentedTest.java (0 LoC | 774.00B)
│       │   │       ├── main/ [1.08MB]
│       │   │       │   ├── AndroidManifest.xml (0 LoC | 1.50KB)
│       │   │       │   ├── assets/ [884.44KB]
│       │   │       │   │   ├── capacitor.config.json (0 LoC | 324.00B)
│       │   │       │   │   ├── capacitor.plugins.json (0 LoC | 116.00B)
│       │   │       │   │   └── public/ [884.01KB]
│       │   │       │   │       ├── client/ [385.03KB]
│       │   │       │   │       │   ├── _astro/ [349.74KB]
│       │   │       │   │       │   │   ├── LocalFsCard.astro_astro_type_script_index_0_lang.Ct80Deii.js (2 LoC | 2.68KB)
│       │   │       │   │       │   │   ├── MainLayout.astro_astro_type_script_index_0_lang.DPOytrZ2.js (1 LoC | 2.61KB)
│       │   │       │   │       │   │   ├── TauriGreeter.astro_astro_type_script_index_0_lang.DDq0_nc4.js (1 LoC | 328.00B)
│       │   │       │   │       │   │   ├── client.T9fhd2RU.js (41 LoC | 190.07KB)
│       │   │       │   │       │   │   ├── core.8Cl2bxx0.js (1 LoC | 803.00B)
│       │   │       │   │       │   │   ├── dashboard.BR5CI-XS.css (1 LoC | 48.13KB)
│       │   │       │   │       │   │   ├── geist-cyrillic-wght-normal.CHSlOQsW.woff2 (0 LoC | 14.35KB)
│       │   │       │   │       │   │   ├── geist-latin-ext-wght-normal.DMtmJ5ZE.woff2 (0 LoC | 14.95KB)
│       │   │       │   │       │   │   ├── geist-latin-wght-normal.Dm3htQBi.woff2 (0 LoC | 27.73KB)
│       │   │       │   │       │   │   └── page.eBp8iYjC.js (3 LoC | 48.11KB)
│       │   │       │   │       │   ├── favicon.ico (0 LoC | 655.00B)
│       │   │       │   │       │   ├── favicon.svg (0 LoC | 749.00B)
│       │   │       │   │       │   ├── icon-192.png (0 LoC | 10.80KB)
│       │   │       │   │       │   ├── icon-512.png (0 LoC | 22.59KB)
│       │   │       │   │       │   └── manifest.json (0 LoC | 530.00B)
│       │   │       │   │       ├── cordova.js (0 LoC | 0.00B)
│       │   │       │   │       ├── cordova_plugins.js (0 LoC | 0.00B)
│       │   │       │   │       ├── index.html (36 LoC | 936.00B)
│       │   │       │   │       └── server/ [498.07KB]
│       │   │       │   │           ├── _@astrojs-ssr-adapter.mjs (0 LoC | 94.00B)
│       │   │       │   │           ├── _astro-internal_middleware.mjs (0 LoC | 443.00B)
│       │   │       │   │           ├── chunks/ [380.67KB]
│       │   │       │   │           │   ├── MainLayout_DaDulU1s.mjs (0 LoC | 17.15KB)
│       │   │       │   │           │   ├── Sidebar_D1Y-NQSw.mjs (0 LoC | 17.55KB)
│       │   │       │   │           │   ├── _@astrojs-ssr-adapter_DSyeKdD3.mjs (0 LoC | 61.38KB)
│       │   │       │   │           │   ├── astro/ [108.08KB]
│       │   │       │   │           │   │   └── server_Xn0vGSOg.mjs (0 LoC | 108.08KB)
│       │   │       │   │           │   ├── astro-designed-error-pages_D70pN2ZN.mjs (0 LoC | 11.83KB)
│       │   │       │   │           │   ├── fs-lite_COtHaKzy.mjs (0 LoC | 4.08KB)
│       │   │       │   │           │   ├── index_B69C6PXY.mjs (0 LoC | 88.43KB)
│       │   │       │   │           │   ├── input_BfK8uy_1.mjs (0 LoC | 3.92KB)
│       │   │       │   │           │   ├── label_BMn6tAxN.mjs (0 LoC | 537.00B)
│       │   │       │   │           │   ├── node_BnF3sgX8.mjs (0 LoC | 58.41KB)
│       │   │       │   │           │   ├── noop-middleware_CPmWs0jQ.mjs (0 LoC | 276.00B)
│       │   │       │   │           │   ├── path_CLTPhSP2.mjs (0 LoC | 3.87KB)
│       │   │       │   │           │   ├── remote_DrauV6zU.mjs (0 LoC | 2.15KB)
│       │   │       │   │           │   └── sharp_DDeEBu4p.mjs (0 LoC | 3.03KB)
│       │   │       │   │           ├── entry.mjs (0 LoC | 3.47KB)
│       │   │       │   │           ├── manifest_BbBZq9f-.mjs (0 LoC | 22.96KB)
│       │   │       │   │           ├── noop-entrypoint.mjs (0 LoC | 39.00B)
│       │   │       │   │           ├── pages/ [84.31KB]
│       │   │       │   │           │   ├── _image.astro.mjs (0 LoC | 103.00B)
│       │   │       │   │           │   ├── api/ [25.04KB]
│       │   │       │   │           │   │   ├── auth/ [5.88KB]
│       │   │       │   │           │   │   │   ├── login.astro.mjs (0 LoC | 1.38KB)
│       │   │       │   │           │   │   │   ├── logout.astro.mjs (0 LoC | 574.00B)
│       │   │       │   │           │   │   │   ├── me.astro.mjs (0 LoC | 1.59KB)
│       │   │       │   │           │   │   │   └── register.astro.mjs (0 LoC | 2.35KB)
│       │   │       │   │           │   │   ├── health.astro.mjs (0 LoC | 505.00B)
│       │   │       │   │           │   │   ├── roles/ [2.79KB]
│       │   │       │   │           │   │   │   ├── _id_.astro.mjs (0 LoC | 1.94KB)
│       │   │       │   │           │   │   │   └── permissions.astro.mjs (0 LoC | 865.00B)
│       │   │       │   │           │   │   ├── roles.astro.mjs (0 LoC | 2.15KB)
│       │   │       │   │           │   │   └── users/ [13.74KB]
│       │   │       │   │           │   │       ├── create.astro.mjs (0 LoC | 3.02KB)
│       │   │       │   │           │   │       ├── delete.astro.mjs (0 LoC | 996.00B)
│       │   │       │   │           │   │       ├── edit.astro.mjs (0 LoC | 3.14KB)
│       │   │       │   │           │   │       ├── list.astro.mjs (0 LoC | 5.00KB)
│       │   │       │   │           │   │       └── me.astro.mjs (0 LoC | 1.60KB)
│       │   │       │   │           │   ├── dashboard.astro.mjs (0 LoC | 14.24KB)
│       │   │       │   │           │   ├── index.astro.mjs (0 LoC | 4.90KB)
│       │   │       │   │           │   ├── login.astro.mjs (0 LoC | 4.21KB)
│       │   │       │   │           │   ├── logout.astro.mjs (0 LoC | 902.00B)
│       │   │       │   │           │   ├── register.astro.mjs (0 LoC | 4.49KB)
│       │   │       │   │           │   ├── roles.astro.mjs (0 LoC | 10.54KB)
│       │   │       │   │           │   ├── settings.astro.mjs (0 LoC | 7.93KB)
│       │   │       │   │           │   └── users.astro.mjs (0 LoC | 11.97KB)
│       │   │       │   │           └── renderers.mjs (0 LoC | 6.10KB)
│       │   │       │   ├── java/ [119.00B]
│       │   │       │   │   └── com/ [119.00B]
│       │   │       │   │       └── lab3026/ [119.00B]
│       │   │       │   │           └── app/ [119.00B]
│       │   │       │   │               └── MainActivity.java (0 LoC | 119.00B)
│       │   │       │   └── res/ [215.36KB]
│       │   │       │       ├── drawable/ [9.42KB]
│       │   │       │       │   ├── ic_launcher_background.xml (0 LoC | 5.47KB)
│       │   │       │       │   └── splash.png (0 LoC | 3.95KB)
│       │   │       │       ├── drawable-land-hdpi/ [7.52KB]
│       │   │       │       │   └── splash.png (0 LoC | 7.52KB)
│       │   │       │       ├── drawable-land-mdpi/ [3.95KB]
│       │   │       │       │   └── splash.png (0 LoC | 3.95KB)
│       │   │       │       ├── drawable-land-xhdpi/ [9.03KB]
│       │   │       │       │   └── splash.png (0 LoC | 9.03KB)
│       │   │       │       ├── drawable-land-xxhdpi/ [13.66KB]
│       │   │       │       │   └── splash.png (0 LoC | 13.66KB)
│       │   │       │       ├── drawable-land-xxxhdpi/ [17.27KB]
│       │   │       │       │   └── splash.png (0 LoC | 17.27KB)
│       │   │       │       ├── drawable-port-hdpi/ [7.75KB]
│       │   │       │       │   └── splash.png (0 LoC | 7.75KB)
│       │   │       │       ├── drawable-port-mdpi/ [4.00KB]
│       │   │       │       │   └── splash.png (0 LoC | 4.00KB)
│       │   │       │       ├── drawable-port-xhdpi/ [9.64KB]
│       │   │       │       │   └── splash.png (0 LoC | 9.64KB)
│       │   │       │       ├── drawable-port-xxhdpi/ [13.03KB]
│       │   │       │       │   └── splash.png (0 LoC | 13.03KB)
│       │   │       │       ├── drawable-port-xxxhdpi/ [17.08KB]
│       │   │       │       │   └── splash.png (0 LoC | 17.08KB)
│       │   │       │       ├── drawable-v24/ [1.84KB]
│       │   │       │       │   └── ic_launcher_foreground.xml (0 LoC | 1.84KB)
│       │   │       │       ├── layout/ [535.00B]
│       │   │       │       │   └── activity_main.xml (0 LoC | 535.00B)
│       │   │       │       ├── mipmap-anydpi-v26/ [530.00B]
│       │   │       │       │   ├── ic_launcher.xml (0 LoC | 265.00B)
│       │   │       │       │   └── ic_launcher_round.xml (0 LoC | 265.00B)
│       │   │       │       ├── mipmap-hdpi/ [10.33KB]
│       │   │       │       │   ├── ic_launcher.png (0 LoC | 2.72KB)
│       │   │       │       │   ├── ic_launcher_foreground.png (0 LoC | 3.37KB)
│       │   │       │       │   └── ic_launcher_round.png (0 LoC | 4.24KB)
│       │   │       │       ├── mipmap-mdpi/ [6.55KB]
│       │   │       │       │   ├── ic_launcher.png (0 LoC | 1.83KB)
│       │   │       │       │   ├── ic_launcher_foreground.png (0 LoC | 2.06KB)
│       │   │       │       │   └── ic_launcher_round.png (0 LoC | 2.66KB)
│       │   │       │       ├── mipmap-xhdpi/ [15.24KB]
│       │   │       │       │   ├── ic_launcher.png (0 LoC | 3.89KB)
│       │   │       │       │   ├── ic_launcher_foreground.png (0 LoC | 4.92KB)
│       │   │       │       │   └── ic_launcher_round.png (0 LoC | 6.44KB)
│       │   │       │       ├── mipmap-xxhdpi/ [26.26KB]
│       │   │       │       │   ├── ic_launcher.png (0 LoC | 6.49KB)
│       │   │       │       │   ├── ic_launcher_foreground.png (0 LoC | 9.56KB)
│       │   │       │       │   └── ic_launcher_round.png (0 LoC | 10.21KB)
│       │   │       │       ├── mipmap-xxxhdpi/ [39.93KB]
│       │   │       │       │   ├── ic_launcher.png (0 LoC | 9.22KB)
│       │   │       │       │   ├── ic_launcher_foreground.png (0 LoC | 15.17KB)
│       │   │       │       │   └── ic_launcher_round.png (0 LoC | 15.54KB)
│       │   │       │       ├── values/ [1.22KB]
│       │   │       │       │   ├── ic_launcher_background.xml (0 LoC | 120.00B)
│       │   │       │       │   ├── strings.xml (0 LoC | 302.00B)
│       │   │       │       │   └── styles.xml (0 LoC | 823.00B)
│       │   │       │       └── xml/ [620.00B]
│       │   │       │           ├── config.xml (0 LoC | 407.00B)
│       │   │       │           └── file_paths.xml (0 LoC | 213.00B)
│       │   │       └── test/ [402.00B]
│       │   │           └── java/ [402.00B]
│       │   │               └── com/ [402.00B]
│       │   │                   └── getcapacitor/ [402.00B]
│       │   │                       └── myapp/ [402.00B]
│       │   │                           └── ExampleUnitTest.java (0 LoC | 402.00B)
│       │   ├── build.gradle (0 LoC | 637.00B)
│       │   ├── capacitor-cordova-android-plugins/ [2.17KB]
│       │   │   ├── build.gradle (0 LoC | 1.63KB)
│       │   │   ├── cordova.variables.gradle (0 LoC | 312.00B)
│       │   │   └── src/ [246.00B]
│       │   │       └── main/ [246.00B]
│       │   │           ├── AndroidManifest.xml (0 LoC | 245.00B)
│       │   │           ├── java/ [0.00B]
│       │   │           │   └── .gitkeep (0 LoC | 0.00B)
│       │   │           └── res/ [1.00B]
│       │   │               └── .gitkeep (0 LoC | 1.00B)
│       │   ├── capacitor.settings.gradle (0 LoC | 347.00B)
│       │   ├── gradle/ [42.99KB]
│       │   │   └── wrapper/ [42.99KB]
│       │   │       ├── gradle-wrapper.jar (0 LoC | 42.74KB)
│       │   │       └── gradle-wrapper.properties (0 LoC | 253.00B)
│       │   ├── gradle.properties (0 LoC | 987.00B)
│       │   ├── gradlew (0 LoC | 8.53KB)
│       │   ├── gradlew.bat (0 LoC | 2.87KB)
│       │   ├── settings.gradle (0 LoC | 208.00B)
│       │   └── variables.gradle (0 LoC | 498.00B)
│       ├── astro.config.mjs (0 LoC | 478.00B)
│       ├── bun.lock (0 LoC | 188.62KB)
│       ├── capacitor.config.ts (19 LoC | 448.00B)
│       ├── components.json (0 LoC | 523.00B)
│       ├── package-lock.json (0 LoC | 219.88KB)
│       ├── package.json (0 LoC | 1.27KB)
│       ├── postcss.config.cjs (0 LoC | 72.00B)
│       ├── public/ [35.40KB]
│       │   ├── favicon.ico (0 LoC | 655.00B)
│       │   ├── favicon.svg (0 LoC | 749.00B)
│       │   ├── icon-192.png (0 LoC | 10.80KB)
│       │   ├── icon-512.png (0 LoC | 22.59KB)
│       │   ├── manifest.json (0 LoC | 530.00B)
│       │   └── robots.txt (0 LoC | 118.00B)
│       ├── src/ [163.95KB]
│       │   ├── assets/ [4.27KB]
│       │   │   ├── astro.svg (0 LoC | 2.85KB)
│       │   │   └── background.svg (0 LoC | 1.42KB)
│       │   ├── components/ [7.18KB]
│       │   │   └── ui/ [7.18KB]
│       │   │       ├── button.tsx (54 LoC | 3.09KB)
│       │   │       ├── card.tsx (83 LoC | 2.55KB)
│       │   │       ├── input.tsx (17 LoC | 1.03KB)
│       │   │       └── label.tsx (16 LoC | 529.00B)
│       │   ├── domain/ [786.00B]
│       │   │   ├── entities/ [374.00B]
│       │   │   │   ├── .gitkeep (0 LoC | 0.00B)
│       │   │   │   └── auth.ts (21 LoC | 374.00B)
│       │   │   ├── interfaces/ [0.00B]
│       │   │   │   └── .gitkeep (0 LoC | 0.00B)
│       │   │   └── schemas/ [412.00B]
│       │   │       ├── .gitkeep (0 LoC | 0.00B)
│       │   │       └── auth.ts (13 LoC | 412.00B)
│       │   ├── infrastructure/ [3.97KB]
│       │   │   ├── api/ [3.97KB]
│       │   │   │   ├── auth-client.ts (86 LoC | 2.63KB)
│       │   │   │   └── hx-bridge.ts (34 LoC | 1.34KB)
│       │   │   └── storage/ [0.00B]
│       │   ├── lib/ [5.60KB]
│       │   │   ├── alpine.ts (125 LoC | 4.15KB)
│       │   │   ├── db.ts (42 LoC | 1.29KB)
│       │   │   └── utils.ts (5 LoC | 169.00B)
│       │   ├── middleware.ts (5 LoC | 180.00B)
│       │   ├── pages/ [89.23KB]
│       │   │   ├── api/ [25.93KB]
│       │   │   │   ├── auth/ [6.30KB]
│       │   │   │   │   ├── login.ts (51 LoC | 1.67KB)
│       │   │   │   │   ├── logout.ts (14 LoC | 391.00B)
│       │   │   │   │   ├── me.ts (45 LoC | 1.42KB)
│       │   │   │   │   └── register.ts (76 LoC | 2.83KB)
│       │   │   │   ├── consent/ [592.00B]
│       │   │   │   │   └── opt-out.ts (24 LoC | 592.00B)
│       │   │   │   ├── health.ts (16 LoC | 321.00B)
│       │   │   │   ├── roles/ [4.51KB]
│       │   │   │   │   ├── [id].ts (58 LoC | 1.85KB)
│       │   │   │   │   ├── index.ts (62 LoC | 1.99KB)
│       │   │   │   │   └── permissions.ts (20 LoC | 680.00B)
│       │   │   │   └── users/ [14.22KB]
│       │   │   │       ├── create.ts (66 LoC | 2.89KB)
│       │   │   │       ├── delete.ts (23 LoC | 808.00B)
│       │   │   │       ├── edit.ts (71 LoC | 3.03KB)
│       │   │   │       ├── list.ts (144 LoC | 6.08KB)
│       │   │   │       └── me.ts (48 LoC | 1.43KB)
│       │   │   ├── dashboard.astro (249 LoC | 13.57KB)
│       │   │   ├── index.astro (133 LoC | 7.05KB)
│       │   │   ├── login-nocache.astro (14 LoC | 294.00B)
│       │   │   ├── login.astro (129 LoC | 4.78KB)
│       │   │   ├── logout.astro (14 LoC | 295.00B)
│       │   │   ├── register-nocache.astro (13 LoC | 246.00B)
│       │   │   ├── register.astro (134 LoC | 4.90KB)
│       │   │   ├── roles.astro (236 LoC | 10.69KB)
│       │   │   ├── settings.astro (146 LoC | 6.32KB)
│       │   │   ├── sitemap.xml.ts (27 LoC | 829.00B)
│       │   │   └── users.astro (300 LoC | 14.38KB)
│       │   ├── presentation/ [45.26KB]
│       │   │   ├── components/ [30.29KB]
│       │   │   │   ├── auth/ [449.00B]
│       │   │   │   │   └── Guard.astro (12 LoC | 449.00B)
│       │   │   │   ├── command/ [4.79KB]
│       │   │   │   │   └── CommandPalette.astro (92 LoC | 4.79KB)
│       │   │   │   ├── landing/ [2.46KB]
│       │   │   │   │   └── LeadForm.astro (68 LoC | 2.46KB)
│       │   │   │   ├── shared/ [19.91KB]
│       │   │   │   │   ├── LocalFsCard.astro (90 LoC | 3.97KB)
│       │   │   │   │   ├── Navbar.astro (81 LoC | 5.63KB)
│       │   │   │   │   ├── Sidebar.astro (177 LoC | 8.60KB)
│       │   │   │   │   └── TauriGreeter.astro (31 LoC | 1.71KB)
│       │   │   │   └── ui/ [2.69KB]
│       │   │   │       ├── Button.astro (56 LoC | 1.51KB)
│       │   │   │       ├── Card.astro (14 LoC | 220.00B)
│       │   │   │       └── Input.astro (41 LoC | 992.00B)
│       │   │   ├── layouts/ [14.98KB]
│       │   │   │   ├── LandingLayout.astro (97 LoC | 4.55KB)
│       │   │   │   └── MainLayout.astro (264 LoC | 10.42KB)
│       │   │   └── pages/ [0.00B]
│       │   │       └── index.astro (0 LoC | 0.00B)
│       │   ├── styles/ [6.94KB]
│       │   │   └── global.css (221 LoC | 6.94KB)
│       │   └── types/ [567.00B]
│       │       └── htmx.d.ts (24 LoC | 567.00B)
│       └── tsconfig.json (0 LoC | 259.00B)
├── biome.json (0 LoC | 859.00B)
├── bun.lock (0 LoC | 4.20KB)
├── crates/ [162.20KB]
│   ├── api_server/ [60.56KB]
│   │   ├── Cargo.toml (40 LoC | 1.55KB)
│   │   ├── src/ [47.21KB]
│   │   │   ├── config/ [4.70KB]
│   │   │   │   ├── di.rs (96 LoC | 3.83KB)
│   │   │   │   ├── env.rs (21 LoC | 792.00B)
│   │   │   │   └── mod.rs (4 LoC | 100.00B)
│   │   │   ├── entry_points/ [32.93KB]
│   │   │   │   ├── api/ [22.43KB]
│   │   │   │   │   ├── mod.rs (3 LoC | 104.00B)
│   │   │   │   │   └── v1/ [22.33KB]
│   │   │   │   │       ├── errors.rs (58 LoC | 2.56KB)
│   │   │   │   │       ├── landing_handlers.rs (74 LoC | 3.00KB)
│   │   │   │   │       ├── mod.rs (14 LoC | 610.00B)
│   │   │   │   │       ├── role_handlers.rs (180 LoC | 5.82KB)
│   │   │   │   │       └── user_handlers.rs (325 LoC | 10.34KB)
│   │   │   │   ├── auth.rs (80 LoC | 2.93KB)
│   │   │   │   ├── middleware/ [7.45KB]
│   │   │   │   │   ├── audit.rs (93 LoC | 2.85KB)
│   │   │   │   │   ├── mod.rs (4 LoC | 105.00B)
│   │   │   │   │   ├── rate_limit.rs (82 LoC | 2.45KB)
│   │   │   │   │   └── rbac.rs (61 LoC | 2.05KB)
│   │   │   │   └── mod.rs (5 LoC | 127.00B)
│   │   │   ├── errors.rs (41 LoC | 2.00KB)
│   │   │   ├── lib.rs (20 LoC | 886.00B)
│   │   │   ├── main.rs (100 LoC | 4.20KB)
│   │   │   └── routes.rs (61 LoC | 2.52KB)
│   │   └── tests/ [11.80KB]
│   │       └── integration_tests.rs (318 LoC | 11.80KB)
│   ├── core_logic/ [57.03KB]
│   │   ├── Cargo.toml (32 LoC | 1.11KB)
│   │   └── src/ [55.92KB]
│   │       ├── adapters/ [3.11KB]
│   │       │   ├── mod.rs (1 LoC | 15.00B)
│   │       │   └── proto/ [3.09KB]
│   │       │       └── mod.rs (73 LoC | 3.09KB)
│   │       ├── application/ [20.68KB]
│   │       │   ├── mod.rs (14 LoC | 577.00B)
│   │       │   └── use_cases/ [20.12KB]
│   │       │       ├── lead/ [1.21KB]
│   │       │       │   ├── create.rs (30 LoC | 1.04KB)
│   │       │       │   └── mod.rs (4 LoC | 183.00B)
│   │       │       ├── mod.rs (11 LoC | 402.00B)
│   │       │       ├── role/ [6.71KB]
│   │       │       │   ├── assign.rs (28 LoC | 1.02KB)
│   │       │       │   ├── create.rs (47 LoC | 1.71KB)
│   │       │       │   ├── delete.rs (26 LoC | 911.00B)
│   │       │       │   ├── list.rs (18 LoC | 547.00B)
│   │       │       │   ├── list_permissions.rs (15 LoC | 500.00B)
│   │       │       │   ├── mod.rs (14 LoC | 409.00B)
│   │       │       │   └── update.rs (50 LoC | 1.66KB)
│   │       │       └── user/ [11.80KB]
│   │       │           ├── create_session.rs (47 LoC | 1.62KB)
│   │       │           ├── delete.rs (18 LoC | 557.00B)
│   │       │           ├── get_user_by_id.rs (27 LoC | 963.00B)
│   │       │           ├── list.rs (18 LoC | 542.00B)
│   │       │           ├── login.rs (58 LoC | 2.07KB)
│   │       │           ├── mod.rs (19 LoC | 588.00B)
│   │       │           ├── register.rs (95 LoC | 3.54KB)
│   │       │           └── update.rs (59 LoC | 1.99KB)
│   │       ├── domain/ [29.22KB]
│   │       │   ├── entities/ [11.77KB]
│   │       │   │   ├── audit.rs (40 LoC | 1.15KB)
│   │       │   │   ├── lead.rs (60 LoC | 1.63KB)
│   │       │   │   ├── mod.rs (23 LoC | 791.00B)
│   │       │   │   ├── role.rs (147 LoC | 3.99KB)
│   │       │   │   ├── session.rs (17 LoC | 488.00B)
│   │       │   │   └── user.rs (116 LoC | 3.75KB)
│   │       │   ├── errors.rs (29 LoC | 1.06KB)
│   │       │   ├── interfaces/ [8.00KB]
│   │       │   │   ├── audit_repo.rs (12 LoC | 461.00B)
│   │       │   │   ├── hasher.rs (24 LoC | 1.14KB)
│   │       │   │   ├── lead_repo.rs (14 LoC | 530.00B)
│   │       │   │   ├── mod.rs (26 LoC | 1014.00B)
│   │       │   │   ├── role_repository.rs (52 LoC | 2.58KB)
│   │       │   │   ├── session_repo.rs (16 LoC | 694.00B)
│   │       │   │   └── user_repo.rs (36 LoC | 1.65KB)
│   │       │   ├── mod.rs (16 LoC | 679.00B)
│   │       │   └── value_objects/ [7.73KB]
│   │       │       ├── email.rs (50 LoC | 1.84KB)
│   │       │       ├── lead_id.rs (35 LoC | 1.08KB)
│   │       │       ├── mod.rs (21 LoC | 897.00B)
│   │       │       ├── password_hash.rs (36 LoC | 1.29KB)
│   │       │       ├── session_token.rs (31 LoC | 1.05KB)
│   │       │       └── user_id.rs (45 LoC | 1.59KB)
│   │       ├── lib.rs (13 LoC | 475.00B)
│   │       └── proto_generated/ [2.44KB]
│   │           ├── auth/ [1.80KB]
│   │           │   └── v1/ [1.80KB]
│   │           │       └── auth.rs (50 LoC | 1.80KB)
│   │           ├── common/ [489.00B]
│   │           │   └── v1/ [489.00B]
│   │           │       └── common.rs (15 LoC | 489.00B)
│   │           └── mod.rs (10 LoC | 161.00B)
│   └── infra_db/ [44.61KB]
│       ├── Cargo.toml (29 LoC | 1.24KB)
│       ├── examples/ [347.00B]
│       │   └── generate_hash.rs (11 LoC | 347.00B)
│       ├── migrations/ [7.37KB]
│       │   ├── 20260305135148_create_users_table.sql (43 LoC | 2.15KB)
│       │   ├── 20260305135149_create_rbac.sql (19 LoC | 643.00B)
│       │   ├── 20260305135150_create_tokens.sql (11 LoC | 420.00B)
│       │   ├── 20260305135151_create_audit.sql (14 LoC | 530.00B)
│       │   ├── 20260305135152_seed_system_data.sql (24 LoC | 1.36KB)
│       │   ├── 20260305135153_create_sessions.sql (21 LoC | 852.00B)
│       │   ├── 20260305135154_create_user_roles.sql (9 LoC | 360.00B)
│       │   └── 20260312000000_create_leads.sql (28 LoC | 1.13KB)
│       └── src/ [35.66KB]
│           ├── external_services/ [2.92KB]
│           │   ├── hashing.rs (56 LoC | 2.43KB)
│           │   └── mod.rs (12 LoC | 502.00B)
│           ├── lib.rs (22 LoC | 1.20KB)
│           └── persistence/ [31.54KB]
│               ├── mod.rs (13 LoC | 537.00B)
│               └── sqlite/ [31.02KB]
│                   ├── mod.rs (16 LoC | 736.00B)
│                   ├── models.rs (93 LoC | 3.31KB)
│                   └── repositories/ [26.99KB]
│                       ├── mod.rs (21 LoC | 875.00B)
│                       ├── sqlite_audit_repo.rs (124 LoC | 4.47KB)
│                       ├── sqlite_lead_repo.rs (52 LoC | 1.68KB)
│                       ├── sqlite_role_repo.rs (327 LoC | 10.70KB)
│                       ├── sqlite_session_repo.rs (112 LoC | 4.09KB)
│                       └── sqlite_user_repo.rs (135 LoC | 5.20KB)
├── deploy/ [4.21KB]
│   ├── Caddyfile (0 LoC | 785.00B)
│   ├── Dockerfile (0 LoC | 1.84KB)
│   └── podman-compose.yml (53 LoC | 1.61KB)
├── docs/ [175.34KB]
│   ├── DECISIONS.md (0 LoC | 17.20KB)
│   ├── RUST_STANDARDS.md (0 LoC | 3.56KB)
│   ├── TESTING.md (0 LoC | 4.67KB)
│   ├── TROUBLESHOOTING.md (0 LoC | 16.80KB)
│   ├── checklist-landing.md (0 LoC | 4.37KB)
│   ├── core-iu.md (0 LoC | 5.28KB)
│   ├── docs-fases/ [93.58KB]
│   │   ├── docs-backend.md (0 LoC | 17.22KB)
│   │   ├── docs-backend1.md (0 LoC | 33.01KB)
│   │   ├── docs-capacitor.md (0 LoC | 4.74KB)
│   │   ├── docs-frontend.md (0 LoC | 21.06KB)
│   │   ├── docs-genesis.md (0 LoC | 14.82KB)
│   │   └── docs-tauri.md (0 LoC | 2.72KB)
│   ├── promps-back.md (0 LoC | 2.52KB)
│   ├── promps-capacitor.md (0 LoC | 2.00KB)
│   ├── promps-front.md (0 LoC | 2.64KB)
│   ├── promps-landing.md (0 LoC | 3.68KB)
│   ├── promps-tauri.md (0 LoC | 1.64KB)
│   ├── testing-capacitor.md (0 LoC | 5.22KB)
│   ├── testing-front.md (0 LoC | 7.12KB)
│   └── testing-tauri.md (0 LoC | 5.06KB)
├── docs-fases/ [3.84KB]
│   └── docs-capacitor.md (0 LoC | 3.84KB)
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
├── roadmaps/ [29.66KB]
│   ├── backend.md (0 LoC | 10.16KB)
│   ├── capacitor.md (0 LoC | 3.78KB)
│   ├── frontend.md (0 LoC | 4.40KB)
│   ├── genesis.md (0 LoC | 3.39KB)
│   ├── landing-page.md (0 LoC | 5.30KB)
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
