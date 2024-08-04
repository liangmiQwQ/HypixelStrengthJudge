import { Heading, Table } from "@radix-ui/themes";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import PlayerName from "./PlayerName";

interface PlayersInfoProps {
  playersInfo: null | ReturnPlayerData[];
  otherThing?: string;
}

export default function PlayersInfo({ playersInfo, otherThing }: PlayersInfoProps) {
  const { t } = useTranslation();

  const state: "success" | "needWhoCommand" | "needLogPath" | "needUsername" | "needJoinServer" =
    useMemo(() => {
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
      {state == "success" && (
        <Table.Root className="w-full">
          <Table.Header>
            <Table.ColumnHeaderCell>ID</Table.ColumnHeaderCell>
            <Table.ColumnHeaderCell>{t("bw_level")}</Table.ColumnHeaderCell>
            <Table.ColumnHeaderCell>{t("lobby_level")}</Table.ColumnHeaderCell>
            <Table.ColumnHeaderCell>FKDR</Table.ColumnHeaderCell>
            <Table.ColumnHeaderCell>BBLR</Table.ColumnHeaderCell>
            <Table.ColumnHeaderCell>{t("win_streak")}</Table.ColumnHeaderCell>
          </Table.Header>
          <Table.Body>
            {playersInfo?.map((item, index) => (
              <Table.Row key={JSON.stringify(item) + index}>
                <Table.RowHeaderCell>
                  <PlayerName playerData={item.data} playerName={item.name}></PlayerName>
                </Table.RowHeaderCell>
                {item.data?.bw_fkdr !== "nick" && (
                  <>
                    <Table.Cell>{item.data?.bw_fkdr}</Table.Cell>
                    <Table.Cell>{item.data?.bw_fkdr}</Table.Cell>
                    <Table.Cell>{item.data?.bblr}</Table.Cell>
                  </>
                )}
                {item.data?.bw_fkdr === "nick" && (
                  <>
                    <Table.Cell colSpan={5} className="text-center text-amber-800 font-bold">
                      Nick
                    </Table.Cell>
                  </>
                )}
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Root>
      )}
    </div>
  );
}
