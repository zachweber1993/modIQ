use crate::commands::{AssessCommand, HelpCommand, VersionCommand};

/// The process exit code categories `Application::run` maps every
/// command outcome to.
///
/// Kept intentionally small and predictable (Sprint 6 authorization,
/// Exit Codes): `Success` for a completed assessment (Findings present
/// or not — modIQ explains, it does not judge), `ExecutionFailure` for
/// an assessment that was attempted but aborted during execution, and
/// `InvalidUsage` for a problem caught before execution ever began —
/// CLI-level usage errors and `AssessmentInputError` alike, since both
/// represent "the input itself was invalid," not a runtime failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Success = 0,
    ExecutionFailure = 1,
    InvalidUsage = 2,
}

impl ExitCode {
    pub fn code(self) -> i32 {
        self as i32
    }
}

/// Root application entry point.
///
/// Owns command dispatch only (`GOVERNANCE.md`, CLI: "user interaction,
/// command execution, platform entry point"); it must never itself
/// evaluate an Assessment or reimplement anything `AssessmentService`
/// already does. Three concrete commands exist today, dispatched by one
/// direct match — no command trait, registry, or lookup table, per this
/// platform's "capability before abstraction" discipline: two commands
/// is not evidence a dispatch abstraction is justified, and three still
/// isn't.
pub struct Application;

impl Application {
    /// Dispatches on the first CLI argument. `args` is expected to
    /// already have the program name stripped (`main` does this via
    /// `std::env::args().skip(1)`).
    pub fn run(args: &[String]) -> (String, ExitCode) {
        match args.first().map(String::as_str) {
            None | Some("help") => (HelpCommand::run(), ExitCode::Success),
            Some("version") => (VersionCommand::run(), ExitCode::Success),
            Some("assess") => match args.get(1) {
                Some(path) => AssessCommand::run(path),
                None => (
                    format!(
                        "{}\n\nerror: `assess` requires a path argument",
                        HelpCommand::run()
                    ),
                    ExitCode::InvalidUsage,
                ),
            },
            Some(other) => (
                format!(
                    "{}\n\nerror: unrecognized command `{other}`",
                    HelpCommand::run()
                ),
                ExitCode::InvalidUsage,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_arguments_shows_help_and_succeeds() {
        let (message, exit_code) = Application::run(&[]);

        assert_eq!(exit_code, ExitCode::Success);
        assert!(message.contains("Usage"));
    }

    #[test]
    fn help_command_succeeds() {
        let (_, exit_code) = Application::run(&["help".to_string()]);

        assert_eq!(exit_code, ExitCode::Success);
    }

    #[test]
    fn version_command_succeeds() {
        let (message, exit_code) = Application::run(&["version".to_string()]);

        assert_eq!(exit_code, ExitCode::Success);
        assert!(message.contains("modiq-cli"));
    }

    #[test]
    fn assess_without_a_path_is_invalid_usage() {
        let (message, exit_code) = Application::run(&["assess".to_string()]);

        assert_eq!(exit_code, ExitCode::InvalidUsage);
        assert!(message.contains("requires a path argument"));
    }

    #[test]
    fn unrecognized_command_is_invalid_usage() {
        let (message, exit_code) = Application::run(&["bogus".to_string()]);

        assert_eq!(exit_code, ExitCode::InvalidUsage);
        assert!(message.contains("unrecognized command"));
    }
}
