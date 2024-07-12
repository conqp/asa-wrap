use asa_wrap::{
    GameUserSettings, ScriptEngineGameSession, ServerSettings, SERVER_ADMIN_PASSWORD,
    SERVER_PASSWORD,
};
use clap::Parser;
use env_logger::init;
use ini::Ini;
use log::{error, trace};
use std::process::{exit, Command};

const GAME_USER_SETTINGS: &str = "ShooterGame/Saved/Config/WindowsServer/GameUserSettings.ini";
const XVFB_RUN: &str = "/usr/bin/xvfb-run";
const WINE: &str = "/usr/bin/wine";
const SERVER_EXE: &str = "ShooterGame/Binaries/Win64/ArkAscendedServer.exe";
const DEFAULT_ARK: &str = "TheIsland_WP";
const LISTEN_ARG: &str = "listen";
const DEFAULT_SERVER_PORT: u16 = 7777;
const DEFAULT_QUERY_PORT: u16 = 27015;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, help = "Path to the GameUserSettings.ini", default_value = GAME_USER_SETTINGS)]
    game_user_settings: String,
    #[arg(short, long, help = "Path to the xvfb-run binary", default_value = XVFB_RUN)]
    xvfb_run: String,
    #[arg(short, long, help = "Path to the wine binary", default_value = WINE)]
    wine: String,
    #[arg(short, long, help = "Path to the server's exe", default_value = SERVER_EXE)]
    server_exe: String,
    #[arg(short, long, help = "Name of the ark", default_value = DEFAULT_ARK)]
    ark: String,
    #[arg(short, long, help = "Use BattlEye", default_value_t = false)]
    battleye: bool,
    #[arg(short, long, help = "Server port", default_value_t = DEFAULT_SERVER_PORT)]
    port: u16,
    #[arg(short, long, help = "Query port", default_value_t = DEFAULT_QUERY_PORT)]
    query_port: u16,
    #[arg(short, long, help = "Max players")]
    max_players: Option<u8>,
    #[arg(
        short = 't',
        long,
        help = "Additional attributes (those separated by '?')"
    )]
    attributes: Vec<String>,
    #[allow(clippy::struct_field_names)]
    #[arg(long, help = "Additional arguments")]
    args: Vec<String>,
    #[arg(index = 1, help = "The server name")]
    session_name: String,
}

impl Args {
    pub fn command(&self) -> Command {
        let game_user_settings = Ini::load_from_file(&self.game_user_settings)
            .map_err(|error| error!("{error}"))
            .unwrap_or_default();
        trace!("Settings: {game_user_settings:?}");

        #[cfg(target_os = "windows")]
        let mut command = Command::new(&self.server_exe);
        #[cfg(target_os = "linux")]
        let mut command = {
            let mut command = Command::new(&self.xvfb_run);
            command.arg(&self.wine).arg(&self.server_exe);
            command
        };

        command.arg(self.attributes(&game_user_settings));

        if !self.battleye {
            command.arg("-NoBattlEye");
        }

        if let Some(max_players) = self.max_players {
            command.arg(format!("-WinLiveMaxPlayers={max_players}"));
        } else if let Some(max_players) = game_user_settings
            .script_engine_game_session()
            .and_then(ScriptEngineGameSession::max_players)
        {
            command.arg(format!("-WinLiveMaxPlayers={max_players}"));
        }

        if let Some(mods) = game_user_settings
            .server_settings()
            .and_then(ServerSettings::active_mods)
        {
            command.arg(format!("-mods={mods}"));
        }

        command
    }

    fn attributes(&self, game_user_settings: &Ini) -> String {
        let mut attributes = Vec::new();
        attributes.push(self.ark.clone());
        attributes.push(LISTEN_ARG.into());
        attributes.push(format!("SessionName={}", self.session_name));

        if let Some(server_password) = game_user_settings
            .server_settings()
            .and_then(ServerSettings::server_password)
        {
            attributes.push(format!("{SERVER_PASSWORD}={server_password}"));
        }

        attributes.push(format!("Port={}", self.port));
        attributes.push(format!("QueryPort={}", self.query_port));
        attributes.extend_from_slice(&self.attributes);

        if let Some(server_admin_password) = game_user_settings
            .server_settings()
            .and_then(ServerSettings::server_admin_password)
        {
            attributes.push(format!("{SERVER_ADMIN_PASSWORD}={server_admin_password}"));
        }

        attributes.join("?")
    }
}

fn main() {
    init();
    exit(
        Args::parse()
            .command()
            .spawn()
            .unwrap_or_else(|error| {
                error!("{error}");
                exit(3);
            })
            .wait()
            .unwrap_or_else(|error| {
                error!("{error}");
                exit(4);
            })
            .code()
            .unwrap_or(255),
    )
}
