use clap::Parser;

/// 指定したファイルからパターンに合致した行を探す。
/// 合致した行をコンソールに表示する。
#[derive(Parser)]
struct Cli {
    /// 探すパターン
    #[arg(short = 'p', long = "pattern", default_value_t = String::from("test"))]
    pattern: String,
    /// パターンを探す対象のファイルパス
    #[arg(long = "path")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
