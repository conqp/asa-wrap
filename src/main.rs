use asa_wrap::GameUserSettings;
use clap::Parser;
use env_logger::init;
use ini::ini;
use log::debug;
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
    #[arg(short, long, help = "Additional attributes, separated by '?'")]
    attributes: Vec<String>,
    #[arg(short, long, help = "Additional arguments")]
    args: Vec<String>,
    #[arg(index = 1, help = "The server name")]
    session_name: String,
}

impl Args {
    pub fn command(&self) -> Command {
        #[cfg(target_os = "windows")]
        let mut command = Command::new(&self.server_exe);
        #[cfg(target_os = "linux")]
        let mut command = Command::new(&self.xvfb_run);
        #[cfg(target_os = "linux")]
        command.arg(&self.wine).arg(&self.server_exe.clone());

        command.arg(self.command_attributes());

        if !self.battleye {
            command.arg("-NoBattlEye");
        }

        if let Some(mods) = self.mods() {
            command.arg(format!(r#"-mods="{mods}""#));
        }

        command
    }

    fn command_attributes(&self) -> String {
        let mut command_attributes = Vec::new();
        command_attributes.push(self.map.clone());
        command_attributes.push(LISTEN_ARG.into());
        command_attributes.push(format!("SessionName={}", self.session_name));
        command_attributes.extend_from_slice(&self.attributes);
        command_attributes.join("?")
    }

    fn mods(&self) -> Option<String> {
        let game_user_settings = ini!(&self.game_user_settings);
        debug!("Settings: {game_user_settings:?}");
        let mods = game_user_settings.active_mods();
        debug!("Mods: {mods:?}");
        mods
    }
}

fn main() {
    init();
    if let Some(exit_code) = Args::parse()
        .command()
        .spawn()
        .expect("Failed to run subprocess.")
        .wait()
        .expect("Subprocess terminated unexpectedly")
        .code()
    {
        exit(exit_code)
    }
}
