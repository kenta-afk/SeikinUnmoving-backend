import React, { useState, useEffect, useRef } from 'react';
import {
  StyleSheet,
  View,
  Text,
  TouchableOpacity,
  Alert,
  Dimensions,
  ActivityIndicator,
} from 'react-native';
import { CameraView, useCameraPermissions } from 'expo-camera';
import { Accelerometer } from 'expo-sensors';
import { useAuth } from '../context/AuthContext';
import { startGame, updatePosition, endGame, FacePosition } from '../services/api';

const { width: SCREEN_WIDTH, height: SCREEN_HEIGHT } = Dimensions.get('window');

export default function GameScreen() {
  const { user } = useAuth();
  const [permission, requestPermission] = useCameraPermissions();
  const [gameStarted, setGameStarted] = useState(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [gameStatus, setGameStatus] = useState<'idle' | 'active' | 'failed' | 'success'>('idle');
  const [message, setMessage] = useState<string>('');
  const [elapsedTime, setElapsedTime] = useState(0);
  const [duration, setDuration] = useState(30);
  const [isProcessing, setIsProcessing] = useState(false);
  
  const lastUpdateRef = useRef<number>(0);
  const timerRef = useRef<number | null>(null);
  const gameEndTimeRef = useRef<number | null>(null);
  const accelerometerSubscription = useRef<any>(null);
  const baseAcceleration = useRef<{ x: number; y: number; z: number } | null>(null);
  const currentPosition = useRef<FacePosition>({ x: SCREEN_WIDTH / 2, y: SCREEN_HEIGHT / 2, width: 100, height: 100 });

  useEffect(() => {
    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current);
      }
      if (accelerometerSubscription.current) {
        accelerometerSubscription.current.remove();
      }
    };
  }, []);

  // カメラパーミッションをリクエスト
  const handleRequestPermission = async () => {
    const result = await requestPermission();
    if (!result.granted) {
      Alert.alert('カメラの許可が必要です', 'ゲームをプレイするにはカメラへのアクセスを許可してください。');
    }
  };

  // ゲーム開始
  const handleStartGame = async () => {
    if (!user?.user_id) {
      Alert.alert('エラー', 'ユーザー情報が見つかりません');
      return;
    }

    try {
      setIsProcessing(true);
      const response = await startGame(user.user_id, 50, 30);
      setSessionId(response.session_id);
      setDuration(response.duration_seconds);
      setGameStarted(true);
      setGameStatus('active');
      setMessage('動かないでください！');
      setElapsedTime(0);
      
      gameEndTimeRef.current = Date.now() + response.duration_seconds * 1000;
      
      // 加速度センサーを開始
      Accelerometer.setUpdateInterval(100);
      accelerometerSubscription.current = Accelerometer.addListener(accelerometerData => {
        if (!baseAcceleration.current) {
          baseAcceleration.current = accelerometerData;
        } else {
          // 加速度の変化を計算（動き検出）
          const deltaX = Math.abs(accelerometerData.x - baseAcceleration.current.x);
          const deltaY = Math.abs(accelerometerData.y - baseAcceleration.current.y);
          const deltaZ = Math.abs(accelerometerData.z - baseAcceleration.current.z);
          const totalDelta = deltaX + deltaY + deltaZ;

          // 加速度変化を位置に変換（簡易的）
          if (totalDelta > 0.1) {
            const scaleFactor = 100;
            currentPosition.current = {
              x: currentPosition.current.x + deltaX * scaleFactor,
              y: currentPosition.current.y + deltaY * scaleFactor,
              width: 100,
              height: 100,
            };
          }
        }
      });
      
      // タイマーを開始
      timerRef.current = setInterval(() => {
        if (gameEndTimeRef.current) {
          const remaining = Math.max(0, Math.floor((gameEndTimeRef.current - Date.now()) / 1000));
          const elapsed = response.duration_seconds - remaining;
          setElapsedTime(elapsed);
          
          if (remaining === 0) {
            handleGameSuccess();
          } else {
            // 2秒ごとに位置を送信
            const now = Date.now();
            if (now - lastUpdateRef.current >= 2000) {
              lastUpdateRef.current = now;
              updatePositionData(response.session_id);
            }
          }
        }
      }, 100);
    } catch (error: any) {
      console.error('ゲーム開始エラー:', error);
      Alert.alert('エラー', error.response?.data?.error || 'ゲームを開始できませんでした');
    } finally {
      setIsProcessing(false);
    }
  };

  // 位置データを更新
  const updatePositionData = async (sid: string) => {
    if (!gameStarted || gameStatus !== 'active') {
      return;
    }

    try {
      const response = await updatePosition(sid, currentPosition.current);
      
      if (response.has_moved) {
        handleGameFailed();
      } else if (response.game_status === 'success') {
        handleGameSuccess();
      }
    } catch (error) {
      console.error('位置更新エラー:', error);
    }
  };

  // ゲーム成功
  const handleGameSuccess = () => {
    if (timerRef.current) {
      clearInterval(timerRef.current);
      timerRef.current = null;
    }
    if (accelerometerSubscription.current) {
      accelerometerSubscription.current.remove();
      accelerometerSubscription.current = null;
    }
    setGameStatus('success');
    setMessage('成功！動かずにいられました！');
    setGameStarted(false);
    
    if (sessionId) {
      endGame(sessionId).catch(console.error);
    }
  };

  // ゲーム失敗
  const handleGameFailed = () => {
    if (timerRef.current) {
      clearInterval(timerRef.current);
      timerRef.current = null;
    }
    if (accelerometerSubscription.current) {
      accelerometerSubscription.current.remove();
      accelerometerSubscription.current = null;
    }
    setGameStatus('failed');
    setMessage('動いてしまいました！');
    setGameStarted(false);
    
    if (sessionId) {
      endGame(sessionId).catch(console.error);
    }
  };

  // 顔検出時のハンドラー（使用しない）
  // const handleFacesDetected = async ({ faces }: FaceDetector.FaceDetectorResult) => {};

  // リセット
  const handleReset = () => {
    setGameStatus('idle');
    setMessage('');
    setSessionId(null);
    setElapsedTime(0);
    baseAcceleration.current = null;
    currentPosition.current = { x: SCREEN_WIDTH / 2, y: SCREEN_HEIGHT / 2, width: 100, height: 100 };
    if (timerRef.current) {
      clearInterval(timerRef.current);
      timerRef.current = null;
    }
    if (accelerometerSubscription.current) {
      accelerometerSubscription.current.remove();
      accelerometerSubscription.current = null;
    }
  };

  if (!permission) {
    return (
      <View style={styles.container}>
        <ActivityIndicator size="large" />
      </View>
    );
  }

  if (!permission.granted) {
    return (
      <View style={styles.container}>
        <Text style={styles.title}>動かないゲーム</Text>
        <Text style={styles.text}>カメラの許可が必要です</Text>
        <TouchableOpacity style={styles.button} onPress={handleRequestPermission}>
          <Text style={styles.buttonText}>カメラを許可</Text>
        </TouchableOpacity>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <Text style={styles.title}>動かないゲーム</Text>
      
      {gameStarted && (
        <View style={styles.timerContainer}>
          <Text style={styles.timerText}>
            残り時間: {duration - elapsedTime}秒
          </Text>
        </View>
      )}

      {message && (
        <View style={[
          styles.messageContainer,
          gameStatus === 'success' && styles.messageSuccess,
          gameStatus === 'failed' && styles.messageFailed,
        ]}>
          <Text style={styles.messageText}>{message}</Text>
        </View>
      )}

      <View style={styles.cameraContainer}>
        <CameraView
          style={styles.camera}
          facing="front"
        >
          {gameStarted && (
            <View style={styles.overlay}>
              <Text style={styles.overlayText}>動かないで！</Text>
            </View>
          )}
        </CameraView>
      </View>

      <View style={styles.controls}>
        {!gameStarted && gameStatus === 'idle' && (
          <TouchableOpacity
            style={[styles.button, styles.startButton]}
            onPress={handleStartGame}
            disabled={isProcessing}
          >
            {isProcessing ? (
              <ActivityIndicator color="#fff" />
            ) : (
              <Text style={styles.buttonText}>ゲーム開始</Text>
            )}
          </TouchableOpacity>
        )}

        {!gameStarted && (gameStatus === 'success' || gameStatus === 'failed') && (
          <TouchableOpacity
            style={[styles.button, styles.resetButton]}
            onPress={handleReset}
          >
            <Text style={styles.buttonText}>もう一度</Text>
          </TouchableOpacity>
        )}
      </View>

      <View style={styles.instructions}>
        <Text style={styles.instructionText}>
          📱 スマホを手に持って動かさないようにしてください
        </Text>
        <Text style={styles.instructionText}>
          ⏱️ 30秒間動かずにいられたら成功です
        </Text>
        <Text style={styles.instructionText}>
          🔧 加速度センサーで動きを検出します
        </Text>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
    padding: 20,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    textAlign: 'center',
    marginTop: 40,
    marginBottom: 20,
    color: '#333',
  },
  text: {
    fontSize: 16,
    textAlign: 'center',
    marginBottom: 20,
    color: '#666',
  },
  timerContainer: {
    backgroundColor: '#fff',
    padding: 15,
    borderRadius: 10,
    marginBottom: 15,
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  timerText: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#333',
  },
  messageContainer: {
    padding: 15,
    borderRadius: 10,
    marginBottom: 15,
    alignItems: 'center',
  },
  messageSuccess: {
    backgroundColor: '#4CAF50',
  },
  messageFailed: {
    backgroundColor: '#f44336',
  },
  messageText: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#fff',
  },
  cameraContainer: {
    flex: 1,
    borderRadius: 20,
    overflow: 'hidden',
    marginBottom: 20,
    backgroundColor: '#000',
  },
  camera: {
    flex: 1,
  },
  overlay: {
    flex: 1,
    backgroundColor: 'transparent',
    justifyContent: 'center',
    alignItems: 'center',
  },
  overlayText: {
    fontSize: 32,
    fontWeight: 'bold',
    color: '#fff',
    textShadowColor: 'rgba(0, 0, 0, 0.75)',
    textShadowOffset: { width: -1, height: 1 },
    textShadowRadius: 10,
  },
  controls: {
    marginBottom: 20,
  },
  button: {
    backgroundColor: '#007AFF',
    padding: 18,
    borderRadius: 12,
    alignItems: 'center',
    justifyContent: 'center',
    minHeight: 56,
  },
  startButton: {
    backgroundColor: '#4CAF50',
  },
  resetButton: {
    backgroundColor: '#2196F3',
  },
  buttonText: {
    color: '#fff',
    fontSize: 18,
    fontWeight: 'bold',
  },
  instructions: {
    backgroundColor: '#fff',
    padding: 15,
    borderRadius: 10,
    marginBottom: 20,
  },
  instructionText: {
    fontSize: 14,
    color: '#666',
    marginBottom: 8,
    lineHeight: 20,
  },
});
