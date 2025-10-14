use std::io;
use std::os::unix::process::CommandExt;
use std::process::Command;

use log::info;

pub fn exec_command(mut cmd: Vec<String>) -> io::Error {
    info!("executing: {:?}", cmd);
    let args = cmd.split_off(1);
    Command::new(&cmd[0])
        .args(args)
        .env_remove("LD_PRELOAD")
        .env_remove("LD_AUDIT")
        .exec()
}
