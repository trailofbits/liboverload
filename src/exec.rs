use std::io;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub fn exec_command(mut cmd: Vec<String>) -> io::Error {
    let args = cmd.split_off(1);
    Command::new(&cmd[0])
        .args(args)
        .env_remove("LD_PRELOAD")
        .exec()
}
