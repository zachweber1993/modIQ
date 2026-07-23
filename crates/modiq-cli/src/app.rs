use crate::commands::{AssessCommand, HelpCommand, RetrieveCommand, VersionCommand};

/// Default root directory Storage persists reports under, relative to
/// the current working directory. Not configurable yet — this is the
/// smallest slice Phase 2 authorizes; a configurable location is a
/// separate, later capability, not decided here.
const STORAGE_ROOT: &str = ".modiq-storage";

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
/// already does. Four concrete commands exist today, dispatched by one
/// direct match — no command trait, registry, or lookup table, per this
/// platform's "capability before abstraction" discipline: two commands
/// is not evidence a dispatch abstraction is justified, and four still
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
                Some(path) => AssessCommand::run(path, STORAGE_ROOT),
                None => (
                    format!(
                        "{}\n\nerror: `assess` requires a path argument",
                        HelpCommand::run()
                    ),
                    ExitCode::InvalidUsage,
                ),
            },
            Some("retrieve") => match args.get(1) {
                Some(key) => RetrieveCommand::run(key, STORAGE_ROOT),
                None => (
                    format!(
                        "{}\n\nerror: `retrieve` requires a key argument",
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
    fn retrieve_without_a_key_is_invalid_usage() {
        let (message, exit_code) = Application::run(&["retrieve".to_string()]);

        assert_eq!(exit_code, ExitCode::InvalidUsage);
        assert!(message.contains("requires a key argument"));
    }

    #[test]
    fn retrieve_with_an_unrecognized_key_is_invalid_usage() {
        let (message, exit_code) = Application::run(&[
            "retrieve".to_string(),
            "no-such-key-anyone-would-plausibly-collide-with".to_string(),
        ]);

        assert_eq!(exit_code, ExitCode::InvalidUsage);
        assert!(message.contains("no report is stored"));
    }

    #[test]
    fn unrecognized_command_is_invalid_usage() {
        let (message, exit_code) = Application::run(&["bogus".to_string()]);

        assert_eq!(exit_code, ExitCode::InvalidUsage);
        assert!(message.contains("unrecognized command"));
    }
}
