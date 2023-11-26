use std::collections::HashMap;

const SERVER_SETTINGS_SECTION: &str = "ServerSettings";
const SCRIPT_ENGINE_GAME_SESSION_SECTION: &str = "/Script/Engine.GameSession";
const ACTIVE_MODS_KEY: &str = "ActiveMods";
const MAX_PLAYERS: &str = "MaxPlayers";
pub const SERVER_PASSWORD: &str = "ServerPassword";
pub const SERVER_ADMIN_PASSWORD: &str = "ServerAdminPassword";

pub type Section = HashMap<String, Option<String>>;
pub type IniFile = HashMap<String, Section>;

pub trait GameUserSettings {
    fn server_settings_section(&self) -> Option<&Section>;

    fn script_engine_game_session_section(&self) -> Option<&Section>;

    fn server_setting(&self, key: &str) -> Option<&str> {
        self.server_settings_section()
            .and_then(|section| section.get(key).and_then(Option::as_deref))
    }

    fn script_engine_game_session(&self, key: &str) -> Option<&str> {
        self.script_engine_game_session_section()
            .and_then(|section| section.get(key).and_then(Option::as_deref))
    }

    fn active_mods(&self) -> Option<&str> {
        self.server_setting(&ACTIVE_MODS_KEY.to_ascii_lowercase())
    }

    fn server_password(&self) -> Option<&str> {
        self.server_setting(&SERVER_PASSWORD.to_ascii_lowercase())
    }

    fn server_admin_password(&self) -> Option<&str> {
        self.server_setting(&SERVER_ADMIN_PASSWORD.to_ascii_lowercase())
    }

    fn max_players(&self) -> Option<&str> {
        self.script_engine_game_session(&MAX_PLAYERS.to_ascii_lowercase())
    }
}

impl GameUserSettings for IniFile {
    fn server_settings_section(&self) -> Option<&Section> {
        self.get(&SERVER_SETTINGS_SECTION.to_ascii_lowercase())
    }

    fn script_engine_game_session_section(&self) -> Option<&Section> {
        self.get(&SCRIPT_ENGINE_GAME_SESSION_SECTION.to_ascii_lowercase())
    }
}
