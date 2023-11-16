use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    // 第2引数に存在しないファイルを指定
    cmd.arg("foobar").arg("test/file/doesnt/exist");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error reading"));

    Ok(())
}
