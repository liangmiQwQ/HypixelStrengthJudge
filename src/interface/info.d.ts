interface info {
  player_data: null | PlayerData;
  location: Location;
  party_info: null | PartyInfo;
}

interface PlayerData {
  name: String;
  rank: String;
  bw_fkdr: number;
  bw_level: number;
  lobby_level: number;
}

interface Location {
  game_type: String;
  server_type: "LOBBY" | "GAME"; // "LOBBY" or "GAME", if "server" starts with "dynamiclobby", it's "LOBBY"
  game_mode: String | null; // "BEDWARS_FOUR_FOUR" etc.
}

interface PartyInfo {
  user_job: "LEADER" | "OTHER";
  players: String[];
}
