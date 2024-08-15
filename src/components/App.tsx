import { Theme } from "@radix-ui/themes";
import "../global.css";
import "@radix-ui/themes/styles.css";
import NavBar from "./NavBar";
import useConfig from "../store/config";
import GameInfo from "./Main/GameInfo";
import { useEffect, useRef } from "react";
import i18next from "i18next";
import { initI18n } from "../language/languages";

export default function App() {
  const { accentColor, language } = useConfig();
  const isInit = useRef(false);

  useEffect(() => {
    if (isInit.current) {
      i18next.changeLanguage(language, (err, _t) => {
        if (err) return console.log("something went wrong loading", err);
      });
    }
  }, [language]);

  if (!isInit.current) {
    initI18n(language);
    isInit.current = true;
  }

  return (
    <Theme appearance="light" className="rounded-lg overflow-hidden" accentColor={accentColor}>
      <div className="w-full h-screen rounded-md overflow-hidden">
        <NavBar />
        <div className="pt-14 w-full h-screen">
          <GameInfo />
        </div>
      </div>
    </Theme>
  );
}
