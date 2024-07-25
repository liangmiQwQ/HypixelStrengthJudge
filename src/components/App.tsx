import { Theme } from "@radix-ui/themes";
import "../global.css"
import '@radix-ui/themes/styles.css';
import NavBar from "./NavBar";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import languages from "../language/languages";
import useConfig from "../store/config";
import GameInfo from "./Main/GameInfo";

export default function App() {
  const { accentColor, language } = useConfig()

  console.log({
    resources: {
      ...languages
    }
  })
  i18n.use(initReactI18next).init({
    resources: {
      ...languages
    },
    lng: language, // if you're using a language detector, do not define the lng option
    fallbackLng: language,
    interpolation: {
      escapeValue: false // react already safes from xss => https://www.i18next.com/translation-function/interpolation#unescape
    }
  })


  return (
    <Theme appearance="light" className="rounded-lg overflow-hidden" accentColor={accentColor}>
      <div className="w-full h-screen rounded-md overflow-hidden">
        <NavBar />
        <div className="pt-14 w-full h-screen">
          <GameInfo />
        </div>
      </div>
    </Theme>
  )
}

