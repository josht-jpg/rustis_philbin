use rustis_philbin::{get_args, start_game};

fn main() {
    if let Err(e) = start_game(get_args()) {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
