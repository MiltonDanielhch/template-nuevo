export type LandingTranslations = {
  nav: {
    features: string;
    register: string;
    login: string;
  };
  hero: {
    badge: string;
    heading: string;
    subheading: string;
    primaryCta: string;
    secondaryCta: string;
  };
  form: {
    title: string;
    subtitle: string;
  };
  features: {
    title: string;
    subtitle: string;
    items: Array<{ icon: string; title: string; description: string }>;
  };
  trust: {
    label: string;
    heading: string;
    items: Array<{ icon: string; title: string; subtitle: string }>;
  };
  footer: {
    title: string;
    subtitle: string;
    privacy: string;
    terms: string;
    contact: string;
  };
};

const TRANSLATIONS: Record<string, LandingTranslations> = {
  es: {
    nav: {
      features: "Características",
      register: "Regístrate",
      login: "Iniciar sesión",
    },
    hero: {
      badge: "Lanzamiento exclusivo",
      heading: "Gestiona usuarios y roles con un stack moderno, seguro y ligero.",
      subheading:
        "Radar de permisos, autenticación basada en tokens y experiencia HTMX para interfaces que responden como apps nativas sin perder SEO.",
      primaryCta: "Iniciar sesión",
      secondaryCta: "Quiero una demo",
    },
    form: {
      title: "Regístrate y recibe acceso anticipado",
      subtitle: "(No spam, solo novedades y lanzamientos)",
    },
    features: {
      title: "Lo que obtienes",
      subtitle:
        "Todo lo necesario para arrancar una plataforma con usuarios, roles y acceso segmentado, listo para escalar.",

      items: [
        {
          icon: "🔒",
          title: "Autenticación segura",
          description:
            "Login con tokens, gestión de sesión y protección RBAC lista para producción.",
        },
        {
          icon: "⚡",
          title: "Interfaz ligera",
          description:
            "HTMX + Astro brindan navegación instantánea sin sacrificar SEO ni accesibilidad.",
        },
        {
          icon: "🧩",
          title: "Escalable y plasmable",
          description: "Arquitectura hexagonal en backend + componentes reutilizables en frontend.",
        },
      ],
    },
    trust: {
      label: "Confían en nosotros",
      heading: "Proyectos reales, resultados reales",
      items: [
        {
          icon: "🏥",
          title: "Salud",
          subtitle: "Gestión de accesos a servicios críticos.",
        },
        {
          icon: "🛡️",
          title: "Fintech",
          subtitle: "Roles y permisos para productos regulados.",
        },
        {
          icon: "📦",
          title: "SaaS",
          subtitle: "Escalando con control y visibilidad.",
        },
      ],
    },
    footer: {
      title: "Laboratorio 3026",
      subtitle: "Construido con Astro, HTMX y Rust.",
      privacy: "Privacidad",
      terms: "Términos",
      contact: "Contacto",
    },
  },
  en: {
    nav: {
      features: "Features",
      register: "Sign up",
      login: "Sign in",
    },
    hero: {
      badge: "Early Access",
      heading: "Manage users and roles with a modern, secure, lightweight stack.",
      subheading:
        "Permission radar, token-based auth and HTMX UX for app-like interactions without sacrificing SEO.",
      primaryCta: "Sign in",
      secondaryCta: "Request a demo",
    },
    form: {
      title: "Sign up to get early access",
      subtitle: "No spam, just updates and launch news.",
    },
    features: {
      title: "What you get",
      subtitle:
        "Everything needed to boot a platform with users, roles and segmented access, ready to scale.",

      items: [
        {
          icon: "🔒",
          title: "Secure authentication",
          description: "Token login, session management and RBAC protection ready for production.",
        },
        {
          icon: "⚡",
          title: "Lightweight UI",
          description:
            "HTMX + Astro deliver instant navigation without sacrificing SEO or accessibility.",
        },
        {
          icon: "🧩",
          title: "Scalable & composable",
          description: "Hexagonal backend architecture + reusable frontend components.",
        },
      ],
    },
    trust: {
      label: "Trusted by",
      heading: "Real projects, real results",
      items: [
        {
          icon: "🏥",
          title: "Healthcare",
          subtitle: "Access control for critical services.",
        },
        {
          icon: "🛡️",
          title: "Fintech",
          subtitle: "Roles and permissions for regulated products.",
        },
        {
          icon: "📦",
          title: "SaaS",
          subtitle: "Scaling with visibility and control.",
        },
      ],
    },
    footer: {
      title: "Laboratorio 3026",
      subtitle: "Built with Astro, HTMX and Rust.",
      privacy: "Privacy",
      terms: "Terms",
      contact: "Contact",
    },
  },
};

export function getLanguage(acceptLanguage: string | null): string {
  if (!acceptLanguage) return "es";
  const [lang] = acceptLanguage.split(",")[0].split("-");
  return lang === "en" ? "en" : "es";
}

export function getLandingTranslations(lang: string) {
  return TRANSLATIONS[lang] ?? TRANSLATIONS.es;
}
