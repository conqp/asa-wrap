use asa_wrap::{GameUserSettings, IniFile, MAX_PLAYERS, SERVER_ADMIN_PASSWORD, SERVER_PASSWORD};
use clap::Parser;
use env_logger::init;
use ini::ini;
use log::trace;
use std::process::{exit, Command};

const GAME_USER_SETTINGS: &str = "ShooterGame/Saved/Config/WindowsServer/GameUserSettings.ini";
const XVFB_RUN: &str = "/usr/bin/xvfb-run";
const WINE: &str = "/usr/bin/wine";
const SERVER_EXE: &str = "ShooterGame/Binaries/Win64/ArkAscendedServer.exe";
const DEFAULT_MAP: &str = "TheIsland_WP";
const LISTEN_ARG: &str = "listen";

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
    #[arg(short, long, help = "Name of the map", default_value = DEFAULT_MAP)]
    map: String,
    #[arg(short, long, help = "Use BattlEye", default_value_t = false)]
    battleye: bool,
    #[arg(short, long, help = "Server port", default_value_t = 7777)]
    port: u16,
    #[arg(short, long, help = "Query port", default_value_t = 27015)]
    query_port: u16,
    #[arg(
        short = 't',
        long,
        help = "Additional attributes (those separated by '?')"
    )]
    attributes: Vec<String>,
    #[arg(short, long, help = "Additional arguments")]
    args: Vec<String>,
    #[arg(index = 1, help = "The server name")]
    session_name: String,
}

impl Args {
    pub fn command(&self) -> Command {
        let game_user_settings = ini!(&self.game_user_settings);
        trace!("Settings: {game_user_settings:?}");

        #[cfg(target_os = "windows")]
        let mut command = Command::new(&self.server_exe);
        #[cfg(target_os = "linux")]
        let mut command = {
            let mut command = Command::new(&self.xvfb_run);
            command.arg(&self.wine).arg(&self.server_exe.clone());
            command
        };

        command.arg(self.attributes(&game_user_settings));

        if !self.battleye {
            command.arg("-NoBattlEye");
        }

        if let Some(mods) = game_user_settings.active_mods() {
            command.arg(format!(r#"-mods="{mods}""#));
        }

        command
    }

    fn attributes(&self, game_user_settings: &IniFile) -> String {
        let mut attributes = Vec::new();
        attributes.push(self.map.clone());
        attributes.push(LISTEN_ARG.into());
        attributes.push(format!("SessionName={}", self.session_name));

        if let Some(server_password) = game_user_settings.server_password() {
            attributes.push(format!("{SERVER_PASSWORD}={server_password}"));
        }

        attributes.push(format!("Port={}", self.port));
        attributes.push(format!("QueryPort={}", self.query_port));

        if let Some(max_players) = game_user_settings.max_players() {
            attributes.push(format!("{MAX_PLAYERS}={max_players}"));
        }

        attributes.extend_from_slice(&self.attributes);

        if let Some(server_admin_password) = game_user_settings.server_admin_password() {
            attributes.push(format!("{SERVER_ADMIN_PASSWORD}={server_admin_password}"));
        }

        attributes.join("?")
    }
}

fn main() {
    init();
    if let Some(exit_code) = Args::parse()
        .command()
        .spawn()
        .expect("Failed to run subprocess.")
        .wait()
        .expect("Subprocess terminated unexpectedly.")
        .code()
    {
        exit(exit_code)
    }
}
