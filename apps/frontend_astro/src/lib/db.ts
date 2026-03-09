// Simulación de base de datos en memoria (en un entorno real esto sería SQLite o una API de Rust)
// Nota: En modo desarrollo de Astro, este archivo se mantiene mientras el servidor corre.

export interface User {
  id: string;
  username: string;
  email: string;
  password_hash: string;
  role: string;
  status: "Activo" | "Inactivo";
  created_at: string;
}

// Datos reales del Seed de la base de datos Rust
let users: User[] = [
  {
    id: "user_00000001",
    username: "admin_3026",
    email: "admin@lab3026.com",
    password_hash: "$argon2id$v=19$m=19456,t=2,p=1$...", // Simulado
    role: "Admin",
    status: "Activo",
    created_at: new Date().toISOString(),
  },
];

export const getUsers = () => users;

export const addUser = (userData: Omit<User, "id" | "status" | "created_at">) => {
  const newUser: User = {
    ...userData,
    id: `user_${Math.random().toString(36).substr(2, 8)}`,
    status: "Activo",
    created_at: new Date().toISOString(),
  };
  users = [newUser, ...users];
  return newUser;
};

export const deleteUser = (id: string) => {
  users = users.filter((u) => u.id !== id);
  return true;
};

export const updateUser = (id: string, data: Partial<User>) => {
  users = users.map((u) => (u.id === id ? { ...u, ...data } : u));
  return users.find((u) => u.id === id);
};
