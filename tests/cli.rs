use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cat-clone")?;

    cmd.arg("/file/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not open the file"));

    Ok(())
}

#[test]
fn print_file_content() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("cat-clone")?;
    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("More content"));

    Ok(())
}