interface info {
  player_data: null | ReturnPlayerData[];
  personal_data: PersonalData;
  party_info: null | PartyInfo;
}

interface PersonalData {
  location: Location;
  data: null | PlayerData;
}

interface PlayerData {
  name: string;
  rank: Rank;
  bw_fkdr: string;
  bw_level: number;
  lobby_level: number;
  bblr: string;
  win_streak: string;
}

interface ReturnPlayerData {
  name: string;
  data: null | PlayerData;
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
  server_type: "LOBBY" | "GAME" | "UNKNOWN"; // "LOBBY" or "GAME", if "server" starts with "dynamiclobby", it's "LOBBY"
  game_mode: string | null; // "BEDWARS_FOUR_FOUR" etc.
  map: string | null;
}

interface PartyInfo {
  user_job: "LEADER" | "OTHER";
  players: PartyPlayerData[];
}
