import axios, { AxiosError, AxiosInstance, InternalAxiosRequestConfig } from 'axios';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { jwtDecode } from 'jwt-decode';
import Constants from 'expo-constants';
import type {
  SignUpRequest,
  SignUpResponse,
  SignInRequest,
  SignInResponse,
  GetUserResponse,
  RefreshResponse,
} from '../types';

// 環境変数からAPI URLを取得（デフォルトはローカル開発環境）
const API_BASE_URL = process.env.EXPO_PUBLIC_API_URL || 
  Constants.expoConfig?.extra?.apiUrl || 
  'http://localhost:8080';

// デバッグ用にAPI URLを出力
console.log('API Configuration:', {
  EXPO_PUBLIC_API_URL: process.env.EXPO_PUBLIC_API_URL,
  extraApiUrl: Constants.expoConfig?.extra?.apiUrl,
  API_BASE_URL,
  allEnvVars: Object.keys(process.env).filter(key => key.startsWith('EXPO_'))
});

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
            `${API_BASE_URL}/api/user/refresh`,
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
  console.log('API call: signup', { name, email, baseURL: API_BASE_URL });
  const requestData: SignUpRequest = { name, email, password };
  
  try {
    const response = await api.post<SignUpResponse>('/api/user/signup', requestData);
    console.log('Signup API response:', response.data);

    const { jwt, refresh_token } = response.data;
    await AsyncStorage.setItem('jwt', jwt);
    await AsyncStorage.setItem('refresh_token', refresh_token);

    const userId = getUserIdFromToken(jwt);
    console.log('User ID from token:', userId);
    return { ...response.data, userId: userId || undefined };
  } catch (error: any) {
    console.error('Signup API error:', error);
    console.error('Error response:', error.response?.data);
    throw error;
  }
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
  try {
    await api.post('/api/user/logout');
  } catch (error) {
    console.error('Logout API error:', error);
    // APIエラーでもローカルのトークンは削除する
  } finally {
    await AsyncStorage.removeItem('jwt');
    await AsyncStorage.removeItem('refresh_token');
  }
};

// トークンリフレッシュ
export const refreshToken = async (): Promise<RefreshResponse> => {
  const response = await api.post<RefreshResponse>('/api/user/refresh');
  const { jwt, refresh_token } = response.data;
  await AsyncStorage.setItem('jwt', jwt);
  await AsyncStorage.setItem('refresh_token', refresh_token);
  return response.data;
};

// ゲーム関連の型定義
export interface FacePosition {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface StartGameRequest {
  user_id: string;
  movement_threshold?: number;
  duration_seconds?: number;
}

export interface StartGameResponse {
  session_id: string;
  started_at: string;
  duration_seconds: number;
}

export interface UpdatePositionRequest {
  session_id: string;
  position: FacePosition;
}

export interface UpdatePositionResponse {
  has_moved: boolean;
  game_status: string;
  message?: string;
}

export interface GameStatusResponse {
  session_id: string;
  user_id: string;
  status: string;
  started_at: string;
  ended_at?: string;
  elapsed_seconds: number;
  duration_seconds: number;
}

// ゲーム開始
export const startGame = async (
  userId: string,
  durationSeconds: number = 180
): Promise<StartGameResponse> => {
  console.log('startGame called with:', { userId, durationSeconds });
  const requestData: StartGameRequest = {
    user_id: userId,
    duration_seconds: durationSeconds,
  };
  console.log('Request data:', requestData);
  const response = await api.post<StartGameResponse>('/api/game/start', requestData);
  return response.data;
};

// 顔位置を更新
export const updatePosition = async (
  sessionId: string,
  position: FacePosition
): Promise<UpdatePositionResponse> => {
  const requestData: UpdatePositionRequest = {
    session_id: sessionId,
    position,
  };
  const response = await api.post<UpdatePositionResponse>('/api/game/update-position', requestData);
  return response.data;
};

// ゲーム状態を取得
export const getGameStatus = async (sessionId: string): Promise<GameStatusResponse> => {
  const response = await api.get<GameStatusResponse>(`/api/game/status/${sessionId}`);
  return response.data;
};

// ゲーム終了
export const endGame = async (sessionId: string): Promise<void> => {
  await api.post(`/api/game/end/${sessionId}`);
};

export default api;
