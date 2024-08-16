import { Dialog, IconButton } from "@radix-ui/themes";
import { IoMdSettings } from "react-icons/io";
import { useTranslation } from "react-i18next";
import Settings from "./Settings";
import { useEffect, useRef } from "react";
import { os } from "@tauri-apps/api";
import Control from "./Control";

export default function NavBar() {
  const { t } = useTranslation();
  const userOS = useRef<"WINDOWS" | "DARWIN" | "LINUX">();

  useEffect(() => {
    const fetchOSInfo = async () => {
      const platform = await os.platform();
      if (platform.startsWith("win")) {
        userOS.current = "WINDOWS";
      } else if (platform.startsWith("darwin")) {
        userOS.current = "DARWIN";
      } else {
        userOS.current = "LINUX";
      }
    };
    fetchOSInfo();
  }, []);

  return (
    <div
      className={
        "bg-accent-700 h-14 w-full flex justify-between items-center fixed z-50 [--webkit-app-region:no-drag] px-2 " +
        (userOS.current === "DARWIN" ? "" : "rounded-t-lg")
      }
      data-tauri-drag-region
    >
      {/* <div className={userOS.current === "DARWIN" ? "opacity-0" : ""}> */}
      <Control></Control>
      {/* </div> */}
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
          <Settings userOS={userOS.current}></Settings>
        </Dialog.Content>
      </Dialog.Root>
    </div>
  );
}
