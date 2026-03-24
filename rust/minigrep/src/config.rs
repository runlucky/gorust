use std::env;

pub struct Config {
    pub query: String,
    pub file: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("引数が足りません");
        }

        let query = args[1].clone();
        let file = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file,
            ignore_case,
        })
    }
}
