import { type } from "arktype";

export const loginSchema = type({
  email: "string.email",
  password: "string.min(8)",
});

export const registerSchema = type({
  email: "string.email",
  password: "string.min(8).regex(/[A-Z]/).regex(/[0-9]/)",
  confirmPassword: "string",
});

export type LoginInput = typeof loginSchema.infer;
export type RegisterInput = typeof registerSchema.infer;
