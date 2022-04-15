fn main() {
    if let Err(e) = rustis_philbin::get_args().and_then(rustis_philbin::start) {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
