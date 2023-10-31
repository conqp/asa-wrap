use std::collections::HashMap;

const SERVER_SETTINGS_SECTION: &str = "ServerSettings";
const ACTIVE_MODS_KEY: &str = "ActiveMods";
pub const MAX_PLAYERS: &str = "MaxPlayers";
pub const SERVER_PASSWORD: &str = "ServerPassword";
pub const SERVER_ADMIN_PASSWORD: &str = "ServerAdminPassword";

pub type Section = HashMap<String, Option<String>>;
pub type IniFile = HashMap<String, Section>;

pub trait GameUserSettings {
    fn get_server_settings(&self) -> Option<Section>;

    fn get_server_setting(&self, key: &str) -> Option<String> {
        self.get_server_settings()
            .and_then(|server_settings| server_settings.get(key).and_then(Clone::clone))
    }

    fn active_mods(&self) -> Option<String> {
        self.get_server_setting(&ACTIVE_MODS_KEY.to_ascii_lowercase())
    }

    fn server_password(&self) -> Option<String> {
        self.get_server_setting(&SERVER_PASSWORD.to_ascii_lowercase())
    }

    fn server_admin_password(&self) -> Option<String> {
        self.get_server_setting(&SERVER_ADMIN_PASSWORD.to_ascii_lowercase())
    }

    fn max_players(&self) -> Option<String> {
        self.get_server_setting(&MAX_PLAYERS.to_ascii_lowercase())
    }
}

impl GameUserSettings for IniFile {
    fn get_server_settings(&self) -> Option<Section> {
        self.get(&SERVER_SETTINGS_SECTION.to_ascii_lowercase())
            .cloned()
    }
}
