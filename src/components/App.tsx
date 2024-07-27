import { Theme } from "@radix-ui/themes";
import "../global.css";
import "@radix-ui/themes/styles.css";
import NavBar from "./NavBar";
import { initI18n } from "../language/languages";
import useConfig from "../store/config";
import GameInfo from "./Main/GameInfo";

export default function App() {
  const { accentColor, language } = useConfig();

  initI18n(language);

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
