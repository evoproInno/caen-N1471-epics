// Copyright (c) 2024 evopro Innovation 

use crate::command::Command;
use regex::Regex;
use std::str;

/**
 * Receive and collect incoming bytes and attempt to parse them into a Command object
 */
pub struct Parser {
    module_address: String,
    command_buffer: Vec<u8>,
    command: Option<Command>,
}

impl Parser {
    // The associated functions for Parser structs are defined in this impl block: https://doc.rust-lang.org/book/ch05-03-method-syntax.html

    // Rust does not have constructors: https://doc.rust-lang.org/nomicon/constructors.html
    // This is just a simple associated function that returns a Parser object with explicitly initialized fields.
    // These are often (but not necessarily) called "new".
    // It is also public, so accessible where Parser is in the scope.
    pub fn new(address: &str) -> Self {
        Self {
            module_address: address.to_string(),
            command_buffer: Vec::new(),
            command: None,
        }
    }

    // This is a public method, that takes the "self" paramterer as a mutable reference, so it can change the object.
    // In C++ this would be a non-const member function.
    pub fn receive(&mut self, data: &[u8]) {
        // Iterate over incoming characters stored in input_buffer
        for character in data {
            match character {
                b'\n' => {
                    // When \n (LF) is received -> process data in command_buffer and respond
                    if let Ok(raw_command) = str::from_utf8(&self.command_buffer) {
                        self.command = self.process_command(raw_command);
                    }
                    // Clear buffer after processing
                    self.command_buffer.clear();
                }
                b'\r' => { /* Ignore \r (CR) */ }
                other_character => {
                    // Put any other characters into the command_buffer
                    // TODO: change command_buffer to a cirular buffer to prevent endless growing
                    self.command_buffer.push(*other_character);
                }
            }
        }
    }

    // This is a method with "self" passed as immutable reference, so it can't change the object.
    // In C++ this would be a const member function.
    // It's not public, so it's private by default. It can only be called from other methods.
    fn process_command(&self, raw_command: &str) -> Option<Command> {
        // Print incoming commands
        print!("-> {raw_command:<width$} | ", width = 40);

        // Check the address in the incoming command
        if Self::command_address_is_matching(raw_command, &self.module_address) {
            // Address is correct -> we need to handle this command -> parse & return
            let command = Self::parse_command(raw_command);
            println!("ACCEPT - {command}");
            Some(command)
        } else {
            // Address check failed -> we are not the recipient -> command is ignored
            println!("IGNORE - Module address wrong or not found");
            None
        }
    }

    // This is a non-method associated function, it does not have the "self" parameter.
    // In C++ this would be a static function.
    fn command_address_is_matching(raw_command: &str, module_address: &str) -> bool {
        raw_command.starts_with(format!("$BD:{},", module_address).as_str())
    }

    // Non-method associated function.
    fn parse_command(raw_command: &str) -> Command {
        // Strip the "$BD:XX," part and start parsing from "CMD"
        let command_part = &raw_command[7..];

        // Regex explanation here: https://regex101.com/r/hpLXUd/3
        let re = Regex::new(
            r"^CMD:(?P<type>MON|SET)(,CH:(?P<ch>\d))?,PAR:(?P<par>[A-Z]+)(,VAL:(?P<val>\S+))?$",
        )
        .unwrap();

        if let Some(captures) = re.captures(command_part) {
            // Command type (SET/MON) and parameter name must be matched
            let command_type = captures.name("type").unwrap().as_str();
            let command_param = captures.name("par").unwrap().as_str();

            // The channel and value parts may be missing, save Option values for now
            let command_channel_opt = captures.name("ch");
            let command_value_opt = captures.name("val").map(|s| s.as_str().to_string()); // convert to Option<String>

            // Decide between the 4 valid Command variants based on the command type (SET/MON)
            // and the availability of the channel field.
            // Then crate the actual variant with all the necessary extracted values.
            if command_type == "SET" {
                if let Some(channel_match) = command_channel_opt {
                    return Command::ChannelSetCommand {
                        channel: channel_match.as_str().parse().unwrap(), // extract channel as usize
                        param: command_param.to_string(),
                        value: command_value_opt,
                    };
                } else {
                    return Command::ModuleSetCommand {
                        param: command_param.to_string(),
                        value: command_value_opt,
                    };
                }
            } else if command_type == "MON" {
                if let Some(channel_match) = captures.name("ch") {
                    return Command::ChannelMonitorCommand {
                        channel: channel_match.as_str().parse().unwrap(), // extract channel as usize
                        param: command_param.to_string(),
                    };
                } else {
                    return Command::ModuleMonitorCommand {
                        param: command_param.to_string(),
                    };
                }
            }
        }

        // If the function gets here, it means that the command could not be parsed.
        // e.g. the regex did not match
        return Command::WrongFormatCommand;
    }

    // Public method with immutable reference
    pub fn get_command(&self) -> Option<&Command> {
        self.command.as_ref()
    }

    // Public method with mutable reference
    pub fn clear(&mut self) {
        self.command = None;
    }
}
