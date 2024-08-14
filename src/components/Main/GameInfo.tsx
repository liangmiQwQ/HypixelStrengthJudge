import UserInfo from "./UserInfo";
import PartyInfo from "./PartyInfo";
import PlayersInfo from "./PlayersInfo";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useRef, useState } from "react";
import useConfig from "../../store/config";

export default function GameInfo() {
  // invoke("get_latest_info");
  const [partyInfo, setPartyInfo] = useState<PartyInfo | null>(null);
  const [playerInfo, setPlayerInfo] = useState<ReturnPlayerData[] | null>(null);
  const [personalData, setPersonalData] = useState<PersonalData | null>(null);
  const { logPath, username, hypApiKey } = useConfig();
  const [otherThing, setOtherThing] = useState("");

  const runTimes = useRef(0);
  const timer = useRef<number | undefined>();

  useEffect(() => {
    fetch(`https://api.hypixel.net/punishmentstats?key=${hypApiKey}`)
      .then(data => data.json())
      .then(data => {
        if (!data.success) {
          // have error
          setOtherThing("badApiKey");
        }
      })
      .catch(_e => {
        setOtherThing("badApiKey");
      });
  }, [hypApiKey]);

  useEffect(() => {
    if (import.meta.env.DEV) {
      runTimes.current++;
      if (runTimes.current === 1) return;
    }

    const handleTimer = async () => {
      await getLatestInfo();
      timer.current = setInterval(getLatestInfo, 2500);
    };

    async function getLatestInfo() {
      if (logPath === "" && otherThing != "badApiKey") {
        setOtherThing("needLogPath");
      } else if (username === "" && otherThing != "badApiKey") {
        setOtherThing("needUsername");
      } else if (hypApiKey === "" && otherThing != "badApiKey") {
        setOtherThing("needKey");
      } else {
        console.log("Start invoke");
        const info: any = await invoke("get_latest_info", {
          logDirPath: logPath,
          username,
          apiKey: hypApiKey,
        });

        console.log(JSON.stringify(info));
        if (
          (info as info).personal_data.location.server_type === "UNKNOWN" &&
          otherThing != "badApiKey"
        ) {
          setOtherThing("needJoinServer");
        } else {
          setPlayerInfo((info as info).player_data);
          setPartyInfo((info as info).party_info);
          setPersonalData((info as info).personal_data);
          setOtherThing("");
        }
      }
    }

    if (timer.current != undefined) {
      clearInterval(timer.current);
      timer.current = undefined;
    }
    handleTimer();

    return () => {
      clearInterval(timer.current);
    };
  }, [logPath, username, hypApiKey]);

  return (
    <div className="flex w-full h-full">
      <div className="w-[55%] flex justify-center h-full sm:p-2 sm:pl-3 sm:pr-[6px] md:p-3 md:pl-5 md:pr-[10px] lg:p-7 lg:pl-10 lg:pr-5">
        {/* LEFT */}
        <PlayersInfo playersInfo={playerInfo} otherThing={otherThing} />
      </div>
      <div className="w-[45%] flex flex-col items-center h-full sm:p-2 sm:pr-3 sm:pl-[6px] md:p-3 md:pr-5 md:pl-[10px] lg:p-7 lg:pr-10 lg:pl-5">
        {/* RIGHT */}
        <div className="h-1/2 w-full sm:pb-1 md:pb-2 lg:pb-4">
          <PartyInfo partyInfo={partyInfo} otherThing={otherThing} />
        </div>
        <div className="h-1/2 w-full sm:pt-1 md:pt-2 lg:pt-4">
          <UserInfo personalData={personalData} otherThing={otherThing} name={username} />
        </div>
      </div>
    </div>
  );
}
