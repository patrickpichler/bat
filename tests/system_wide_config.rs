use assert_cmd::cargo::CommandCargoExt;
use predicates::{prelude::predicate, str::PredicateStrExt};
use std::process::Command;

fn bat_raw_command() -> Command {
    let mut cmd = Command::cargo_bin("bat").unwrap();
    cmd.current_dir("tests/examples");
    cmd.env_remove("BAT_CACHE_PATH");
    cmd.env_remove("BAT_CONFIG_DIR");
    cmd.env_remove("BAT_CONFIG_PATH");
    cmd.env_remove("BAT_OPTS");
    cmd.env_remove("BAT_PAGER");
    cmd.env_remove("BAT_STYLE");
    cmd.env_remove("BAT_TABS");
    cmd.env_remove("BAT_THEME");
    cmd.env_remove("COLORTERM");
    cmd.env_remove("NO_COLOR");
    cmd.env_remove("PAGER");
    cmd
}

fn bat() -> assert_cmd::Command {
    assert_cmd::Command::from_std(bat_raw_command())
}

// This test is ignored, as it needs a special system wide config put into place
#[test]
#[ignore]
fn use_systemwide_config() {
    bat()
        .arg("test.txt")
        .assert()
        .success()
        .stdout(predicate::eq("dummy-pager-from-system-config\n").normalize());
}

// This test is ignored, as it needs a special system wide config put into place
#[test]
#[ignore]
fn config_overrides_system_config() {
    bat()
        .env("BAT_CONFIG_PATH", "bat.conf")
        .arg("test.txt")
        .assert()
        .success()
        .stdout(predicate::eq("dummy-pager-from-config\n").normalize());
}
