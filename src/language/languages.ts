import { initReactI18next } from "react-i18next";
import i18n from "i18next";

const en = {
  translation: {
    title: "Hyp BW Strength Judge",
    language: "Language",
    zh: "简体中文",
    en: "English",
    hypApiKey: "HypApi Key",
    logPath: "Minecraft Logs Filepath",
    accentColor: "Theme Color",
    username: "Minecraft ID",
    info_needWhoCommand:
      "Join the game and enter the /who command in Minecraft\nto check the opponent's battle record",
    info_needPLCommand:
      "Join a party and enter the /pl command \n to start tracking your party's state",
    info_needLogPath: "Please set the log path of Minecraft in the settings",
    info_needUsername: "Please set your Minecraft ID in the settings",
    info_needKey: "Please set your personal Hypixel API key in the settings",
    info_badApiKey: "Invalid Hypixel API key",
    info_needJoinServer: "Please join Hypixel first",
    info_somethingError: "We have some unknown errors, please wait a moment and try again! ",
    bw_level: "BW Level",
    lobby_level: "Lobby Level",
    win_streak: "WS",
    location: "Location",
    map: "map",
    gameMode: "Game Mode",
  },
};

const zh = {
  translation: {
    title: "Hypixel 起床战争强度法官",
    language: "语言",
    zh: "简体中文",
    en: "English",
    hypApiKey: "HypApi 密钥",
    logPath: "Minecraft 日志路径",
    accentColor: "主题颜色",
    username: "游戏ID",
    info_needWhoCommand: "加入游戏并在聊天框输入\n/who 命令以查询对手战绩",
    info_needPLCommand: "请加入组队并在聊天框中输入\n/pl 命令以开始追踪组队状态",
    info_needLogPath: "请在设置中填写Minecraft的日志路径",
    info_needUsername: "请在设置中填写自己的游戏ID",
    info_needKey: "请在设置中填写您的Hypixel API key",
    info_badApiKey: "无效的 Hypixel API key",
    info_needJoinServer: "请先加入Hypixel服务器",
    info_somethingError: "发生了未知的错误! 请等待一段时间后重试!",
    bw_level: "BW等级",
    lobby_level: "大厅等级",
    win_streak: "连胜",
    location: "位置",
    map: "地图",
    gameMode: "游戏模式",
  },
};

export default {
  en,
  zh,
};

export function initI18n(language: "zh" | "en") {
  i18n.use(initReactI18next).init({
    resources: {
      zh,
      en,
    },
    lng: language, // if you're using a language detector, do not define the lng option
    fallbackLng: language,
    interpolation: {
      escapeValue: false, // react already safes from xss => https://www.i18next.com/translation-function/interpolation#unescape
    },
  });
}
