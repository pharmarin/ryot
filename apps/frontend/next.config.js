const withPWAInit = require('next-pwa');
const runtimeCaching = require('./worker/cache');

/** @type {import('next-pwa').PWAConfig} */
const withPWA = withPWAInit({
  dest: 'public',
  register: true,
  skipWaiting: true,
  disable: process.env.NODE_ENV === 'development',
  runtimeCaching,
});

/** @type {import('next').NextConfig} */
const nextConfig = {
  eslint: { ignoreDuringBuilds: true },
  // output: "export",
  reactStrictMode: true,
  transpilePackages: ['@ryot/generated', '@ryot/graphql'],
  typescript: { ignoreBuildErrors: true },
  rewrites: async () => {
    if (process.env.NODE_ENV !== 'development') {
      return undefined;
    }

    return [
      {
        source: '/graphql/:path*',
        destination: 'http://127.0.0.1:8000/graphql/:path*',
      },
    ];
  },
};

module.exports = withPWA(nextConfig);
