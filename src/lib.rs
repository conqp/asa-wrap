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
    fn server_settings(&self) -> Option<&Section>;

    fn script_engine_game_session(&self) -> Option<&Section>;
}

pub trait ServerSettings {
    fn active_mods(&self) -> Option<&str>;

    fn server_password(&self) -> Option<&str>;

    fn server_admin_password(&self) -> Option<&str>;
}

pub trait ScriptEngineGameSession {
    fn max_players(&self) -> Option<&str>;
}

impl GameUserSettings for IniFile {
    fn server_settings(&self) -> Option<&Section> {
        self.get(&SERVER_SETTINGS_SECTION.to_ascii_lowercase())
    }

    fn script_engine_game_session(&self) -> Option<&Section> {
        self.get(&SCRIPT_ENGINE_GAME_SESSION_SECTION.to_ascii_lowercase())
    }
}

impl ServerSettings for Section {
    fn active_mods(&self) -> Option<&str> {
        self.get(&ACTIVE_MODS_KEY.to_ascii_lowercase())
            .and_then(Option::as_deref)
    }

    fn server_password(&self) -> Option<&str> {
        self.get(&SERVER_PASSWORD.to_ascii_lowercase())
            .and_then(Option::as_deref)
    }

    fn server_admin_password(&self) -> Option<&str> {
        self.get(&SERVER_ADMIN_PASSWORD.to_ascii_lowercase())
            .and_then(Option::as_deref)
    }
}

impl ScriptEngineGameSession for Section {
    fn max_players(&self) -> Option<&str> {
        self.get(&MAX_PLAYERS.to_ascii_lowercase())
            .and_then(Option::as_deref)
    }
}
