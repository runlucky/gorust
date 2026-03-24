use std::env;
use std::fs;

fn main() {
    // コマンドラインから引数を受け取るためにはstd::env::args関数が必要
    let args: Vec<String> = env::args().collect();
    
    // let query = &args[1];
    let config = Config::new(&args);

    println!("クエリ: {}", config.query);
    println!("ファイル: {}", config.file);



    let contents = fs::read_to_string(&config.file)
        .expect("ファイルを読み込むことができるはずでしたが、失敗しました。");

    println!("ファイルの中身:\n{contents}");


}

struct Config {
    query: String,
    file: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("引数が足りません")
        }

        let query = args[1].clone();
        let file = args[2].clone();
        
        Config { query, file }
    }
}