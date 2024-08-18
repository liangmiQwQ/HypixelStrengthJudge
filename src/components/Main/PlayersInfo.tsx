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
    if (playersInfo !== null && playersInfo.length != 0) {
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
        "w-full p-3 min-h-full info-shadow shadow-slate-600 rounded-lg overflow-x-visible sb" +
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
                  // Step 1: Check if bw_fkdr is 'nick'
                  const isANick = a.data.bw_fkdr === "nick";
                  const isBNick = b.data.bw_fkdr === "nick";
                  const isAError = a.data.bw_fkdr === "error";
                  const isBError = b.data.bw_fkdr === "error";

                  if (isANick && !isBNick) {
                    return -1; // a should come before b
                  }
                  if (!isANick && isBNick) {
                    return 1; // b should come before a
                  }

                  // Step 2: Check if bw_fkdr is 'error'
                  if (isAError && !isBError) {
                    return 1; // a should come after b
                  }
                  if (!isAError && isBError) {
                    return -1; // b should come after a
                  }

                  // Step 3: Convert bw_fkdr to number and sort numerically if not 'nick' or 'error'
                  if (!isAError && !isBError) {
                    const fkdrA = isNaN(Number(a.data.bw_fkdr))
                      ? Number.MAX_VALUE
                      : Number(a.data.bw_fkdr);
                    const fkdrB = isNaN(Number(b.data.bw_fkdr))
                      ? Number.MAX_VALUE
                      : Number(b.data.bw_fkdr);

                    if (fkdrA !== fkdrB) {
                      return fkdrB - fkdrA; // Sort numerically in descending order
                    }
                  }
                }

                // Step 4: If bw_fkdr is the same or one of the data is null, sort by name
                const letterA = a.name.toUpperCase();
                const letterB = b.name.toUpperCase();

                return letterA.localeCompare(letterB);
              })
              .map((item, index) => (
                <Table.Row key={JSON.stringify(item) + index}>
                  <Table.RowHeaderCell>
                    <PlayerName playerData={item.data} playerName={item.name}></PlayerName>
                  </Table.RowHeaderCell>
                  {item.data?.bw_fkdr !== "nick" && item.data?.bw_fkdr != "error" && (
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
                  {item.data?.bw_fkdr === "error" && (
                    <>
                      <Table.Cell colSpan={5} className="text-center text-slate-600 font-bold">
                        {t("error")}
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
