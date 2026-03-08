export interface User {
  id: string;
  email: string;
  createdAt: string;
}

export interface Session {
  token: string;
  expiresAt: string;
}

export interface AuthResponse {
  user: User;
  token: string;
}

export interface ApiError {
  error: string;
  message?: string;
}
