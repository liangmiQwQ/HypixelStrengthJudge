import { Popover, IconButton } from "@radix-ui/themes";
import { IoMdSettings } from "react-icons/io";
import { useTranslation } from "react-i18next";
import SettingOption from "./SettingOption";
import useConfig from "../store/config";

export default function Index() {
  const { t } = useTranslation();
  const { setAccentColor, setHypApiKey, setLogPath, setLanguage } = useConfig()
  const { accentColor, hypApiKey, logPath, language } = useConfig()

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
        <Popover.Content className="w-80 flex flex-col">
          <SettingOption
            tipsText="Language"
            optionType="select"
            selectValue={[
              { name: "Chinese", id: "zh" },
              { name: "English", id: "en" }
            ]}
            defaultValue={language}
            onValueChange={(value: any) => setLanguage(value as ("zh" | "en"))}
          />
        </Popover.Content>
      </Popover.Root>
    </div>
  );
}


