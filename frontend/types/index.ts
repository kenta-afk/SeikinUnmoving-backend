// OpenAPIから自動生成された型をインポート
import type { components, paths } from './api';

export type { components, paths };

// 便利なエイリアス
export type SignUpRequest = components['schemas']['SignUpRequest'];
export type SignUpResponse = components['schemas']['SignUpResponse'];
export type SignInRequest = components['schemas']['SignInRequest'];
export type SignInResponse = components['schemas']['SignInResponse'];
export type GetUserResponse = components['schemas']['GetUserResponse'];
export type RefreshResponse = components['schemas']['RefreshResponse'];

// Userはバックエンドと完全に同期
export type User = GetUserResponse;

// 認証コンテキスト型
export interface AuthContextType {
  user: User | null;
  loading: boolean;
  isAuthenticated: boolean;
  signUp: (name: string, email: string, password: string) => Promise<AuthResult>;
  signIn: (email: string, password: string) => Promise<AuthResult>;
  signOut: () => Promise<void>;
  refreshUser: () => Promise<void>;
}

export interface AuthResult {
  success: boolean;
  error?: string;
  userId?: string;
}

// エラーレスポンス型
export interface ApiError {
  message: string;
  statusCode: number;
}
