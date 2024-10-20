use logger::{init_logger, log_error, log_info};
use suid_checker::SuidChecker;

fn main() {
    init_logger();
    log_info("Starting SUID binary scan...");

    if !SuidChecker::is_linux() {
        log_error("This script is intended to run only on Linux systems.");
        return;
    }

    let checker = SuidChecker::new();
    match checker.check_suid_binaries() {
        Ok(vulnerable_binaries) => {
            if vulnerable_binaries.is_empty() {
                log_info("No vulnerable SUID binaries found.");
            } else {
                for binary in vulnerable_binaries {
                    log_info(format!("{} is vulnerable to Priv Escalation", binary));
                }
            }
        }
        Err(e) => {
            log_error(format!("Error checking SUID binaries: {}", e));
        }
    }

    log_info("SUID binary scan completed.");
}
