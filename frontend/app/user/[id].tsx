import React, { useEffect } from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  ScrollView,
  ActivityIndicator,
} from 'react-native';
import { useAuth } from '../../context/AuthContext';
import { useRouter, useLocalSearchParams } from 'expo-router';

export default function UserScreen() {
  const { user, loading, signOut, refreshUser } = useAuth();
  const router = useRouter();
  const { id } = useLocalSearchParams();

  useEffect(() => {
    refreshUser();
  }, []);

  const handleLogout = async () => {
    await signOut();
    router.replace('/');
  };

  if (loading || !user) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color="#3b5998" />
        <Text style={styles.loadingText}>読み込み中...</Text>
      </View>
    );
  }

  // URLのIDと現在のユーザーIDが一致しない場合は権限エラー
  if (id !== user.user_id) {
    return (
      <View style={styles.errorContainer}>
        <Text style={styles.errorText}>アクセス権限がありません</Text>
        <TouchableOpacity
          style={styles.backButton}
          onPress={() => router.replace(`/user/${user.user_id}` as any)}
        >
          <Text style={styles.backButtonText}>マイページに戻る</Text>
        </TouchableOpacity>
      </View>
    );
  }

  return (
    <ScrollView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.headerTitle}>マイページ</Text>
      </View>

      <View style={styles.content}>
        <View style={styles.profileCard}>
          <View style={styles.avatarContainer}>
            <View style={styles.avatar}>
              <Text style={styles.avatarText}>
                {user.name.charAt(0).toUpperCase()}
              </Text>
            </View>
          </View>

          <View style={styles.infoContainer}>
            <View style={styles.infoRow}>
              <Text style={styles.infoLabel}>名前</Text>
              <Text style={styles.infoValue}>{user.name}</Text>
            </View>

            <View style={styles.infoRow}>
              <Text style={styles.infoLabel}>メールアドレス</Text>
              <Text style={styles.infoValue}>{user.email}</Text>
            </View>

            <View style={styles.infoRow}>
              <Text style={styles.infoLabel}>ユーザーID</Text>
              <Text style={styles.infoValue}>{user.user_id}</Text>
            </View>
          </View>
        </View>

        <View style={styles.statsCard}>
          <Text style={styles.statsTitle}>セイキン類似度</Text>
          <View style={styles.similarityContainer}>
            <Text style={styles.similarityValue}>
              {(user.seikin_similarity * 100).toFixed(1)}%
            </Text>
            <Text style={styles.similarityDescription}>
              あなたのセイキンらしさ
            </Text>
          </View>
          <View style={styles.progressBar}>
            <View
              style={[
                styles.progressFill,
                { width: `${user.seikin_similarity * 100}%` },
              ]}
            />
          </View>
        </View>

        <View style={styles.actionsCard}>
          <TouchableOpacity
            style={styles.actionButton}
            onPress={() => {
              // ゲーム画面への遷移（今後実装）
            }}
          >
            <Text style={styles.actionButtonText}>ゲームを始める</Text>
          </TouchableOpacity>

          <TouchableOpacity
            style={[styles.actionButton, styles.secondaryActionButton]}
            onPress={refreshUser}
          >
            <Text style={[styles.actionButtonText, styles.secondaryActionButtonText]}>
              情報を更新
            </Text>
          </TouchableOpacity>

          <TouchableOpacity
            style={[styles.actionButton, styles.logoutButton]}
            onPress={handleLogout}
          >
            <Text style={[styles.actionButtonText, styles.logoutButtonText]}>
              ログアウト
            </Text>
          </TouchableOpacity>
        </View>
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#f5f5f5',
  },
  loadingText: {
    marginTop: 16,
    fontSize: 16,
    color: '#666',
  },
  errorContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#f5f5f5',
    padding: 20,
  },
  errorText: {
    fontSize: 18,
    color: '#dc3545',
    marginBottom: 20,
    textAlign: 'center',
  },
  backButton: {
    backgroundColor: '#3b5998',
    paddingVertical: 12,
    paddingHorizontal: 24,
    borderRadius: 8,
  },
  backButtonText: {
    color: '#ffffff',
    fontSize: 16,
    fontWeight: 'bold',
  },
  header: {
    backgroundColor: '#3b5998',
    paddingTop: 60,
    paddingBottom: 20,
    paddingHorizontal: 20,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 4,
  },
  headerTitle: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#ffffff',
  },
  content: {
    padding: 20,
  },
  profileCard: {
    backgroundColor: '#ffffff',
    borderRadius: 16,
    padding: 20,
    marginBottom: 20,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 8,
    elevation: 4,
  },
  avatarContainer: {
    alignItems: 'center',
    marginBottom: 20,
  },
  avatar: {
    width: 80,
    height: 80,
    borderRadius: 40,
    backgroundColor: '#3b5998',
    justifyContent: 'center',
    alignItems: 'center',
  },
  avatarText: {
    fontSize: 36,
    fontWeight: 'bold',
    color: '#ffffff',
  },
  infoContainer: {
    gap: 16,
  },
  infoRow: {
    borderBottomWidth: 1,
    borderBottomColor: '#e0e0e0',
    paddingBottom: 12,
  },
  infoLabel: {
    fontSize: 14,
    color: '#666',
    marginBottom: 4,
  },
  infoValue: {
    fontSize: 16,
    color: '#333',
    fontWeight: '500',
  },
  statsCard: {
    backgroundColor: '#ffffff',
    borderRadius: 16,
    padding: 20,
    marginBottom: 20,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 8,
    elevation: 4,
  },
  statsTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 16,
    textAlign: 'center',
  },
  similarityContainer: {
    alignItems: 'center',
    marginBottom: 16,
  },
  similarityValue: {
    fontSize: 48,
    fontWeight: 'bold',
    color: '#3b5998',
  },
  similarityDescription: {
    fontSize: 14,
    color: '#666',
    marginTop: 4,
  },
  progressBar: {
    height: 12,
    backgroundColor: '#e0e0e0',
    borderRadius: 6,
    overflow: 'hidden',
  },
  progressFill: {
    height: '100%',
    backgroundColor: '#3b5998',
    borderRadius: 6,
  },
  actionsCard: {
    gap: 12,
  },
  actionButton: {
    backgroundColor: '#3b5998',
    paddingVertical: 16,
    paddingHorizontal: 24,
    borderRadius: 12,
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 4,
  },
  actionButtonText: {
    color: '#ffffff',
    fontSize: 16,
    fontWeight: 'bold',
  },
  secondaryActionButton: {
    backgroundColor: '#ffffff',
    borderWidth: 2,
    borderColor: '#3b5998',
  },
  secondaryActionButtonText: {
    color: '#3b5998',
  },
  logoutButton: {
    backgroundColor: '#dc3545',
  },
  logoutButtonText: {
    color: '#ffffff',
  },
});
