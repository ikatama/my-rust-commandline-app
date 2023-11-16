use anyhow::{Context, Ok, Result};
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

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Error reading `{}`", args.path.display()))?;

    let _ = grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
