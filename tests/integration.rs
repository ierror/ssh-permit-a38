#[macro_use]
extern crate assert_cli;

use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::panic;
use std::path::{Path, PathBuf};

fn tests_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")
}

fn tests_tmp_dir() -> PathBuf {
    tests_dir().join("tmp")
}

fn settings_fixtures_src() -> PathBuf {
    tests_dir().join("fixtures").join("ssh-permit.json")
}

fn settings_fixtures_copy(test_id: u32) -> PathBuf {
    tests_tmp_dir().join(format!("ssh-permit-{}.json", test_id))
}

fn setup(test_id: u32) {
    fs::remove_file(&settings_fixtures_copy(test_id).as_path());
    fs::copy(
        &settings_fixtures_src().as_path(),
        &settings_fixtures_copy(test_id).as_path(),
    );
}

fn teardown(test_id: u32) {
    fs::remove_file(&settings_fixtures_copy(test_id).as_path());
}

fn assert_cli_bin(test_id: u32) -> assert_cli::Assert {
    assert_cli::Assert::main_binary().with_args(&[
        "--database",
        &settings_fixtures_copy(test_id).to_str().unwrap(),
    ])
}

fn run_test<T>(test_id: u32, test: T) -> ()
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    setup(test_id);
    let result = panic::catch_unwind(|| test());
    teardown(test_id);
    assert!(result.is_ok())
}

#[test]
fn host_add_remove() {
    let test_id = line!();

    run_test(test_id, || {
        // host foo1@example.com add
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // host foo1@exmap2e.com add
        assert_cli_bin(test_id)
            .with_args(&["host", "2.example.com", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // host list
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .contains("1.example.com")
            .unwrap();

        // host list
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .contains("2.example.com")
            .unwrap();

        // host foo1@example.com remove
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "remove"])
            .succeeds()
            .unwrap();

        // host foo2@example.com remove
        assert_cli_bin(test_id)
            .with_args(&["host", "2.example.com", "remove"])
            .succeeds()
            .unwrap();

        // host list
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .doesnt_contain("1.example.com")
            .stdout()
            .doesnt_contain("2.example.com")
            .unwrap();
    })
}

#[test]
fn host_add_duplicate_deny() {
    let test_id = line!();

    run_test(test_id, || {
        // user foo1@example.com add
        assert_cli_bin(test_id)
            .with_args(&["host", "example.com", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // user foo1@exmap2e.com add
        assert_cli_bin(test_id)
            .with_args(&["host", "example.com", "add"])
            .fails()
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .contains("example.com")
            .unwrap();
    })
}

#[test]
fn user_add_remove() {
    let test_id = line!();

    run_test(test_id, || {
        // user foo1@example.com add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1@example.com", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // user foo1@exmap2e.com add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo2@example.com", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["user", "list"])
            .succeeds()
            .stdout()
            .contains("foo1@example.com")
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["user", "list"])
            .succeeds()
            .stdout()
            .contains("foo2@example.com")
            .unwrap();

        // user foo1@example.com remove
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1@example.com", "remove"])
            .succeeds()
            .unwrap();

        // user foo2@example.com remove
        assert_cli_bin(test_id)
            .with_args(&["user", "foo2@example.com", "remove"])
            .succeeds()
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["user", "list"])
            .succeeds()
            .stdout()
            .doesnt_contain("foo1@example.com")
            .stdout()
            .doesnt_contain("foo2@example.com")
            .unwrap();
    })
}

#[test]
fn user_add_duplicate_deny() {
    let test_id = line!();

    run_test(test_id, || {
        // user foo1@example.com add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo@example.com", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // user foo1@exmap2e.com add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo@example.com", "add"])
            .fails()
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["user", "list"])
            .succeeds()
            .stdout()
            .contains("foo@example.com")
            .unwrap();
    })
}

#[test]
fn group_add_remove() {
    let test_id = line!();

    run_test(test_id, || {
        // group foo1@example.com add
        assert_cli_bin(test_id)
            .with_args(&["group", "dev-ops", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // group foo1@exmap2e.com add
        assert_cli_bin(test_id)
            .with_args(&["group", "support", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // group list
        assert_cli_bin(test_id)
            .with_args(&["group", "list"])
            .succeeds()
            .stdout()
            .contains("dev-ops")
            .unwrap();

        // group list
        assert_cli_bin(test_id)
            .with_args(&["group", "list"])
            .succeeds()
            .stdout()
            .contains("support")
            .unwrap();

        // group foo1@example.com remove
        assert_cli_bin(test_id)
            .with_args(&["group", "dev-ops", "remove"])
            .succeeds()
            .unwrap();

        // group foo2@example.com remove
        assert_cli_bin(test_id)
            .with_args(&["group", "support", "remove"])
            .succeeds()
            .unwrap();

        // group list
        assert_cli_bin(test_id)
            .with_args(&["group", "list"])
            .succeeds()
            .stdout()
            .doesnt_contain("dev-ops")
            .stdout()
            .doesnt_contain("support")
            .unwrap();
    })
}

#[test]
fn group_add_duplicate_deny() {
    let test_id = line!();

    run_test(test_id, || {
        // user foo1@example.com add
        assert_cli_bin(test_id)
            .with_args(&["group", "dev-ops", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // user foo1@exmap2e.com add
        assert_cli_bin(test_id)
            .with_args(&["group", "dev-ops", "add"])
            .fails()
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["group", "list"])
            .succeeds()
            .stdout()
            .contains("dev-ops")
            .unwrap();
    })
}
