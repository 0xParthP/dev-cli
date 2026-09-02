use dev_cli::{cli::InstallCommand, commands};

#[test]
fn install_command_executes() {
    let cmd = InstallCommand {};

    let result = commands::install::execute(cmd);

    assert!(result.is_ok());
}
