pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> anyhow::Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let _ = find_matches(
        "Lorem ipsum dolor\nsit amet erat minim",
        "ipsum",
        &mut result,
    );
    assert_eq!(result, b"Lorem ipsum dolor\n");
}
