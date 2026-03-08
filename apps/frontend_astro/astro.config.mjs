import alpine from "@astrojs/alpinejs";
import node from "@astrojs/node";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";

export default defineConfig({
  output: "server",
  adapter: node({
    mode: "standalone",
  }),
  integrations: [alpine({ entrypoint: "/src/lib/alpine.ts" })],
  vite: {
    plugins: [tailwindcss()],
    css: {
      postcss: "./postcss.config.cjs",
    },
  },
});
