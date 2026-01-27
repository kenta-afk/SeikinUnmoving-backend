import axios, { AxiosError, AxiosInstance, InternalAxiosRequestConfig } from 'axios';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { jwtDecode } from 'jwt-decode';
import type { components, paths } from '../types/api';

// OpenAPIから生成された型を使用
type SignUpRequest = components['schemas']['SignUpRequest'];
type SignUpResponse = components['schemas']['SignUpResponse'];
type SignInRequest = components['schemas']['SignInRequest'];
type SignInResponse = components['schemas']['SignInResponse'];
type GetUserResponse = components['schemas']['GetUserResponse'];
type RefreshResponse = components['schemas']['RefreshResponse'];

const API_BASE_URL = 'http://localhost:8080';

const api: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  withCredentials: true,
  headers: {
    'Content-Type': 'application/json',
  },
});

// リクエストインターセプター: JWTトークンを追加
api.interceptors.request.use(
  async (config: InternalAxiosRequestConfig) => {
    const token = await AsyncStorage.getItem('jwt');
    if (token && config.headers) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error: AxiosError) => {
    return Promise.reject(error);
  }
);

// レスポンスインターセプター: トークンの自動リフレッシュ
api.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    const originalRequest = error.config as InternalAxiosRequestConfig & {
      _retry?: boolean;
    };

    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;

      try {
        const refreshToken = await AsyncStorage.getItem('refresh_token');
        if (refreshToken) {
          const response = await axios.post<RefreshResponse>(
            `${API_BASE_URL}/refresh`,
            {},
            {
              withCredentials: true,
              headers: {
                'Content-Type': 'application/json',
              },
            }
          );

          const { jwt, refresh_token } = response.data;
          await AsyncStorage.setItem('jwt', jwt);
          await AsyncStorage.setItem('refresh_token', refresh_token);

          if (originalRequest.headers) {
            originalRequest.headers.Authorization = `Bearer ${jwt}`;
          }
          return api(originalRequest);
        }
      } catch (refreshError) {
        await AsyncStorage.removeItem('jwt');
        await AsyncStorage.removeItem('refresh_token');
        return Promise.reject(refreshError);
      }
    }

    return Promise.reject(error);
  }
);

// JWTからユーザーIDを取得
export const getUserIdFromToken = (token: string): string | null => {
  try {
    const decoded: any = jwtDecode(token);
    return decoded.sub || null;
  } catch (error) {
    console.error('JWT decode error:', error);
    return null;
  }
};

// サインアップ
export const signup = async (
  name: string,
  email: string,
  password: string
): Promise<SignUpResponse & { userId?: string }> => {
  const requestData: SignUpRequest = { name, email, password };
  const response = await api.post<SignUpResponse>('/api/user/signup', requestData);

  const { jwt, refresh_token } = response.data;
  await AsyncStorage.setItem('jwt', jwt);
  await AsyncStorage.setItem('refresh_token', refresh_token);

  const userId = getUserIdFromToken(jwt);
  return { ...response.data, userId: userId || undefined };
};

// サインイン
export const signin = async (
  email: string,
  password: string
): Promise<SignInResponse & { userId?: string }> => {
  const requestData: SignInRequest = { email, password };
  const response = await api.post<SignInResponse>('/api/user/signin', requestData);

  const { jwt, refresh_token } = response.data;
  await AsyncStorage.setItem('jwt', jwt);
  await AsyncStorage.setItem('refresh_token', refresh_token);

  const userId = getUserIdFromToken(jwt);
  return { ...response.data, userId: userId || undefined };
};

// ユーザー情報取得
export const getUser = async (): Promise<GetUserResponse> => {
  const response = await api.get<GetUserResponse>('/api/user/me');
  return response.data;
};

// ログアウト
export const logout = async (): Promise<void> => {
  await AsyncStorage.removeItem('jwt');
  await AsyncStorage.removeItem('refresh_token');
};

// トークンリフレッシュ
export const refreshToken = async (): Promise<RefreshResponse> => {
  const response = await api.post<RefreshResponse>('/api/user/refresh');
  const { jwt, refresh_token } = response.data;
  await AsyncStorage.setItem('jwt', jwt);
  await AsyncStorage.setItem('refresh_token', refresh_token);
  return response.data;
};

export default api;

// 型のエクスポート
export type {
  SignUpRequest,
  SignUpResponse,
  SignInRequest,
  SignInResponse,
  GetUserResponse,
  RefreshResponse,
};
