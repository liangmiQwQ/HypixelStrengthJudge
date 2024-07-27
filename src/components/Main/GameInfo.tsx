import UserInfo from "./UserInfo";
import PartyInfo from "./PartyInfo";
import PlayersInfo from "./PlayersInfo";

export default function GameInfo() {
  return (
    <div className="flex w-full h-full">
      <div className="w-7/12 flex justify-center h-full sm:p-2 sm:pl-3 sm:pr-[6px] md:p-3 md:pl-5 md:pr-[10px] lg:p-7 lg:pl-10 lg:pr-5">
        {/* LEFT */}
        <PlayersInfo />
      </div>
      <div className="w-5/12 flex flex-col items-center h-full sm:p-2 sm:pr-3 sm:pl-[6px] md:p-3 md:pr-5 md:pl-[10px] lg:p-7 lg:pr-10 lg:pl-5">
        {/* RIGHT */}
        <div className="h-1/2 w-full sm:pb-1 md:pb-2 lg:pb-4">
          <PartyInfo />
        </div>
        <div className="h-1/2 w-full sm:pt-1 md:pt-2 lg:pt-4">
          <UserInfo />
        </div>
      </div>
    </div>
  );
}
