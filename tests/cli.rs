use std::io::{self, Write};
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn find_content_in_file() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActualcontent\nMore content\nAnother test")?;
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test")
        .arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));
    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar")
        .arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}