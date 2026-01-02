import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';
import * as api from '../services/api';
import { AuthContextType, User, AuthResult } from '../types';

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);

  // アプリ起動時にトークンをチェック
  useEffect(() => {
    checkAuth();
  }, []);

  const checkAuth = async (): Promise<void> => {
    try {
      const token = await AsyncStorage.getItem('jwt');
      if (token) {
        const userData = await api.getUser();
        setUser(userData);
        setIsAuthenticated(true);
      }
    } catch (error) {
      console.error('認証チェックエラー:', error);
      setIsAuthenticated(false);
      setUser(null);
    } finally {
      setLoading(false);
    }
  };

  const signUp = async (
    name: string,
    email: string,
    password: string
  ): Promise<AuthResult> => {
    try {
      const result = await api.signup(name, email, password);
      // バックグラウンドでユーザー情報を取得
      api.getUser().then(userData => {
        setUser(userData);
        setIsAuthenticated(true);
      });
      return { success: true, userId: result.userId };
    } catch (error: any) {
      console.error('サインアップエラー:', error);
      return {
        success: false,
        error: error.response?.data?.message || 'サインアップに失敗しました',
      };
    }
  };

  const signIn = async (email: string, password: string): Promise<AuthResult> => {
    try {
      const result = await api.signin(email, password);
      // バックグラウンドでユーザー情報を取得
      api.getUser().then(userData => {
        setUser(userData);
        setIsAuthenticated(true);
      });
      return { success: true, userId: result.userId };
    } catch (error: any) {
      console.error('サインインエラー:', error);
      return {
        success: false,
        error: error.response?.data?.message || 'サインインに失敗しました',
      };
    }
  };

  const signOut = async (): Promise<void> => {
    try {
      await api.logout();
      setUser(null);
      setIsAuthenticated(false);
    } catch (error) {
      console.error('ログアウトエラー:', error);
    }
  };

  const refreshUser = async (): Promise<void> => {
    try {
      const userData = await api.getUser();
      setUser(userData);
    } catch (error) {
      console.error('ユーザー情報更新エラー:', error);
    }
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        loading,
        isAuthenticated,
        signUp,
        signIn,
        signOut,
        refreshUser,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
