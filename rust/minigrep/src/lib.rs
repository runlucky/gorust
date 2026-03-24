use std::error::Error;
use std::fs;

mod config;
pub use config::Config;

mod search;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;

    let results = if config.ignore_case {
        search::search_case_insensitive(&config.query, &contents)
    } else {
        search::search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
