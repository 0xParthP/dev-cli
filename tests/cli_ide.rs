use assert_cmd::Command;

#[test]
fn ide_list_runs() {
    let mut cmd = Command::cargo_bin("dev").unwrap();

    cmd.arg("ide").arg("list").assert().success();
}
