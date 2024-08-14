import { Dialog, IconButton } from "@radix-ui/themes";
import { IoMdSettings } from "react-icons/io";
import { useTranslation } from "react-i18next";
import Settings from "./Settings";

export default function NavBar() {
  const { t } = useTranslation();

  return (
    <div
      className="bg-accent-700 h-14 w-full flex justify-between items-center fixed z-50 [--webkit-app-region:no-drag] px-2 rounded-t-lg"
      data-tauri-drag-region
    >
      <div className="opacity-0">
        <IconButton radius="full" size="3">
          <IoMdSettings />
        </IconButton>
      </div>
      <div>
        <span
          className="text-white font-semibold text-2xl cursor-default [--webkit-app-region:no-drag]"
          data-tauri-drag-region
        >
          {t("title")}
        </span>
      </div>
      <Dialog.Root>
        <Dialog.Trigger>
          <IconButton radius="full" size="3">
            <IoMdSettings size={25} />
          </IconButton>
        </Dialog.Trigger>
        <Dialog.Content>
          <Dialog.Title>{t("settings")}</Dialog.Title>
          <Settings></Settings>
        </Dialog.Content>
      </Dialog.Root>
    </div>
  );
}
