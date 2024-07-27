import { Heading } from "@radix-ui/themes";
import { useState } from "react";
import { useTranslation } from "react-i18next";

export default function PartyInfo() {
  const { t } = useTranslation();

  const [state, setState] = useState<"success" | "needPLCommand" | "needLogPath">("needPLCommand");
  return (
    <div
      className={
        "w-full p-3 h-full info-shadow-top shadow-slate-600 rounded-lg" +
        (state === "success" ? "" : " flex justify-center items-center flex-col")
      }
    >
      {state != "success" &&
        state !== undefined &&
        t("info_" + state)
          .split("\n")
          .map((line, index) => (
            <Heading as="h3" key={index}>
              {line}
            </Heading>
          ))}
    </div>
  );
}
