import { useMemo } from "react";

function PlayerName({
  playerData,
  playerName,
  className,
}: {
  playerData: PlayerData | null;
  playerName?: string;
  className?: string;
}) {
  const [beforePlus, plus] = useMemo(() => {
    if (playerData?.rank.name.endsWith("++")) {
      return [playerData?.rank.name.slice(0, -2), "++"];
    } else if (playerData?.rank.name.endsWith("+")) {
      return [playerData?.rank.name.slice(0, -1), "+"];
    } else {
      return [playerData?.rank.name, ""];
    }
  }, [playerData]);
  return (
    <span className={`font-bold ${className}`}>
      {playerData?.rank.name != "DEFAULT" && (
        <>
          <span style={{ color: playerData?.rank.name_color as string }}>{"[" + beforePlus}</span>
          <span style={{ color: playerData?.rank.plus_color as string }}>{plus}</span>
          <span style={{ color: playerData?.rank.name_color as string }}>{"] "}</span>
        </>
      )}
      <span style={{ color: playerData?.rank.name_color as string }}>
        {playerName === undefined ? playerData?.name : playerName}
      </span>
    </span>
  );
}

export default PlayerName;
