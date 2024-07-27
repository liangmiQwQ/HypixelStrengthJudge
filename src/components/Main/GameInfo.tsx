import UserInfo from "./UserInfo";
import PartyInfo from "./PartyInfo";
import PlayersInfo from "./PlayersInfo";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import useConfig from "../../store/config";

export default function GameInfo() {
  // invoke("get_latest_info");
  const [partyInfo, setPartyInfo] = useState<PartyInfo | null>(null);
  const { logPath, username } = useConfig();
  const [otherThing, setOtherThing] = useState("");

  useEffect(() => {
    const timer = setInterval(() => {
      if (logPath === "") {
        setOtherThing("needLogPath");
      } else if (username === "") {
        setOtherThing("needUsername");
      } else {
        invoke("get_latest_info", { logDirPath: logPath, username }).then((info: unknown) => {
          console.log(JSON.stringify((info as any as info).party_info));
          if ((info as any as info).party_info != null) {
            setPartyInfo((info as any as info).party_info);
          }
        });
      }
    }, 5000);

    return () => {
      clearInterval(timer);
    };
  }, [logPath, username]);

  return (
    <div className="flex w-full h-full">
      <div className="w-7/12 flex justify-center h-full sm:p-2 sm:pl-3 sm:pr-[6px] md:p-3 md:pl-5 md:pr-[10px] lg:p-7 lg:pl-10 lg:pr-5">
        {/* LEFT */}
        <PlayersInfo />
      </div>
      <div className="w-5/12 flex flex-col items-center h-full sm:p-2 sm:pr-3 sm:pl-[6px] md:p-3 md:pr-5 md:pl-[10px] lg:p-7 lg:pr-10 lg:pl-5">
        {/* RIGHT */}
        <div className="h-1/2 w-full sm:pb-1 md:pb-2 lg:pb-4">
          <PartyInfo partyInfo={partyInfo} otherThing={otherThing} />
        </div>
        <div className="h-1/2 w-full sm:pt-1 md:pt-2 lg:pt-4">
          <UserInfo />
        </div>
      </div>
    </div>
  );
}
