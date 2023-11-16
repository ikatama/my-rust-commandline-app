use anyhow::{Context, Ok, Result};
use clap::Parser;

/// 指定したファイルからパターンに合致した行を探す。
/// 合致した行をコンソールに表示する。
#[derive(Parser)]
struct Cli {
    /// 探すパターン
    pattern: String,
    /// パターンを探す対象のファイルパス
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Error reading `{}`", args.path.display()))?;

    let _ = grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
