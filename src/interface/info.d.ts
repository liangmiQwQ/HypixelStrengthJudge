interface info {
  player_data: null | PlayerData;
  location: Location;
  party_info: null | PartyInfo;
}

interface PlayerData {
  name: string;
  rank: Rank;
  bw_fkdr: number;
  bw_level: number;
  lobby_level: number;
  bblr: string;
  win_streak: string;
}

interface PartyPlayerData {
  name: string;
  player_data: null | PlayerData;
}

interface Rank {
  name: string;
  plus_color: null | String;
  name_color: string;
}

interface Location {
  game_type: string;
  server_type: "LOBBY" | "GAME"; // "LOBBY" or "GAME", if "server" starts with "dynamiclobby", it's "LOBBY"
  game_mode: string | null; // "BEDWARS_FOUR_FOUR" etc.
}

interface PartyInfo {
  user_job: "LEADER" | "OTHER";
  players: PartyPlayerData[];
}
