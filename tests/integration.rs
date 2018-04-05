extern crate assert_cli;

use std::fs;
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
    fs::remove_file(&settings_fixtures_copy(test_id).as_path()).is_ok();
    fs::copy(
        &settings_fixtures_src().as_path(),
        &settings_fixtures_copy(test_id).as_path(),
    ).unwrap();
}

fn teardown(test_id: u32) {
    fs::remove_file(&settings_fixtures_copy(test_id).as_path()).unwrap();
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

        // host list (check for existing host in fixture)
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .contains("existing.example.com")
            .unwrap();

        // remove existing host
        assert_cli_bin(test_id)
            .with_args(&["host", "existing.example.com", "remove"])
            .succeeds()
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
        // host example.com add
        assert_cli_bin(test_id)
            .with_args(&["host", "example.com", "add"])
            .succeeds()
            .stdin("ssh-")
            .unwrap();

        // host examaple.com add
        assert_cli_bin(test_id)
            .with_args(&["host", "example.com", "add"])
            .fails()
            .unwrap();

        // host list
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .contains("example.com")
            .unwrap();
    })
}

#[test]
fn host_alias() {
    let test_id = line!();

    run_test(test_id, || {
        // add two hosts
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "add"])
            .succeeds()
            .unwrap();
        assert_cli_bin(test_id)
            .with_args(&["host", "2.example.com", "add"])
            .succeeds()
            .unwrap();

        // try to alias unknown host
        assert_cli_bin(test_id)
            .with_args(&["host", "foo.example.com", "alias", "1"])
            .fails()
            .unwrap();

        // try to alias with an alias where the hostname already exists
        assert_cli_bin(test_id)
            .with_args(&["host", "foo.example.com", "alias", "1.example.com"])
            .fails()
            .unwrap();

        // check no alias were set
        assert_cli_bin(test_id)
            .with_args(&["host", "list", "--raw"])
            .succeeds()
            .stdout()
            .contains("alias: None")
            .stdout()
            .doesnt_contain("alias: \"")
            .unwrap();

        // alias 1.example.com
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "alias", "1"])
            .succeeds()
            .unwrap();

        // check alias was set
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "list", "--raw"])
            .succeeds()
            .stdout()
            .contains("alias: Some(\"1\")")
            .stdout()
            .doesnt_contain("alias: None")
            .unwrap();

        // check lookup by alias works
        assert_cli_bin(test_id)
            .with_args(&["host", "1", "list"])
            .succeeds()
            .stdout()
            .contains("1.example.com")
            .stdout()
            .doesnt_contain("2.example.com")
            .unwrap();

        // overwrite alias
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "alias", "one"])
            .succeeds()
            .unwrap();

        // check alias was set
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "list", "--raw"])
            .succeeds()
            .stdout()
            .contains("alias: Some(\"one\")")
            .stdout()
            .doesnt_contain("alias: None")
            .unwrap();

        // remove alias
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "alias"])
            .succeeds()
            .unwrap();

        // check alias was removed
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "list", "--raw"])
            .succeeds()
            .stdout()
            .contains("alias: None")
            .stdout()
            .doesnt_contain("alias: Some(\"one\")")
            .unwrap();

        // try to remove alias again results in error msg
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "alias"])
            .fails()
            .unwrap();
    })
}

#[test]
fn user_add_remove() {
    let test_id = line!();

    run_test(test_id, || {
        // user foo1 add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "add"])
            .stdin("ssh-123")
            .succeeds()
            .unwrap();

        // user foo2 add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo2", "add"])
            .stdin("ssh-456")
            .succeeds()
            .unwrap();

        // user foo3 add (missing key)
        assert_cli_bin(test_id)
            .with_args(&["user", "foo3", "add"])
            .fails()
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["user", "list"])
            .succeeds()
            .stdout()
            .contains("foo1")
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["user", "list"])
            .succeeds()
            .stdout()
            .contains("foo1")
            .stdout()
            .contains("foo2")
            .unwrap();

        // user list --raw
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "list", "--raw"])
            .succeeds()
            .stdout()
            .contains("ssh-123")
            .stdout()
            .doesnt_contain("ssh-456")
            .unwrap();

        // user foo1 remove
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "remove"])
            .succeeds()
            .unwrap();

        // user foo2 remove
        assert_cli_bin(test_id)
            .with_args(&["user", "foo2", "remove"])
            .succeeds()
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["user", "list"])
            .succeeds()
            .stdout()
            .doesnt_contain("foo1")
            .stdout()
            .doesnt_contain("foo2")
            .unwrap();
    })
}

#[test]
fn user_add_duplicate_deny() {
    let test_id = line!();

    run_test(test_id, || {
        // user foo add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "add"])
            .stdin("ssh-")
            .succeeds()
            .unwrap();

        // user foo add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "add"])
            .stdin("ssh-")
            .fails()
            .unwrap();

        // user list
        assert_cli_bin(test_id)
            .with_args(&["user", "list"])
            .succeeds()
            .stdout()
            .contains("foo1")
            .unwrap();
    })
}

#[test]
fn group_add_remove() {
    let test_id = line!();

    run_test(test_id, || {
        // group dev-ops add
        assert_cli_bin(test_id)
            .with_args(&["group", "dev-ops", "add"])
            .succeeds()
            .unwrap();

        // group fsupport add
        assert_cli_bin(test_id)
            .with_args(&["group", "support", "add"])
            .succeeds()
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

        // group dev-ops .com remove
        assert_cli_bin(test_id)
            .with_args(&["group", "dev-ops", "remove"])
            .succeeds()
            .unwrap();

        // group support remove
        assert_cli_bin(test_id)
            .with_args(&["group", "support", "remove"])
            .succeeds()
            .unwrap();

        // group support remove
        assert_cli_bin(test_id)
            .with_args(&["group", "support", "remove"])
            .fails()
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
        // group dev-ops add
        assert_cli_bin(test_id)
            .with_args(&["group", "dev-ops", "add"])
            .succeeds()
            .unwrap();

        // group dev-ops add (duplicate)
        assert_cli_bin(test_id)
            .with_args(&["group", "dev-ops", "add"])
            .fails()
            .unwrap();

        // group list
        assert_cli_bin(test_id)
            .with_args(&["group", "list"])
            .succeeds()
            .stdout()
            .contains("dev-ops")
            .unwrap();
    })
}

#[test]
fn user_grant_revoke() {
    let test_id = line!();

    run_test(test_id, || {
        // user foo1 add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "add"])
            .stdin("ssh-")
            .succeeds()
            .unwrap();

        // user foo2 add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo2", "add"])
            .stdin("ssh-")
            .succeeds()
            .unwrap();

        // user foo3 add
        assert_cli_bin(test_id)
            .with_args(&["user", "foo3", "add"])
            .stdin("ssh-")
            .succeeds()
            .unwrap();

        // host 1.example.com add
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "add"])
            .succeeds()
            .unwrap();

        // host 2.example.com add
        assert_cli_bin(test_id)
            .with_args(&["host", "2.example.com", "add"])
            .succeeds()
            .unwrap();

        // user foo1 grant 1.example.com
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "grant", "1.example.com"])
            .succeeds()
            .unwrap();

        // user foo1 grant 1.example.com (fail, second add)
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "grant", "1.example.com"])
            .fails()
            .unwrap();

        // user foo1 grant 2.example.com
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "grant", "2.example.com"])
            .succeeds()
            .unwrap();

        // user foo2 grant 2.example.com
        assert_cli_bin(test_id)
            .with_args(&["user", "foo2", "grant", "1.example.com"])
            .succeeds()
            .unwrap();

        // host list
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .contains("foo1")
            .stdout()
            .contains("foo2")
            .stdout()
            .doesnt_contain("foo3")
            .unwrap();

        // host 1.example.com list
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "list"])
            .succeeds()
            .stdout()
            .contains("foo1")
            .stdout()
            .contains("foo2")
            .stdout()
            .doesnt_contain("foo3")
            .unwrap();

        // host 2.example.com list
        assert_cli_bin(test_id)
            .with_args(&["host", "2.example.com", "list"])
            .succeeds()
            .stdout()
            .contains("foo1")
            .stdout()
            .doesnt_contain("foo2")
            .stdout()
            .doesnt_contain("foo3")
            .unwrap();

        // user foo1 revoke 2.example.com
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "revoke", "2.example.com"])
            .succeeds()
            .unwrap();

        // host list
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .contains("foo1")
            .stdout()
            .contains("foo2")
            .unwrap();

        // host 2.example.com list
        assert_cli_bin(test_id)
            .with_args(&["host", "2.example.com", "list"])
            .succeeds()
            .stdout()
            .doesnt_contain("foo1")
            .stdout()
            .doesnt_contain("foo2")
            .unwrap();

        // host 1.example.com list
        assert_cli_bin(test_id)
            .with_args(&["host", "1.example.com", "list"])
            .succeeds()
            .stdout()
            .contains("foo1")
            .unwrap();

        // user foo1 revoke 1.example.com
        assert_cli_bin(test_id)
            .with_args(&["user", "foo1", "revoke", "1.example.com"])
            .succeeds()
            .unwrap();

        // host list
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .doesnt_contain("foo1")
            .stdout()
            .contains("foo2")
            .unwrap();

        // user foo2 revoke 1.example.com
        assert_cli_bin(test_id)
            .with_args(&["user", "foo2", "revoke", "1.example.com"])
            .succeeds()
            .unwrap();

        // host list
        assert_cli_bin(test_id)
            .with_args(&["host", "list"])
            .succeeds()
            .stdout()
            .doesnt_contain("foo2")
            .unwrap();
    })
}
