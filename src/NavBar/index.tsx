import { useTranslation } from "react-i18next";

export default function Index() {
  const { t } = useTranslation();

  return (
    <div
      className="bg-accent-700 h-14 w-full flex justify-between items-center fixed z-50 [--webkit-app-region:drag]"
      data-tauri-drag-region
      draggable
    >
      <div>
      </div>
      <div>
        <span className="text-white font-semibold text-2xl cursor-default">
          {t("title")}
        </span>
      </div>
      <div></div>
    </div>
  );
}


