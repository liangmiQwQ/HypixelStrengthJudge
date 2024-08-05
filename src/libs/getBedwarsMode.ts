export function getBedwarsMode(mode: string) {
  const modeDictionary = {
    BEDWARS_FOUR_FOUR: "4s",
    BEDWARS_FOUR_THREE: "3s",
    BEDWARS_TWO_FOUR: "4v4",
    BEDWARS_EIGHT_TWO: "Double",
    BEDWARS_EIGHT_ONE: "Solo",
  };

  return modeDictionary[mode as keyof typeof modeDictionary];
}
