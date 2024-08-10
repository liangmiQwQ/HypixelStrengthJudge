import { Heading } from "@radix-ui/themes";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import PlayerName from "./PlayerName";
import { getBedwarsMode } from "../../libs/getBedwarsMode";

interface UserInfoProps {
  otherThing?: string;
  personalData: PersonalData | null;
  name: string;
}

export default function UserInfo({ personalData, name, otherThing }: UserInfoProps) {
  const { t } = useTranslation();

  const state: string = useMemo(() => {
    if (personalData !== null) {
      return "success";
    } else if (otherThing != undefined && otherThing != "") {
      return otherThing as any;
    } else {
      return "somethingError";
    }
  }, [personalData, otherThing]);

  return (
    <div
      className={
        "w-full p-3 h-full info-shadow shadow-slate-600 rounded-lg" +
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
        <div className="flex flex-col px-3 h-full">
          <div className="flex h-[100px] items-center px-1">
            <img src={`https://mineskin.eu/headhelm/${name}/100.png`}></img>
            <div className="flex flex-col items-center flex-1">
              <PlayerName
                playerData={personalData?.data as any}
                playerName={name}
                className="text-lg flex-1 mt-[-5px]"
              ></PlayerName>
              <div>
                <span className="text-slate-800">FKDR: {personalData?.data?.bw_fkdr}</span>
                <span className="text-slate-800 ml-3">
                  {t("win_streak")}: {personalData?.data?.win_streak}
                </span>
              </div>
            </div>
          </div>
          <div className="flex flex-1 py-1 pb-4">
            <div className="flex-1 border-r-[0.5px] border-slate-500 flex flex-col justify-center">
              {/* location */}
              <p>
                {t("location")}: {personalData?.location.game_type}{" "}
                {personalData?.location.server_type}
              </p>
              {personalData?.location.map != null && (
                <p>
                  {t("map")}: {personalData?.location.map}
                </p>
              )}
              {personalData?.location.game_mode != null && (
                <p>
                  {t("gameMode")}: {getBedwarsMode(personalData.location.game_mode)}
                </p>
              )}
            </div>
            <div className="flex-1 border-l-[0.5px] border-slate-500 flex flex-col justify-center pl-5">
              {/* state */}
              <p>
                {t("lobby_level")}: {personalData?.data?.lobby_level}
              </p>
              <p>
                {t("bw_level")}: {personalData?.data?.bw_level}
              </p>
              <p>BBLR: {personalData?.data?.bblr}</p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
