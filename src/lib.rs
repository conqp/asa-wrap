use std::collections::HashMap;

const SERVER_SETTINGS_SECTION: &str = "ServerSettings";
const ACTIVE_MODS_KEY: &str = "ActiveMods";

type Section = HashMap<String, Option<String>>;
type IniFile = HashMap<String, Section>;

pub trait GameUserSettings {
    fn get_server_settings(&self) -> Option<Section>;

    fn get_server_setting(&self, key: &str) -> Option<String> {
        self.get_server_settings()
            .and_then(|server_settings| server_settings.get(key).and_then(Clone::clone))
    }

    fn active_mods(&self) -> Option<String> {
        self.get_server_setting(ACTIVE_MODS_KEY)
    }
}

impl GameUserSettings for IniFile {
    fn get_server_settings(&self) -> Option<Section> {
        self.get(SERVER_SETTINGS_SECTION).cloned()
    }
}
