use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let query = &args[1];
    // let file_path = &args[2];
    // let (query, file_path) = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Ошибка обработки аргументов: {err}");
        process::exit(1);
    });

    println!("Поиск: {}", config.query);
    println!("Файл: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Ошибка программы: {e}");
        process::exit(1);
    }
}
