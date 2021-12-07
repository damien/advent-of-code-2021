use advent_of_code_2021::cli;

fn main() {
    // Initialize global logger
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    log::info!("Version: {}", advent_of_code_2021::VERSION);

    let app = cli::build();

    app.get_matches();
}
