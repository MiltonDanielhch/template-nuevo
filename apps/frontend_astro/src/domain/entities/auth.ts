export interface User {
  id: string;
  email: string;
  username?: string;
  avatar_url?: string;
  email_verified: boolean;
  created_at: string;
  updated_at: string;
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
