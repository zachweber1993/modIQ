/// Displays CLI help.
///
/// Owns static usage text only (`GOVERNANCE.md`, CLI: "user
/// interaction... must never contain business logic").
pub struct HelpCommand;

impl HelpCommand {
    pub fn run() -> String {
        format!(
            concat!(
                "modIQ CLI {}\n",
                "\n",
                "Usage:\n",
                "  modiq-cli assess <path>     Run an assessment against a filesystem path or .zip archive\n",
                "  modiq-cli retrieve <key>    Retrieve a previously-stored assessment report\n",
                "  modiq-cli help              Show this message\n",
                "  modiq-cli version           Show the CLI version",
            ),
            env!("CARGO_PKG_VERSION")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_mentions_all_four_commands() {
        let output = HelpCommand::run();

        assert!(output.contains("assess"));
        assert!(output.contains("retrieve"));
        assert!(output.contains("help"));
        assert!(output.contains("version"));
    }
}
