import { View, Text, StyleSheet } from 'react-native';

export default function DebugScreen() {
  const apiUrl = process.env.EXPO_PUBLIC_API_URL || 'Not set';
  
  return (
    <View style={styles.container}>
      <Text style={styles.title}>Debug Information</Text>
      <Text style={styles.text}>API URL: {apiUrl}</Text>
      <Text style={styles.text}>Environment: {process.env.NODE_ENV}</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 20,
  },
  text: {
    fontSize: 16,
    marginBottom: 10,
  },
});
