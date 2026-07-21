/// Displays version information.
pub struct VersionCommand;

impl VersionCommand {
    pub fn run() -> String {
        format!("modiq-cli {}", env!("CARGO_PKG_VERSION"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_reports_the_crate_version() {
        assert_eq!(
            VersionCommand::run(),
            format!("modiq-cli {}", env!("CARGO_PKG_VERSION"))
        );
    }
}
