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

  const state: string = useMemo(() => {
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
            <Table.Row>
              <Table.ColumnHeaderCell>ID</Table.ColumnHeaderCell>
              <Table.ColumnHeaderCell>{t("bw_level")}</Table.ColumnHeaderCell>
              <Table.ColumnHeaderCell>{t("lobby_level")}</Table.ColumnHeaderCell>
              <Table.ColumnHeaderCell>FKDR</Table.ColumnHeaderCell>
              <Table.ColumnHeaderCell>BBLR</Table.ColumnHeaderCell>
              <Table.ColumnHeaderCell>{t("win_streak")}</Table.ColumnHeaderCell>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {playersInfo
              ?.sort((a, b) => {
                if (a.data != null && b.data != null) {
                  const fkdrA = Number(a.data?.bw_fkdr);
                  const fkdrB = Number(b.data?.bw_fkdr);

                  // if B > A return value < 0, A in front of B
                  // so we need B - A
                  // return fkdrA - fkdrB;
                  if (fkdrB - fkdrA != 0 && !Number.isNaN(fkdrB - fkdrA)) {
                    // console.log(a.player_data, b.player_data);
                    return fkdrB - fkdrA;
                  }
                }

                const letterA = a.name.toUpperCase();
                const letterB = b.name.toUpperCase();

                return letterA.localeCompare(letterB);
              })
              .map((item, index) => (
                <Table.Row key={JSON.stringify(item) + index}>
                  <Table.RowHeaderCell>
                    <PlayerName playerData={item.data} playerName={item.name}></PlayerName>
                  </Table.RowHeaderCell>
                  {item.data?.bw_fkdr !== "nick" && (
                    <>
                      <Table.Cell>{item.data?.bw_level}</Table.Cell>
                      <Table.Cell>{item.data?.lobby_level}</Table.Cell>
                      <Table.Cell>{item.data?.bw_fkdr}</Table.Cell>
                      <Table.Cell>{item.data?.bblr}</Table.Cell>
                      <Table.Cell>{item.data?.win_streak}</Table.Cell>
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
