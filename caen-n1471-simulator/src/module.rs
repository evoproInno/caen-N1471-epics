// Copyright (c) 2024 evopro Innovation 

use crate::command::Command;

enum PowerDownMode {
    Ramp,
    Kill,
}

struct Channel {
    pub v_set: f64,
    pub v_mon: f64,
    pub i_set: f64,
    pub i_mon: f64,
    pub max_v_set: u32,
    pub ramp_up: u32,
    pub ramp_down: u32,
    pub trip: f64,
    pub power_down: PowerDownMode,
    pub polarity: char,
    pub status: u32,
}

impl Channel {
    pub fn new() -> Self {
        Channel {
            v_set: 0.0,
            v_mon: 0.0,
            i_set: 0.0,
            i_mon: 0.0,
            max_v_set: 942,
            ramp_up: 24,
            ramp_down: 98,
            trip: 10.0,
            power_down: PowerDownMode::Ramp,
            polarity: '+',
            status: 0,
        }
    }

    pub fn turn_on(&mut self) {
        self.status |= 1; // set LSB to 1
    }

    pub fn turn_off(&mut self) {
        self.status &= !1; // set LSB to 0
    }
}

pub struct Module {
    channels: [Channel; Self::NUMBER_OF_CHANNELS],
    interlock_mode: String, // TODO: replace with enum (OPEN/CLOSED)
    control_mode: String,   // TODO: replace with enum (LOCAL/REMOTE)
    alarm_state: String,    // TODO: replace with struct with bool fields and a converter method
}

pub enum Response {
    Ok,
    OkWithValue(String),
    WrongFormat,
    WrongChannel,
    WrongParam,
    WrongValue,
    LocalMode,
}

impl Module {
    const NUMBER_OF_CHANNELS: usize = 2;

    pub fn new() -> Self {
        Self {
            channels: [Channel::new(), Channel::new()],
            interlock_mode: "OPEN".to_string(),
            control_mode: "REMOTE".to_string(),
            alarm_state: "15".to_string(),
        }
    }

    pub fn execute(&mut self, command: &Command) -> Response {
        match command {
            Command::WrongFormatCommand => Response::WrongFormat,
            Command::ModuleMonitorCommand { param } => self.get_module_param(param),
            Command::ModuleSetCommand { param, value } => self.set_module_param(param, value),
            Command::ChannelMonitorCommand { channel, param } => {
                self.get_param_for_channel(channel, param)
            }
            Command::ChannelSetCommand {
                channel,
                param,
                value,
            } => self.set_param_for_channel(channel, param, value),
        }
    }

    fn get_module_param(&self, param: &str) -> Response {
        match param {
            "BDNAME" => Response::OkWithValue("N1471A".to_string()),
            "BDNCH" => Response::OkWithValue(Self::NUMBER_OF_CHANNELS.to_string()),
            "BDFREL" => Response::OkWithValue("12.3".to_string()),
            "BDSNUM" => Response::OkWithValue("12345".to_string()),
            "BDILK" => Response::OkWithValue("YES".to_string()),
            "BDILKM" => Response::OkWithValue(self.interlock_mode.clone()),
            "BDCTR" => Response::OkWithValue(self.control_mode.clone()),
            "BDTERM" => Response::OkWithValue("ON".to_string()),
            "BDALARM" => Response::OkWithValue("00000".to_string()),
            _ => Response::WrongParam,
        }
    }

    fn set_module_param(&mut self, param: &str, value: &Option<String>) -> Response {
        if self.control_mode == "LOCAL" {
            return Response::LocalMode;
        }

        match param {
            "BDILKM" => {
                if let Some(interlock_mode) = value {
                    match interlock_mode.as_str() {
                        "OPEN" | "CLOSED" => {
                            self.interlock_mode = interlock_mode.to_string();
                            return Response::Ok;
                        }
                        _ => (),
                    }
                }
                Response::WrongValue
            }
            "BDCLR" => {
                self.alarm_state = "0".to_string();
                Response::Ok
            }
            _ => Response::WrongParam,
        }
    }

    fn get_param_for_channel(&self, channel: &usize, param: &str) -> Response {
        if *channel >= Self::NUMBER_OF_CHANNELS {
            return Response::WrongChannel;
        }
        let module_channel = &self.channels[*channel];
        match param {
            "VSET" => Response::OkWithValue(format!("{:06.1}", module_channel.v_set)),
            "VMON" => Response::OkWithValue(format!("{:06.1}", module_channel.v_mon)),
            "ISET" => Response::OkWithValue(format!("{:07.3}", module_channel.i_set)),
            "IMON" => Response::OkWithValue(format!("{:07.3}", module_channel.i_mon)),
            "IMRANGE" => Response::OkWithValue("HIGH".to_string()), // LOW mode is not supported
            "MAXV" => Response::OkWithValue(format!("{:04}", module_channel.max_v_set)),
            "RUP" => Response::OkWithValue(format!("{:03}", module_channel.ramp_up)),
            "RDW" => Response::OkWithValue(format!("{:03}", module_channel.ramp_down)),
            "TRIP" => Response::OkWithValue(format!("{:06.1}", module_channel.trip)),
            "PDWN" => Response::OkWithValue(match module_channel.power_down {
                PowerDownMode::Kill => "KILL".to_string(),
                PowerDownMode::Ramp => "RAMP".to_string(),
            }),
            "POL" => Response::OkWithValue(module_channel.polarity.to_string()),
            "STAT" => Response::OkWithValue(format!("{:05}", module_channel.status)),
            _ => Response::WrongParam,
        }
    }

    fn set_param_for_channel(
        &mut self,
        channel: &usize,
        param: &str,
        value: &Option<String>,
    ) -> Response {
        if *channel >= Self::NUMBER_OF_CHANNELS {
            return Response::WrongChannel;
        }
        if self.control_mode == "LOCAL" {
            return Response::LocalMode;
        }

        let module_channel = &mut self.channels[*channel];

        match param {
            "VSET" => {
                if let Some(existing_value) = value {
                    if let Ok(parsed_value) = existing_value.parse::<f64>() {
                        module_channel.v_set = parsed_value;
                        return Response::Ok;
                    }
                }
                Response::WrongValue
            }
            "ISET" => {
                if let Some(existing_value) = value {
                    if let Ok(parsed_value) = existing_value.parse::<f64>() {
                        module_channel.i_set = parsed_value;
                        return Response::Ok;
                    }
                }
                Response::WrongValue
            }
            "MAXV" => {
                if let Some(existing_value) = value {
                    if let Ok(parsed_value) = existing_value.parse::<u32>() {
                        module_channel.max_v_set = parsed_value;
                        return Response::Ok;
                    }
                }
                Response::WrongValue
            }
            "RUP" => {
                if let Some(existing_value) = value {
                    if let Ok(parsed_value) = existing_value.parse::<u32>() {
                        module_channel.ramp_up = parsed_value;
                        return Response::Ok;
                    }
                }
                Response::WrongValue
            }
            "RDW" => {
                if let Some(existing_value) = value {
                    if let Ok(parsed_value) = existing_value.parse::<u32>() {
                        module_channel.ramp_down = parsed_value;
                        return Response::Ok;
                    }
                }
                Response::WrongValue
            }
            "TRIP" => {
                if let Some(existing_value) = value {
                    if let Ok(parsed_value) = existing_value.parse::<f64>() {
                        module_channel.i_set = parsed_value;
                        return Response::Ok;
                    }
                }
                Response::WrongValue
            }
            "PDWN" => {
                if let Some(existing_value) = value {
                    match existing_value.as_str() {
                        "RAMP" => {
                            module_channel.power_down = PowerDownMode::Ramp;
                            return Response::Ok;
                        }
                        "KILL" => {
                            module_channel.power_down = PowerDownMode::Kill;
                            return Response::Ok;
                        }
                        _ => (),
                    }
                }
                Response::WrongValue
            }
            "ON" => {
                module_channel.turn_on();
                Response::Ok
            }
            "OFF" => {
                module_channel.turn_off();
                Response::Ok
            }
            _ => Response::WrongParam,
        }
    }
}

impl Response {
    pub fn encode(&self, address: &str) -> String {
        let response_part = match self {
            Response::Ok => "CMD:OK".to_string(),
            Response::OkWithValue(value) => format!("CMD:OK,VAL:{}", value),
            Response::WrongFormat => "CMD:ERR".to_string(),
            Response::WrongChannel => "CH:ERR".to_string(),
            Response::WrongParam => "PAR:ERR".to_string(),
            Response::WrongValue => "VAL:ERR".to_string(),
            Response::LocalMode => "LOC:ERR".to_string(),
        };
        format!("#BD:{address},{response_part}\r\n")
    }
}
