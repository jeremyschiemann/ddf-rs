use std::fmt::Display;
use strum::{EnumIter, EnumString};

#[derive(EnumIter, EnumString)]
pub enum Command {
    Quit,
    Stat,
    Reload,
    ResetSeen,
    ShowAll,
    Help,
    NextQuestion,
    Other(String),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Quit => write!(f, "q (quit)"),
            Command::Stat => write!(f, "stat"),
            Command::Reload => write!(f, "reload"),
            Command::ResetSeen => write!(f, "reset-seen"),
            Command::ShowAll => write!(f, "show-all"),
            Command::Help => write!(f, "help"),
            Command::NextQuestion => write!(f, "no command = next question"),
            Command::Other(_) => write!(f, ""),
        }
    }
}

impl Command {
    pub fn from_string(input: &str) -> Command {
        match input {
            "q" => Command::Quit,
            "stat" => Command::Stat,
            "reload" => Command::Reload,
            "reset-seen" => Command::ResetSeen,
            "show-all" => Command::ShowAll,
            "help" => Command::Help,
            "" => Command::NextQuestion,
            _ => Command::Other(input.to_string()),
        }
    }
}
