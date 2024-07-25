import { Popover, IconButton } from "@radix-ui/themes";
import { IoMdSettings } from "react-icons/io";
import { useTranslation } from "react-i18next";
import SettingOption from "./SettingOption";
import useConfig from "../store/config";
import { selectValue } from "../libs/accentColors";
import { useEffect, useMemo, useRef } from "react";
import { os } from '@tauri-apps/api';
import { Value } from "@radix-ui/themes/src/components/data-list.js";

export default function NavBar() {
  const { t } = useTranslation();
  const { setAccentColor, setHypApiKey, setLogPath, setLanguage } = useConfig()
  const { accentColor, hypApiKey, logPath, language } = useConfig()

  const isWindows = useRef(false)
  useEffect(() => {
    const fetchOSInfo = async () => {
      const platform = await os.platform();
      if (platform.startsWith("win")) {
        isWindows.current = true
      }
    }
    fetchOSInfo();
  }, [])
  const memoizedAccentColor = useMemo(() => accentColor, []);
  const memoizedHypApiKey = useMemo(() => hypApiKey, []);
  const memoizedLogPath = useMemo(() => logPath, []);
  const memoizedLanguage = useMemo(() => language, []);

  return (
    <div
      className="bg-accent-700 h-14 w-full flex justify-between items-center fixed z-50 [--webkit-app-region:no-drag] px-2"
      data-tauri-drag-region
    >
      <div className="opacity-0">
        <IconButton radius="full" size="3" >
          <IoMdSettings />
        </IconButton>

      </div>
      <div>
        <span className="text-white font-semibold text-2xl cursor-default [--webkit-app-region:no-drag]"
          data-tauri-drag-region>
          {t("title")}
        </span>
      </div>
      <Popover.Root>
        <Popover.Trigger>
          <IconButton radius="full" size="3">
            <IoMdSettings size={25} />
          </IconButton>
        </Popover.Trigger>
        <Popover.Content className="w-80 flex flex-col gap-1">
          <SettingOption
            tipsText={t("language")}
            optionType="select"
            selectValue={[
              { name: t("zh"), id: "zh" },
              { name: t("en"), id: "en" }
            ]}
            defaultValue={memoizedLanguage}
            onValueChange={(value: any) => setLanguage(value as ("zh" | "en"))}
          />
          <SettingOption
            tipsText={t("accentColor")}
            optionType="select"
            selectValue={selectValue}
            defaultValue={memoizedAccentColor}
            onValueChange={(value: any) => setAccentColor(value)}
          />
          <SettingOption
            tipsText={t("logPath")}
            optionType="input"
            placeholder={isWindows.current ? "C:\\\\Users\\Admin\\AppData\\.minecraft\\logs" : "~/.minecraft/logs"}
            onValueChange={(value: any) => setLogPath(value)}
          />
          <SettingOption
            tipsText={t("hypApiKey")}
            optionType="input"
            placeholder="xxxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
            onValueChange={(value: any) => setHypApiKey(value)}
          />
        </Popover.Content>
      </Popover.Root>
    </div>
  );
}


