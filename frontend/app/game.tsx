import React, { useState, useRef, useEffect } from 'react';
import { View, Text, StyleSheet, Platform } from 'react-native';
import { useRouter } from 'expo-router';
import { useAuth } from '../context/AuthContext';
import { startGame, endGame, getAllVideos, Video } from '../services/api';

export default function GameScreen() {
  const router = useRouter();
  const { user } = useAuth();
  const [gameState, setGameState] = useState<'idle' | 'playing' | 'success' | 'failed'>('idle');
  const [timeRemaining, setTimeRemaining] = useState(160);
  const [gameId, setGameId] = useState<string | null>(null);
  const [isSmiling, setIsSmiling] = useState(false);
  const [currentVideo, setCurrentVideo] = useState<Video | null>(null);
  const [allVideos, setAllVideos] = useState<Video[]>([]);
  const [currentVideoIndex, setCurrentVideoIndex] = useState(0);
  const [isLoadingVideo, setIsLoadingVideo] = useState(false);
  
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const youtubePlayerRef = useRef<any>(null);
  const iframeRef = useRef<HTMLIFrameElement>(null);
  const faceMeshRef = useRef<any>(null);
  const cameraRef = useRef<any>(null);
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const gameStateRef = useRef<'idle' | 'playing' | 'success' | 'failed'>('idle'); // gameStateの最新値を保持
  const gameEndingRef = useRef<boolean>(false); // ゲーム終了処理中フラグ
  const eyesClosedStartTimeRef = useRef<number | null>(null); // 目が細い状態が始まった時刻
  const eyeHeightHistoryRef = useRef<number[]>([]); // 目の高さ履歴（類似度計算用）
  const gameIdRef = useRef<string | null>(null); // gameIdをrefでも保持（同期的にアクセス可能）

  // 次の動画に切り替え
  const handleVideoEnd = () => {
    if (allVideos.length === 0 || gameState !== 'playing') return;
    
    const nextIndex = (currentVideoIndex + 1) % allVideos.length;
    console.log(`動画終了: ${currentVideoIndex} -> ${nextIndex}`);
    setCurrentVideoIndex(nextIndex);
    setCurrentVideo(allVideos[nextIndex]);
  };

  // YouTube Player APIのロード
  useEffect(() => {
    if (Platform.OS !== 'web') return;

    // YouTube IFrame APIスクリプトをロード
    const tag = document.createElement('script');
    tag.src = 'https://www.youtube.com/iframe_api';
    const firstScriptTag = document.getElementsByTagName('script')[0];
    firstScriptTag?.parentNode?.insertBefore(tag, firstScriptTag);

    // APIの準備完了時のコールバック
    (window as any).onYouTubeIframeAPIReady = () => {
      console.log('YouTube IFrame API is ready');
    };
  }, []);

  // YouTube Playerの初期化（動画が変わった時）
  useEffect(() => {
    if (Platform.OS !== 'web' || !currentVideo || gameState !== 'playing') return;

    const initYouTubePlayer = () => {
      if (!(window as any).YT || !(window as any).YT.Player) {
        console.log('Waiting for YouTube API...');
        setTimeout(initYouTubePlayer, 500);
        return;
      }

      const videoId = extractYouTubeId(currentVideo.youtube_url);
      
      // 既存のプレイヤーがあれば破棄
      if (youtubePlayerRef.current) {
        youtubePlayerRef.current.destroy();
      }

      console.log('Creating YouTube player for video:', videoId);
      youtubePlayerRef.current = new (window as any).YT.Player('youtube-player', {
        videoId: videoId,
        playerVars: {
          autoplay: 1,
          controls: 0,
          loop: 0,
          mute: 0,
          modestbranding: 1,
          rel: 0,
        },
        events: {
          onReady: (event: any) => {
            // 音量を30%に設定
            event.target.setVolume(15);
            console.log('YouTube player ready, volume set to 30%');
          },
          onStateChange: (event: any) => {
            // 動画終了時
            if (event.data === (window as any).YT.PlayerState.ENDED) {
              console.log('動画終了イベント検出');
              handleVideoEnd();
            }
          },
        },
      });
    };

    initYouTubePlayer();

    return () => {
      if (youtubePlayerRef.current) {
        youtubePlayerRef.current.destroy();
        youtubePlayerRef.current = null;
      }
    };
  }, [currentVideo, gameState]);

  // gameStateが変更されたらrefも更新
  useEffect(() => {
    gameStateRef.current = gameState;
    console.log('🔄 gameState更新:', gameState);
  }, [gameState]);

  // MediaPipeスクリプトのロード
  useEffect(() => {
    if (Platform.OS !== 'web') return;

    const loadMediaPipe = async () => {
      // 既に読み込まれている場合はスキップ
      if ((window as any).FaceMesh) {
        console.log('MediaPipe already loaded');
        return;
      }

      // スクリプトの追加（順番に読み込む）- バージョンを0.4に固定
      const scripts = [
        'https://cdn.jsdelivr.net/npm/@mediapipe/camera_utils@0.3/camera_utils.js',
        'https://cdn.jsdelivr.net/npm/@mediapipe/control_utils@0.6/control_utils.js',
        'https://cdn.jsdelivr.net/npm/@mediapipe/drawing_utils@0.3/drawing_utils.js',
        'https://cdn.jsdelivr.net/npm/@mediapipe/face_mesh@0.4/face_mesh.js'
      ];

      for (const src of scripts) {
        await new Promise<void>((resolve, reject) => {
          const script = document.createElement('script');
          script.src = src;
          script.crossOrigin = 'anonymous';
          script.onload = () => {
            console.log(`Loaded: ${src}`);
            resolve();
          };
          script.onerror = () => {
            console.error(`Failed to load script: ${src}`);
            reject(new Error(`Failed to load ${src}`));
          };
          document.head.appendChild(script);
        });
      }
      
      console.log('All MediaPipe scripts loaded successfully');
    };

    loadMediaPipe().catch(err => {
      console.error('Failed to load MediaPipe:', err);
    });
  }, []);

  // YouTube動画URLからVideo IDを抽出
  const extractYouTubeId = (url: string): string | null => {
    const patterns = [
      /(?:youtube\.com\/watch\?v=|youtu\.be\/|youtube\.com\/shorts\/)([^&?/]+)/,
      /youtube\.com\/embed\/([^&?/]+)/
    ];
    
    for (const pattern of patterns) {
      const match = url.match(pattern);
      if (match) return match[1];
    }
    return null;
  };

  // カメラとFace Meshの初期化（ゲーム開始時に呼ばれる）
  const initializeFaceMesh = async () => {
    if (Platform.OS !== 'web') {
      console.error('This game only works on web browsers');
      return false;
    }
      try {
        console.log('Initializing MediaPipe Face Mesh...');
        
        // MediaPipeがロードされているか確認（最大10秒待機）
        const FaceMesh = (window as any).FaceMesh;
        const Camera = (window as any).Camera;
        
        if (!FaceMesh || !Camera) {
          console.log('Waiting for MediaPipe to load...');
          // スクリプトのロードを待つ
          let retries = 0;
          while (retries < 20) {
            await new Promise(resolve => setTimeout(resolve, 500));
            if ((window as any).FaceMesh && (window as any).Camera) {
              console.log('MediaPipe loaded after waiting');
              break;
            }
            retries++;
          }
          
          if (!(window as any).FaceMesh || !(window as any).Camera) {
            alert('MediaPipeライブラリの読み込みに失敗しました。ページをリロードしてください。');
            return false;
          }
        }

        const faceMesh = new (window as any).FaceMesh({
          locateFile: (file: string) => {
            return `https://cdn.jsdelivr.net/npm/@mediapipe/face_mesh/${file}`;
          }
        });

        faceMesh.setOptions({
          maxNumFaces: 1,
          refineLandmarks: true,
          minDetectionConfidence: 0.5,
          minTrackingConfidence: 0.5
        });

        faceMesh.onResults((results: any) => {
          if (!canvasRef.current || !videoRef.current) return;

          const canvasCtx = canvasRef.current.getContext('2d');
          if (!canvasCtx) return;

          // Clear canvas
          canvasCtx.clearRect(0, 0, canvasRef.current.width, canvasRef.current.height);
          
          // Draw video frame
          canvasCtx.drawImage(videoRef.current, 0, 0, canvasRef.current.width, canvasRef.current.height);

          // 顔のランドマークが検出された場合
          if (results.multiFaceLandmarks && results.multiFaceLandmarks.length > 0) {
            const landmarks = results.multiFaceLandmarks[0];
            
            // 笑顔判定：目が細いかどうかで判定
            // 目のランドマーク
            const leftEyeTop = landmarks[159];    // 左目上
            const leftEyeBottom = landmarks[145]; // 左目下
            const rightEyeTop = landmarks[386];   // 右目上
            const rightEyeBottom = landmarks[374];// 右目下
            
            // 目の高さを計算
            const leftEyeHeight = Math.abs(leftEyeBottom.y - leftEyeTop.y);
            const rightEyeHeight = Math.abs(rightEyeBottom.y - rightEyeTop.y);
            const avgEyeHeight = (leftEyeHeight + rightEyeHeight) / 2;
            
            // ゲーム中は目の高さを記録（類似度計算用）
            if (gameStateRef.current === 'playing') {
              eyeHeightHistoryRef.current.push(avgEyeHeight);
              // 最大1000サンプルまで保持（約30秒分）
              if (eyeHeightHistoryRef.current.length > 1000) {
                eyeHeightHistoryRef.current.shift();
              }
            }
            
            // 目が細いかどうかを判定
            const eyesClosed = avgEyeHeight < 0.015;
            const currentTime = Date.now();
            
            // 目が細い状態が続いているかをチェック
            if (eyesClosed) {
              if (eyesClosedStartTimeRef.current === null) {
                // 目が細い状態が始まった
                eyesClosedStartTimeRef.current = currentTime;
              }
            } else {
              // 目が開いたのでタイマーをリセット
              eyesClosedStartTimeRef.current = null;
            }
            
            // 目が細い状態が0.5秒間続いたかを判定
            const eyesClosedDuration = eyesClosedStartTimeRef.current !== null 
              ? (currentTime - eyesClosedStartTimeRef.current) / 1000 
              : 0;
            const isSmiling = eyesClosedDuration >= 0.5;
            
            console.log('笑顔判定:', { 
              leftEyeHeight: leftEyeHeight.toFixed(4),
              rightEyeHeight: rightEyeHeight.toFixed(4),
              avgEyeHeight: avgEyeHeight.toFixed(4), 
              threshold: 0.015,
              eyesClosed,
              eyesClosedDuration: eyesClosedDuration.toFixed(2),
              isSmiling 
            });
            
            // ランドマークを描画
            canvasCtx.fillStyle = '#00FF00';
            landmarks.forEach((landmark: any) => {
              canvasCtx.beginPath();
              canvasCtx.arc(
                landmark.x * canvasRef.current!.width,
                landmark.y * canvasRef.current!.height,
                1,
                0,
                2 * Math.PI
              );
              canvasCtx.fill();
            });
            
            // デバッグ情報と口角を強調表示
            canvasCtx.fillStyle = isSmiling ? '#FF0000' : (eyesClosed ? '#FFA500' : '#00FF00');
            canvasCtx.font = '20px Arial';
            canvasCtx.fillText(
              isSmiling ? '😊 笑顔検出!' : (eyesClosed ? `⚠️ 目が細い (${eyesClosedDuration.toFixed(1)}s)` : '😐 真顔'),
              10,
              30
            );
            
            // デバッグ情報を表示
            canvasCtx.fillStyle = '#FFFF00';
            canvasCtx.font = '14px Arial';
            canvasCtx.fillText(
              `左目高さ: ${leftEyeHeight.toFixed(4)}`,
              10,
              60
            );
            canvasCtx.fillText(
              `右目高さ: ${rightEyeHeight.toFixed(4)}`,
              10,
              80
            );
            canvasCtx.fillText(
              `平均: ${avgEyeHeight.toFixed(4)} (閾値: 0.015)`,
              10,
              100
            );
            canvasCtx.fillText(
              `目が細い継続時間: ${eyesClosedDuration.toFixed(1)}s / 0.5s`,
              10,
              120
            );
            canvasCtx.fillText(
              `ゲーム状態: ${gameState}`,
              10,
              140
            );
            
            setIsSmiling(isSmiling);
            
            // デバッグ: 毎フレームの状態を出力（1秒に1回程度）
            if (Math.random() < 0.1) {
              console.log('顔検出状態:', { 
                isSmiling, 
                gameState: gameStateRef.current, 
                avgEyeHeight: avgEyeHeight.toFixed(4),
                gameEnding: gameEndingRef.current
              });
            }
            
            // 笑顔が検出されたら失敗（refの値を使用）
            if (isSmiling && gameStateRef.current === 'playing' && !gameEndingRef.current) {
              console.log('🚨 笑顔検出！ゲームオーバー', { 
                avgEyeHeight,
                threshold: 0.015,
                gameState: gameStateRef.current
              });
              gameEndingRef.current = true; // 重複防止
              handleGameEnd('failed');
            }
          } else {
            // 顔が検出されない = 失敗
            setIsSmiling(false);
            eyesClosedStartTimeRef.current = null; // タイマーをリセット
            
            if (gameStateRef.current === 'playing' && !gameEndingRef.current) {
              console.log('顔消失！ゲームオーバー', { gameState: gameStateRef.current });
              gameEndingRef.current = true; // 重複防止
              handleGameEnd('failed');
            }
          }
        });

        faceMeshRef.current = faceMesh;

        // カメラの初期化
        if (videoRef.current) {
          const Camera = (window as any).Camera;
          cameraRef.current = new Camera(videoRef.current, {
            onFrame: async () => {
              if (videoRef.current && faceMeshRef.current) {
                await faceMeshRef.current.send({ image: videoRef.current });
              }
            },
            width: 640,
            height: 480
          });
          
          console.log('Starting camera...');
          await cameraRef.current.start();
          console.log('Camera started successfully');
        }
        
        return true;
      } catch (error) {
        console.error('Failed to initialize face mesh:', error);
        alert(`カメラの初期化に失敗しました: ${error}`);
        return false;
      }
    };

  // クリーンアップ
  useEffect(() => {
    return () => {
      if (cameraRef.current) {
        cameraRef.current.stop();
      }
      if (faceMeshRef.current) {
        faceMeshRef.current.close();
      }
      if (timerRef.current) {
        clearInterval(timerRef.current);
      }
    };
  }, []);

  // ゲーム開始
  const handleGameStart = async () => {
    if (!user) {
      console.error('User not authenticated');
      return;
    }

    console.log('Starting game with user:', user);
    console.log('User ID:', user.user_id);

    try {
      // 全動画を取得
      setIsLoadingVideo(true);
      const videos = await getAllVideos();
      console.log('All videos loaded:', videos);
      
      if (videos.length === 0) {
        alert('動画が登録されていません。backend/scripts/add-videos.shで動画を追加してください。');
        setIsLoadingVideo(false);
        return;
      }
      
      setAllVideos(videos);
      setCurrentVideoIndex(0);
      setCurrentVideo(videos[0]);
      setIsLoadingVideo(false);

      // カメラとMediaPipeを初期化
      const initialized = await initializeFaceMesh();
      if (!initialized) {
        alert('カメラの初期化に失敗しました');
        return;
      }

      const response = await startGame(user.user_id, 160);
      console.log('Game started:', response);
      console.log('Setting gameId to:', response.session_id);
      setGameId(response.session_id);
      gameIdRef.current = response.session_id; // refにも保存
      console.log('🎮 ゲーム状態を playing に変更');
      gameEndingRef.current = false; // フラグリセット
      setGameState('playing');
      gameStateRef.current = 'playing';
      setTimeRemaining(160);
      startTimer();
    } catch (error) {
      console.error('Failed to start game:', error);
      setIsLoadingVideo(false);
      alert(error instanceof Error && error.message.includes('404') 
        ? '動画が登録されていません。backend/scripts/add-videos.shで動画を追加してください。'
        : 'ゲームの開始に失敗しました');
    }
  };

  // タイマー開始
  const startTimer = () => {
    if (timerRef.current) {
      clearInterval(timerRef.current);
    }

    timerRef.current = setInterval(() => {
      setTimeRemaining((prev) => {
        if (prev <= 1) {
          handleGameEnd('success');
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
  };

  // セイキン類似度を計算
  const calculateSeikinSimilarity = (): number => {
    const history = eyeHeightHistoryRef.current;
    if (history.length === 0) return 0;
    
    // 目が細い（笑顔）の基準値: 0.015
    const smileThreshold = 0.015;
    
    // 平均目の高さを計算
    const avgEyeHeight = history.reduce((sum, val) => sum + val, 0) / history.length;
    
    // 類似度計算: 目が細いほど高い類似度
    // 0.015以下なら100%、0.030以上なら0%、その間は線形補間
    const similarity = Math.max(0, Math.min(100, (1 - (avgEyeHeight - smileThreshold) / 0.015) * 100));
    
    // 目が細い時間の割合も考慮
    const smileTimeRatio = history.filter(h => h < smileThreshold).length / history.length;
    
    // 最終類似度: 平均目の高さベース(70%) + 笑顔時間割合(30%)
    const finalSimilarity = similarity * 0.7 + smileTimeRatio * 100 * 0.3;
    
    console.log('類似度計算:', {
      avgEyeHeight: avgEyeHeight.toFixed(4),
      similarity: similarity.toFixed(2),
      smileTimeRatio: (smileTimeRatio * 100).toFixed(2) + '%',
      finalSimilarity: finalSimilarity.toFixed(2)
    });
    
    return Math.round(finalSimilarity * 10) / 10; // 小数点第1位まで
  };

  // ゲーム終了
  const handleGameEnd = async (result: 'success' | 'failed') => {
    console.log('🛑 handleGameEnd呼び出し:', result);
    
    if (timerRef.current) {
      clearInterval(timerRef.current);
      timerRef.current = null;
    }

    // カメラを停止
    if (cameraRef.current) {
      cameraRef.current.stop();
      cameraRef.current = null;
    }

    setGameState(result);
    gameStateRef.current = result;

    // セイキン類似度を計算
    const similarity = calculateSeikinSimilarity();
    console.log('セイキン類似度:', similarity + '%');
    console.log('Current gameId (state):', gameId);
    console.log('Current gameId (ref):', gameIdRef.current);
    
    const currentGameId = gameIdRef.current || gameId;
    // バックエンドには0-1の範囲で送信（DBは0-1で保存）
    const similarityForBackend = similarity / 100;
    console.log('Calling endGame with:', { gameId: currentGameId, similarity: similarityForBackend });

    if (currentGameId) {
      try {
        // 成功時も失敗時もセイキン類似度を送信して更新
        await endGame(currentGameId, similarityForBackend);
        console.log('endGame completed successfully');
        console.log('Game ended:', result);
      } catch (error) {
        console.error('Failed to end game:', error);
      }
    } else {
      console.error('⚠️ No gameId available! Game was not started properly.');
      console.error('This means startGame was never called or failed.');
    }

    // 結果を表示してからマイページに戻る
    const message = result === 'success' 
      ? `🎉 成功！時間切れまで耐えました！\n\nセイキン類似度: ${similarity}%` 
      : `😢 失敗...笑ってしまいました（または顔が消えました）\n\nセイキン類似度: ${similarity}%`;
    alert(message);
    
    // マイページに戻る
    if (user?.user_id) {
      router.push(`/user/${user.user_id}` as any);
    } else {
      router.push('/');
    }
  };

  // リセット
  const handleReset = () => {
    // カメラを停止
    if (cameraRef.current) {
      cameraRef.current.stop();
      cameraRef.current = null;
    }
    
    setGameState('idle');
    gameStateRef.current = 'idle';
    gameEndingRef.current = false; // フラグリセット
    eyeHeightHistoryRef.current = []; // 履歴をクリア
    setTimeRemaining(180);
    setGameId(null);
    setIsSmiling(false);
    setCurrentVideo(null);
  };

  // クリーンアップ
  useEffect(() => {
    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current);
      }
    };
  }, []);

  return (
    <View style={styles.container}>
      {Platform.OS === 'web' ? (
        <>
          <div style={{ display: 'flex', gap: 20, alignItems: 'flex-start' }}>
            {/* カメラ表示 */}
            <div style={{ position: 'relative', width: 640, height: 480 }}>
              <video
                ref={videoRef}
                style={{ position: 'absolute', width: 640, height: 480, transform: 'scaleX(-1)' }}
                autoPlay
                playsInline
              />
              <canvas
                ref={canvasRef}
                width={640}
                height={480}
                style={{ position: 'absolute', width: 640, height: 480, transform: 'scaleX(-1)' }}
              />
            </div>

            {/* YouTube動画表示 */}
            {currentVideo && gameState === 'playing' && (
              <div style={{ width: 360, height: 640 }}>
                <div 
                  id="youtube-player"
                  style={{ 
                    width: 360, 
                    height: 640,
                    borderRadius: 8,
                    overflow: 'hidden'
                  }}
                />
                <Text style={styles.videoTitle}>
                  {currentVideo.title || 'セイキン動画'} ({currentVideoIndex + 1}/{allVideos.length})
                </Text>
              </div>
            )}
          </div>

          <View style={styles.infoContainer}>
            <Text style={styles.timerText}>残り時間: {timeRemaining}秒</Text>
            <Text style={styles.statusText}>
              {isSmiling ? '😊 笑顔検出!' : '😐 真顔'}
            </Text>
            <Text style={styles.stateText}>
              {gameState === 'idle' && 'ゲーム開始待ち'}
              {gameState === 'playing' && '笑わないで！セイキンさんの動画を見て耐えろ！'}
              {gameState === 'success' && '🎉 成功！時間切れまで耐えました！'}
              {gameState === 'failed' && '😢 失敗...笑ってしまいました（または顔が消えました）'}
            </Text>
            {isLoadingVideo && <Text style={styles.loadingText}>動画を読み込み中...</Text>}
          </View>

          <View style={styles.buttonContainer}>
            {gameState === 'idle' && (
              <button onClick={handleGameStart} style={styles.button} disabled={isLoadingVideo}>
                {isLoadingVideo ? '読み込み中...' : 'ゲーム開始'}
              </button>
            )}
            {(gameState === 'success' || gameState === 'failed') && (
              <button onClick={handleReset} style={styles.button}>
                もう一度
              </button>
            )}
          </View>
        </>
      ) : (
        <Text style={styles.errorText}>このゲームはWebブラウザでのみ動作します</Text>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: '#000',
    padding: 20,
  },
  infoContainer: {
    marginTop: 20,
    alignItems: 'center',
  },
  timerText: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#fff',
    marginBottom: 10,
  },
  statusText: {
    fontSize: 20,
    color: '#fff',
    marginBottom: 10,
  },
  stateText: {
    fontSize: 18,
    color: '#4CAF50',
    textAlign: 'center',
    marginBottom: 20,
  },
  loadingText: {
    fontSize: 16,
    color: '#FFA500',
    textAlign: 'center',
    marginTop: 10,
  },
  videoTitle: {
    fontSize: 14,
    color: '#fff',
    textAlign: 'center',
    marginTop: 10,
  },
  buttonContainer: {
    marginTop: 20,
  },
  button: {
    backgroundColor: '#2196F3',
    padding: 15,
    borderRadius: 8,
    minWidth: 150,
    cursor: 'pointer',
    border: 'none',
    color: '#fff',
    fontSize: 16,
    fontWeight: 'bold',
  } as any,
  errorText: {
    fontSize: 18,
    color: '#f44336',
    textAlign: 'center',
  },
});
