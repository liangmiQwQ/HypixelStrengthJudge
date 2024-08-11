import { Heading, Table } from "@radix-ui/themes";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import PlayerName from "./PlayerName";

interface PartyInfoProps {
  partyInfo: PartyInfo | null;
  otherThing?: string;
}

export default function PartyInfo({ partyInfo, otherThing }: PartyInfoProps) {
  const { t } = useTranslation();

  const state: string = useMemo(() => {
    if (partyInfo !== null) {
      return "success";
    } else if (otherThing != undefined && otherThing != "") {
      return otherThing as any;
    } else {
      return "needPLCommand";
    }
  }, [partyInfo, otherThing]);
  return (
    <div
      className={
        "w-full p-3 h-full info-shadow-top shadow-slate-600 rounded-lg sb" +
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
              <Table.ColumnHeaderCell>FKDR</Table.ColumnHeaderCell>
              <Table.ColumnHeaderCell>BBLR</Table.ColumnHeaderCell>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {partyInfo?.players
              .sort((a, b) => {
                if (a.player_data != null && b.player_data != null) {
                  const isANick = a.player_data.bw_fkdr === "nick";
                  const isBNick = b.player_data.bw_fkdr === "nick";
                  const isAError = a.player_data.bw_fkdr === "error";
                  const isBError = b.player_data.bw_fkdr === "error";

                  // Step 1: Check if bw_fkdr is 'nick'
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
                    const fkdrA = isNaN(Number(a.player_data.bw_fkdr))
                      ? Number.MAX_VALUE
                      : Number(a.player_data.bw_fkdr);
                    const fkdrB = isNaN(Number(b.player_data.bw_fkdr))
                      ? Number.MAX_VALUE
                      : Number(b.player_data.bw_fkdr);

                    if (fkdrA !== fkdrB) {
                      return fkdrB - fkdrA; // Sort numerically in descending order
                    }
                  }
                }

                // Step 4: If bw_fkdr is the same or one of the player_data is null, sort by name
                const letterA = a.name.toUpperCase();
                const letterB = b.name.toUpperCase();

                return letterA.localeCompare(letterB);
              })
              .map((item, index) => (
                <Table.Row key={item.toString() + index.toString()}>
                  <Table.RowHeaderCell>
                    <PlayerName playerData={item?.player_data} playerName={item?.name}></PlayerName>
                  </Table.RowHeaderCell>
                  {item.player_data?.bw_fkdr !== "nick" && (
                    <>
                      <Table.Cell>{item.player_data?.bw_level}</Table.Cell>
                      <Table.Cell>{item.player_data?.bw_fkdr}</Table.Cell>
                      <Table.Cell>{item.player_data?.bblr}</Table.Cell>
                    </>
                  )}
                  {item.player_data?.bw_fkdr === "nick" && (
                    <>
                      <Table.Cell colSpan={3} className="text-center text-amber-800 font-bold">
                        Nick
                      </Table.Cell>
                    </>
                  )}
                  {item.player_data?.bw_fkdr === "error" && (
                    <>
                      <Table.Cell colSpan={3} className="text-center text-slate-600 font-bold">
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
