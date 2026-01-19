import { DefaultTheme, ThemeProvider } from '@react-navigation/native';
import { Stack } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import 'react-native-reanimated';
import { AuthProvider, useAuth } from '../context/AuthContext';
import { ActivityIndicator, View, StyleSheet } from 'react-native';

function RootLayoutNav() {
  const { isAuthenticated, loading } = useAuth();

  if (loading) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color="#007AFF" />
      </View>
    );
  }

  return (
    <ThemeProvider value={DefaultTheme}>
      <Stack screenOptions={{ headerShown: false }}>
        {isAuthenticated ? (
          <>
            <Stack.Screen name="user/[id]" />
            <Stack.Screen name="(tabs)" />
          </>
        ) : (
          <>
            <Stack.Screen name="index" />
            <Stack.Screen name="signin" />
            <Stack.Screen name="signup" />
          </>
        )}
      </Stack>
      <StatusBar style="auto" />
    </ThemeProvider>
  );
}

export default function RootLayout() {
  return (
    <AuthProvider>
      <RootLayoutNav />
    </AuthProvider>
  );
}

const styles = StyleSheet.create({
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#f5f5f5',
  },
});
