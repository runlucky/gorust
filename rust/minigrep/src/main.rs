use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // コマンドラインから引数を受け取るためにはstd::env::args関数が必要
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("引数解析時にエラーが発生しました: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("アプリケーションエラー: {e}");
        process::exit(1);
    }
}
