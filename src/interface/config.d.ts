interface userConfig {
  language: "zh" | "en";
  hypApiKey: string;
  accentColor: "gray" | "gold" | "bronze" | "brown" | "yellow" | "amber" | "orange" | "tomato" | "red" | "ruby" | "crimson" | "pink" | "plum" | "purple" | "violet" | "iris" | "indigo" | "blue" | "cyan" | "teal" | "jade" | "green" | "grass" | "lime" | "mint" | "sky";
  logPath: string
}

interface userConfigStore extends userConfig {
  setLanguage: (language: "zh" | "en") => void;
  setHypApiKey: (hypApiKey: string) => void;
  setAccentColor: (accentColor: "gray" | "gold" | "bronze" | "brown" | "yellow" | "amber" | "orange" | "tomato" | "red" | "ruby" | "crimson" | "pink" | "plum" | "purple" | "violet" | "iris" | "indigo" | "blue" | "cyan" | "teal" | "jade" | "green" | "grass" | "lime" | "mint" | "sky") => void;
  setLogPath: (logPath: string) => void;
}
