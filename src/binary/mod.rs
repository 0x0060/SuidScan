#[derive(Debug)]
pub struct SUIDBinary {
    pub path: String,
    pub is_vulnerable: bool,
}

impl SUIDBinary {
    pub fn new(path: &str, is_vulnerable: bool) -> Self {
        Self {
            path: path.to_string(),
            is_vulnerable,
        }
    }

    pub fn check_vulnerability(&self, vulnerable_binaries: &[String]) -> bool {
        vulnerable_binaries.iter().any(|binary| self.path.ends_with(binary))
    }
}
