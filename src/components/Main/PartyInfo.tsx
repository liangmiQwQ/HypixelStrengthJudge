import { Heading, Table } from "@radix-ui/themes";
import { useMemo } from "react";
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
  }, [partyInfo, otherThing]);
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
                const firstLetterA = a.name[0].toUpperCase();
                const firstLetterB = b.name[0].toUpperCase();

                if (firstLetterA < firstLetterB) {
                  return -1;
                }
                if (firstLetterA > firstLetterB) {
                  return 1;
                }
                return 0;
              })
              .map((item, index) => {
                function PlayerName({ item }: { item: PartyPlayerData }) {
                  const [beforePlus, plus] = useMemo(() => {
                    if (item.player_data?.rank.name.endsWith("+")) {
                      return [item.player_data?.rank.name.slice(0, -1), "+"];
                    } else {
                      return [item.player_data?.rank.name, ""];
                    }
                  }, [item]);
                  return (
                    <span className="font-bold">
                      {item.player_data?.rank.name != "DEFAULT" && (
                        <>
                          <span style={{ color: item.player_data?.rank.name_color as string }}>
                            {"[" + beforePlus}
                          </span>
                          <span style={{ color: item.player_data?.rank.plus_color as string }}>
                            {plus}
                          </span>
                          <span style={{ color: item.player_data?.rank.name_color as string }}>
                            {"] "}
                          </span>
                        </>
                      )}
                      <span style={{ color: item.player_data?.rank.name_color as string }}>
                        {item.name}
                      </span>
                    </span>
                  );
                }

                return (
                  <Table.Row key={item.toString() + index.toString()}>
                    <Table.RowHeaderCell>
                      <PlayerName item={item}></PlayerName>
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
                  </Table.Row>
                );
              })}
          </Table.Body>
        </Table.Root>
      )}
    </div>
  );
}
