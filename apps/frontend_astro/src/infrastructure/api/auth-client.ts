import type { ApiError, AuthResponse } from "../../domain/entities/auth";

const API_BASE = "/api/auth";

class AuthClient {
  private token: string | null = null;

  constructor() {
    if (typeof window !== "undefined") {
      this.token = localStorage.getItem("auth_token");
    }
  }

  private async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const url = `${API_BASE}${endpoint}`;
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
      ...(options.headers as Record<string, string>),
    };

    if (this.token) {
      headers.Authorization = `Bearer ${this.token}`;
    }

    const response = await fetch(url, {
      ...options,
      headers,
    });

    if (!response.ok) {
      const error: ApiError = await response.json().catch(() => ({
        error: "Error desconocido",
        message: response.statusText,
      }));
      throw error;
    }

    return response.json();
  }

  async login(email: string, password: string): Promise<AuthResponse> {
    const data = await this.request<AuthResponse>("/login", {
      method: "POST",
      body: JSON.stringify({ email, password }),
    });

    this.token = data.token;
    if (typeof window !== "undefined") {
      localStorage.setItem("auth_token", data.token);
    }

    return data;
  }

  async register(email: string, password: string): Promise<AuthResponse> {
    const data = await this.request<AuthResponse>("/register", {
      method: "POST",
      body: JSON.stringify({ email, password }),
    });

    this.token = data.token;
    if (typeof window !== "undefined") {
      localStorage.setItem("auth_token", data.token);
    }

    return data;
  }

  async logout(): Promise<void> {
    try {
      await this.request("/logout", { method: "POST" });
    } finally {
      this.token = null;
      if (typeof window !== "undefined") {
        localStorage.removeItem("auth_token");
      }
    }
  }

  async getCurrentUser(): Promise<AuthResponse["user"]> {
    return this.request<AuthResponse["user"]>("/me");
  }

  isAuthenticated(): boolean {
    return this.token !== null;
  }
}

export const authClient = new AuthClient();
