use ini::{Ini, Properties};

const SERVER_SETTINGS_SECTION: &str = "ServerSettings";
const SCRIPT_ENGINE_GAME_SESSION_SECTION: &str = "/Script/Engine.GameSession";
const ACTIVE_MODS_KEY: &str = "ActiveMods";
const MAX_PLAYERS: &str = "MaxPlayers";
pub const SERVER_PASSWORD: &str = "ServerPassword";
pub const SERVER_ADMIN_PASSWORD: &str = "ServerAdminPassword";

pub trait GameUserSettings {
    fn server_settings(&self) -> Option<&Properties>;

    fn script_engine_game_session(&self) -> Option<&Properties>;
}

pub trait ServerSettings {
    fn active_mods(&self) -> Option<&str>;

    fn server_password(&self) -> Option<&str>;

    fn server_admin_password(&self) -> Option<&str>;
}

pub trait ScriptEngineGameSession {
    fn max_players(&self) -> Option<&str>;
}

impl GameUserSettings for Ini {
    fn server_settings(&self) -> Option<&Properties> {
        self.section(Some(SERVER_SETTINGS_SECTION.to_ascii_lowercase()))
    }

    fn script_engine_game_session(&self) -> Option<&Properties> {
        self.section(Some(
            SCRIPT_ENGINE_GAME_SESSION_SECTION.to_ascii_lowercase(),
        ))
    }
}

impl ServerSettings for Properties {
    fn active_mods(&self) -> Option<&str> {
        self.get(ACTIVE_MODS_KEY.to_ascii_lowercase())
    }

    fn server_password(&self) -> Option<&str> {
        self.get(SERVER_PASSWORD.to_ascii_lowercase())
    }

    fn server_admin_password(&self) -> Option<&str> {
        self.get(SERVER_ADMIN_PASSWORD.to_ascii_lowercase())
    }
}

impl ScriptEngineGameSession for Properties {
    fn max_players(&self) -> Option<&str> {
        self.get(MAX_PLAYERS.to_ascii_lowercase())
    }
}
