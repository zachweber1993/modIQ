use modiq_cli::app::{Application, ExitCode};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (message, exit_code) = Application::run(&args);

    if exit_code == ExitCode::Success {
        println!("{message}");
    } else {
        eprintln!("{message}");
    }

    std::process::exit(exit_code.code());
}
