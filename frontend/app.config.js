module.exports = ({ config }) => {
  return {
    ...config,
    extra: {
      apiUrl: process.env.EXPO_PUBLIC_API_URL || 'http://localhost:8080',
    },
  };
};
