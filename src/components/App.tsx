import { Theme } from "@radix-ui/themes";
import "../global.css";
import "@radix-ui/themes/styles.css";
import NavBar from "./NavBar";
import { initI18n } from "../language/languages";
import useConfig from "../store/config";
import GameInfo from "./Main/GameInfo";
import { useEffect } from "react";
import i18next from "i18next";

export default function App() {
  const { accentColor, language } = useConfig();
  // const { i18n } = useTranslation();

  useEffect(() => {
    initI18n(language);
  });

  useEffect(() => {
    //   i18n.changeLanguage(language);
    i18next.changeLanguage(language, (err, _t) => {
      if (err) return console.log("something went wrong loading", err);
    });
  }, [language]);

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
