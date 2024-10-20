use std::process::Command;
use std::error::Error;
use crate::binary::SUIDBinary;
use crate::config::Config;

pub struct SuidChecker {
    config: Config,
}

impl SuidChecker {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }

    pub fn is_linux() -> bool {
        std::env::consts::OS == "linux"
    }

    pub fn check_suid_binaries(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let output = Command::new("find")
            .arg("/")
            .arg("-perm")
            .arg("-u=s")
            .arg("-type")
            .arg("f")
            .output()?;

        let suid_binaries = String::from_utf8_lossy(&output.stdout);
        let mut vulnerable_binaries = Vec::new();

        for line in suid_binaries.lines() {
            let binary = SUIDBinary::new(line, false);
            if binary.check_vulnerability(&self.config.gtfobins) {
                vulnerable_binaries.push(binary.path);
            }
        }

        Ok(vulnerable_binaries)
    }
}
