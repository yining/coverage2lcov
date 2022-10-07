use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn cli_not_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("coverage2lcov")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("missing"));

    Ok(())
}

#[test]
fn cli_file_not_exists() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("coverage2lcov")?;

    cmd.arg("tests/fixtures/not/exists");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error reading coverage file"));

    Ok(())
}

#[test]
fn cli_okay_lcov_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("coverage2lcov")?;

    cmd.arg("tests/fixtures/test.coverage");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("SF:autoload/lcov.vim"))
        .stdout(predicates::str::contains("end_of_record"));

    Ok(())
}
