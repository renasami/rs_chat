import React, { createContext, useEffect } from "react";
import { useAuth, User } from "../hooks/useAuth";
import { useNavigate } from "react-router-dom";

// ユーザーの型
export type AuthContextType = {
  user: User | null;
  isLoading: boolean;
  logout: () => void;
};

// Context を作成
export const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const { user, isLoading, logoutMutation } = useAuth();

  const navigate = useNavigate();

  const logout = async () => {
    await logoutMutation.mutateAsync();
    navigate("/login");
  };

  return (
    <AuthContext.Provider value={{ user, isLoading, logout }}>
      {children}
    </AuthContext.Provider>
  );
};
