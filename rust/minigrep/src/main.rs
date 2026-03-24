use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // コマンドラインから引数を受け取るためにはstd::env::args関数が必要
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("引数解析時にエラーが発生しました: {err}");
        process::exit(1);
    });

    println!("クエリ: {}", config.query);
    println!("ファイル: {}", config.file);

    if let Err(e) = run(config) {
        println!("アプリケーションエラー: {e}");
        process::exit(1);
    }
}

fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;
    println!("ファイルの中身:\n{contents}");
    Ok(())
}

struct Config {
    query: String,
    file: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("引数が足りません")
        }

        let query = args[1].clone();
        let file = args[2].clone();
        
        Ok(Config { query, file })
    }
}