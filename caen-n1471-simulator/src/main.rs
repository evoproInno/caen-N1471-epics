// Copyright (c) 2024 evopro Innovation 

use std::io::ErrorKind;
use std::time::Duration;

use module::Module;
use parser::Parser;
use regex::Regex;
use socat::SocatProcess;

mod command;
mod module;
mod parser;
mod socat;

fn main() {
    const MAX_COMMAND_LENGTH: usize = 50;
    const BAUD_RATE: u32 = 9600;

    let module_address = get_module_address();
    let mut command_parser = Parser::new(&module_address);

    let _socat_process = SocatProcess::start();

    let mut port = serialport::new(SocatProcess::INTERNAL_PORT_NAME, BAUD_RATE)
        .timeout(Duration::from_millis(1000)) // This sets the timeout for reading data.
        .open()
        .expect(format!("Failed to open port {}", SocatProcess::INTERNAL_PORT_NAME).as_str());

    println!(
        "Module#{} simulator is listening for commands at {}",
        module_address,
        SocatProcess::EXTERNAL_PORT_NAME
    );

    let mut module = Module::new();
    let mut input_buffer = vec![0; MAX_COMMAND_LENGTH];
    loop {
        match port.read(&mut input_buffer) {
            Ok(bytes_read) => {
                command_parser.receive(&input_buffer[..bytes_read]);

                if let Some(command) = command_parser.get_command() {
                    let response = module.execute(&command);
                    command_parser.clear();
                    let encoded_response = response.encode(&module_address);
                    println!("<- {encoded_response}");
                    port.write(&encoded_response.as_bytes().to_vec())
                        .expect("Failed to write response");
                }
            }
            Err(ref e) if e.kind() == ErrorKind::TimedOut => (), // Ignore read timeout
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        }
    }
}

fn get_module_address() -> String {
    // Read module address as the first argument...
    if let Some(address) = std::env::args().nth(1) {
        // Valid module address is between 00..31
        if address.len() == 2 && Regex::new(r"^([012]\d|3[01])$").unwrap().is_match(&address) {
            return address;
        } else {
            panic!("Module address must be between 00 and 31. Invalid input: {address}");
        }
    }
    // ... or use 00 as the default value
    "00".to_string()
}
