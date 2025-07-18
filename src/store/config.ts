import { create } from "zustand";

const useConfig = create<userConfigStore>((set, _get) => {
  const localLanguage = localStorage.getItem("language");
  const localHypApiKey = localStorage.getItem("hypApiKey");
  const localAccentColor = localStorage.getItem("accentColor");
  const localLogPath = localStorage.getItem("logPath");
  const localUsername = localStorage.getItem("username");

  return {
    language: localLanguage ? (localLanguage as "zh" | "en") : "en",
    setLanguage: (language: "zh" | "en") => {
      localStorage.setItem("language", language);
      set({ language });
    },
    hypApiKey: localHypApiKey || "",
    setHypApiKey: (hypApiKey: string) => {
      localStorage.setItem("hypApiKey", hypApiKey);
      set({ hypApiKey });
    },
    accentColor: localAccentColor ? (localAccentColor as userConfig["accentColor"]) : "teal",
    setAccentColor: (accentColor: userConfig["accentColor"]) => {
      localStorage.setItem("accentColor", accentColor);
      set({ accentColor });
    },
    logPath: localLogPath || "",
    setLogPath: (logPath: string) => {
      localStorage.setItem("logPath", logPath);
      set({ logPath });
    },
    username: localUsername || "",
    setUsername: (username: string) => {
      localStorage.setItem("username", username);
      set({ username });
    },
  };
});

export default useConfig;
