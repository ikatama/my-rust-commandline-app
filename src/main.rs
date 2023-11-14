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

    // 引数で受け取ったファイルを読み込む
    let content = std::fs::read_to_string(&args.path).expect("could not read file.");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
