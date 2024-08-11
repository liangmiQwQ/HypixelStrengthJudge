import { selectValue } from "../libs/accentColors";
import useConfig from "../store/config";
import SettingOption from "./SettingOption";
import { useEffect, useMemo, useRef } from "react";
import { useTranslation } from "react-i18next";
import { os } from "@tauri-apps/api";

export default function Settings() {
  const { setAccentColor, setHypApiKey, setLogPath, setLanguage, setUsername } = useConfig();
  const { accentColor, hypApiKey, logPath, language, username } = useConfig();
  const { t } = useTranslation();
  const memoizedAccentColor = useMemo(() => accentColor, []);
  const memoizedLanguage = useMemo(() => language, []);
  const isWindows = useRef(false);

  useEffect(() => {
    const fetchOSInfo = async () => {
      const platform = await os.platform();
      if (platform.startsWith("win")) {
        isWindows.current = true;
      }
    };
    fetchOSInfo();
  }, []);
  return (
    <div className="flex flex-col gap-3">
      <SettingOption
        tipsText={t("language")}
        optionType="select"
        selectValue={[
          { name: t("zh"), id: "zh" },
          { name: t("en"), id: "en" },
        ]}
        defaultValue={memoizedLanguage}
        onValueChange={setLanguage}
      />
      <SettingOption
        tipsText={t("accentColor")}
        optionType="select"
        selectValue={selectValue}
        defaultValue={memoizedAccentColor}
        onValueChange={setAccentColor}
      />
      <SettingOption
        tipsText={t("logPath")}
        optionType="input"
        defaultValue={logPath}
        placeholder={
          isWindows.current ? "C:\\\\Users\\Admin\\AppData\\.minecraft\\logs" : "~/.minecraft/logs"
        }
        onValueChange={setLogPath}
      />
      <SettingOption
        tipsText={t("hypApiKey")}
        optionType="input"
        placeholder="xxxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
        defaultValue={hypApiKey}
        onValueChange={setHypApiKey}
      />
      <SettingOption
        tipsText={t("username")}
        optionType="input"
        placeholder="Steve"
        defaultValue={username}
        onValueChange={setUsername}
      />
    </div>
  );
}
