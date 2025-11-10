use std::io;
use std::os::unix::process::CommandExt;
use std::process::Command;

use log::info;

use crate::io::CmdStdio;

pub fn exec_command(mut cmd: Vec<String>, stdio: CmdStdio) -> io::Error {
    info!("executing: {:?}", cmd);
    let args = cmd.split_off(1);
    Command::new(&cmd[0])
        .args(args)
        .env_remove("DYLD_INSERT_LIBRARIES")
        .env_remove("LD_PRELOAD")
        .env_remove("LD_AUDIT")
        .stdin(stdio.0)
        .stdout(stdio.1)
        .stderr(stdio.2)
        .exec()
}
