import { Heading } from "@radix-ui/themes";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";

interface PlayersInfoProps {
  playersInfo: null | (null | PlayerData)[];
  otherThing?: string;
}

export default function PlayersInfo({ playersInfo, otherThing }: PlayersInfoProps) {
  const { t } = useTranslation();

  const state: "success" | "needWhoCommand" | "needLogPath" | "needUsername" | "needJoinServer" =
    useMemo(() => {
      console.log(playersInfo);
      if (playersInfo !== null) {
        return "success";
      } else if (otherThing != undefined && otherThing != "") {
        return otherThing as any;
      } else {
        return "needWhoCommand";
      }
    }, [playersInfo, otherThing]);

  return (
    <div
      className={
        "w-full p-3 min-h-full info-shadow shadow-slate-600 rounded-lg" +
        (state === "success" ? "" : " flex justify-center items-center flex-col")
      }
    >
      {state != "success" &&
        state !== undefined &&
        t("info_" + state)
          .split("\n")
          .map((line, index) => (
            <Heading as="h3" key={index} className="text-center">
              {line}
            </Heading>
          ))}
    </div>
  );
}
