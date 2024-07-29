import { Heading } from "@radix-ui/themes";
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";

interface PartyInfoProps {
  partyInfo: PartyInfo | null;
  otherThing?: string;
}

export default function PartyInfo({ partyInfo, otherThing }: PartyInfoProps) {
  const { t } = useTranslation();

  const state: "success" | "needPLCommand" | "needLogPath" | "needUsername" = useMemo(() => {
    if (partyInfo !== null) {
      return "success";
    } else if (otherThing != undefined || otherThing != "") {
      return otherThing as any;
    } else {
      return "needPLCommand";
    }
  }, [partyInfo]);
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
            <Heading as="h3" key={index} className="text-center">
              {line}
            </Heading>
          ))}
      {state == "success" && (
        <div>
          <Heading size="4">你 {partyInfo?.user_job === "LEADER" ? "是" : "不是"} 组队队长</Heading>
          <Heading size="5">组队信息</Heading>
          <div className="flex gap-2">
            {partyInfo?.players.map((player, index) => {
              return (
                <span key={player + index.toString()} className="text-lg font-medium">
                  {player}
                </span>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}
