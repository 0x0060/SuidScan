#[derive(Debug)]
pub struct Config {
    pub gtfobins: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            gtfobins: vec![
                "aa-exec", "ar", "ash", "bconsole", "bpftrace", "capsh", "chmod", "chown",
                "chroot", "clamscan", "cp", "cpan", "cpio", "debugfs", "dialog", "diff",
                "dmidecode", "docker", "ed", "efax", "emacs", "env", "ex", "file", "find",
                "finger", "fish", "flock", "gdb", "gimp", "git", "grep", "gtester", "gzip",
                "hd", "head", "hexdump", "highlight", "hping3", "install", "ionice", "ip",
                "julia", "ksh", "ksu", "less", "links", "loginctl", "logsave", "make", "man",
                "more", "mosquitto", "mysql", "nano", "nasm", "nawk", "passwd", "perl", "pkexec",
                "puppet", "python", "redis", "rsync", "sash", "scanmem", "sed", "service",
                "setfacl", "setlock", "sftp", "ssh-agent", "sudo", "systemctl", "tar", "tftp",
                "unshare", "vagrant", "vi", "vipw", "whois", "zsh",
            ]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
        }
    }
}
