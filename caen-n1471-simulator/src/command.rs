// Copyright (c) 2024 evopro Innovation 

use std::fmt;

pub enum Command {
    // Rust enums are powerful, the variant can hold different values too
    // Read more: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    ModuleMonitorCommand {
        param: String,
    },
    ModuleSetCommand {
        param: String,
        value: Option<String>, // there may be some set commands without value
    },
    ChannelMonitorCommand {
        channel: usize,
        param: String,
    },
    ChannelSetCommand {
        channel: usize,
        param: String,
        value: Option<String>, // there may be some set commands without value
    },
    WrongFormatCommand,
}

impl fmt::Display for Command {
    // This implementation allows a Command object to be used in format!, println!, etc.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::ModuleMonitorCommand { param } => {
                write!(f, "Read module param {param}")
            }
            Command::ModuleSetCommand { param, value } => {
                if let Some(value_string) = value {
                    write!(f, "Set module param {param} to {value_string}")
                } else {
                    write!(f, "Set module param {param}")
                }
            }
            Command::ChannelMonitorCommand { channel, param } => {
                write!(f, "Read channel {channel} param {param}")
            }
            Command::ChannelSetCommand {
                channel,
                param,
                value,
            } => {
                if let Some(value_string) = value {
                    write!(f, "Set channel {channel} param {param} to {value_string}")
                } else {
                    write!(f, "Set channel {channel} param {param}")
                }
            }
            Command::WrongFormatCommand => {
                write!(f, "Wrong command format")
            }
        }
    }
}
