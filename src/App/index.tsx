import { Theme } from "@radix-ui/themes";
import "../global.css"
import '@radix-ui/themes/styles.css';
import NavBar from "../NavBar";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import languages from "../language/languages";

export default function index() {
  console.log({
    resources: {
      ...languages
    }
  })
  i18n.use(initReactI18next).init({
    resources: {
      ...languages
    },
    lng: "en", // if you're using a language detector, do not define the lng option
    fallbackLng: "en",
    interpolation: {
      escapeValue: false // react already safes from xss => https://www.i18next.com/translation-function/interpolation#unescape
    }
  })

  return (
    <Theme appearance="light" className="rounded-lg overflow-hidden">
      <div className="w-full h-screen rounded-md bg-white overflow-hidden">
        <NavBar />
      </div>
    </Theme>
  )
}

