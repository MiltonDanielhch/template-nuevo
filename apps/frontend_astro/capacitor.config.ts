import type { CapacitorConfig } from "@capacitor/cli";

const config: CapacitorConfig = {
  appId: "com.lab3026.app",
  appName: "Laboratorio 3026",
  webDir: "dist",
  server: {
    url: "http://localhost:4321",
    cleartext: true,
  },
  cordova: {
    preferences: {
      ScrollEnabled: "false",
      Orientation: "portrait",
      "android-minSdkVersion": "22",
      "android-targetSdkVersion": "33",
    },
  },
};

export default config;
