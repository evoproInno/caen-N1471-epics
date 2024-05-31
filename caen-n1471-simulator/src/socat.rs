// Copyright (c) 2024 evopro Innovation 

use std::process::{Child, Command, Stdio};
use std::time::Duration;

pub struct SocatProcess {
    process: Child,
}

impl SocatProcess {
    pub const EXTERNAL_PORT_NAME: &'static str = "/dev/simulator_port";
    pub const INTERNAL_PORT_NAME: &'static str = "/tmp/internal_simu_port";
    pub fn start() -> Self {
        let process = Command::new("socat")
            .arg(format!("pty,raw,echo=0,link={}", Self::INTERNAL_PORT_NAME))
            .arg(format!("pty,raw,echo=0,link={}", Self::EXTERNAL_PORT_NAME))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to spawn socat");

        Self::wait_for_port_created();

        Self { process }
    }

    fn wait_for_port_created() {
        // Block until the internal port is created
        // Give up after some number of iterations to prevent infinite loop
        let mut give_up_counter = 20;
        while give_up_counter > 0 && !Self::is_port_created() {
            std::thread::sleep(Duration::from_millis(100));
            give_up_counter -= 1;
        }
    }

    fn is_port_created() -> bool {
        std::fs::metadata(Self::INTERNAL_PORT_NAME).is_ok()
    }
}

impl Drop for SocatProcess {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}
