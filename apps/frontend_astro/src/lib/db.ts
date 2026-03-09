// Simulación de base de datos en memoria (en un entorno real esto sería SQLite o una API de Rust)
// Nota: En modo desarrollo de Astro, este archivo se mantiene mientras el servidor corre.

export interface User {
  id: string;
  name: string;
  email: string;
  password?: string;
  role: string;
  status: "Activo" | "Inactivo";
}

let users: User[] = [
  {
    id: "1",
    name: "Admin Principal",
    email: "admin@lab3026.com",
    role: "Administrador",
    status: "Activo",
  },
  { id: "2", name: "Juan Pérez", email: "juan@example.com", role: "Usuario", status: "Activo" },
  { id: "3", name: "María García", email: "maria@example.com", role: "Editor", status: "Inactivo" },
  { id: "4", name: "Carlos López", email: "carlos@test.com", role: "Usuario", status: "Activo" },
  { id: "5", name: "Ana Martínez", email: "ana@lab.com", role: "Administrador", status: "Activo" },
];

export const getUsers = () => users;

export const addUser = (user: Omit<User, "id" | "status">) => {
  const newUser: User = {
    ...user,
    id: Math.random().toString(36).substr(2, 9),
    status: "Activo",
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
