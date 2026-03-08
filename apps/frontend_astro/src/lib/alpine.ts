import type { Alpine } from "alpinejs";

interface ThemeStore {
  dark: boolean;
  toggle: () => void;
}

interface AuthStore {
  token: string | null;
  user: unknown;
  setAuth: (token: string, user: unknown) => void;
  clearAuth: () => void;
  init: () => void;
}

export default (Alpine: Alpine) => {
  Alpine.store("theme", {
    dark:
      localStorage.getItem("theme") === "dark" ||
      (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches),

    toggle() {
      this.dark = !this.dark;
      localStorage.setItem("theme", this.dark ? "dark" : "light");
      document.documentElement.classList.toggle("dark", this.dark);
    },
  } as ThemeStore);

  Alpine.store("auth", {
    token: null as string | null,
    user: null as unknown,

    setAuth(token: string, user: unknown) {
      this.token = token;
      this.user = user;
      localStorage.setItem("token", token);
    },

    clearAuth() {
      this.token = null;
      this.user = null;
      localStorage.removeItem("token");
    },

    init() {
      const token = localStorage.getItem("token");
      if (token) {
        this.token = token;
      }
    },
  } as AuthStore);

  Alpine.store("commandPalette", {
    open: false,
    query: "",
    items: [
      { label: "Ir al Dashboard", href: "/dashboard", icon: "dashboard" },
      { label: "Ver Usuarios", href: "/users", icon: "users" },
      { label: "Configuración", href: "/settings", icon: "settings" },
      { label: "Cerrar Sesión", href: "/logout", icon: "logout" },
    ],
    get filteredItems() {
      if (!this.query) return this.items;
      return this.items.filter((item) =>
        item.label.toLowerCase().includes(this.query.toLowerCase()),
      );
    },
    toggle() {
      this.open = !this.open;
      if (this.open) {
        this.query = "";
        setTimeout(() => {
          document.getElementById("command-input")?.focus();
        }, 50);
      }
    },
  });
};
