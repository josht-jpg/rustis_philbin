use rustis_philbin::{get_game_settings, run_game};

fn main() {
    if let Err(e) = run_game(get_game_settings()) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
