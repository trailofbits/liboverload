use std::{env, fs::OpenOptions, process::Stdio};

use log::{error, info};

pub struct CmdStdio(pub Stdio, pub Stdio, pub Stdio);

fn get_stdio(name: &str, var: &str, write: bool) -> Stdio {
    let file_or_inherit = env::var(var).ok();
    if let Some(file) = file_or_inherit {
        info!("io: detected {}: {}", name, file);
        if file == "inherit" {
            Stdio::inherit()
        } else if let Ok(fd) = OpenOptions::new()
            .read(!write)
            .write(write)
            .create(write)
            .open(&file)
        {
            fd.into()
        } else {
            error!("io: failed to open {}, using inherit instead", file);
            Stdio::inherit()
        }
    } else {
        info!("io: no {} found, defaulting to inherit", name);
        Stdio::inherit()
    }
}

pub fn get_process_stdios() -> CmdStdio {
    CmdStdio(
        get_stdio("stdin", "OVERLOAD_STDIN", false),
        get_stdio("stdout", "OVERLOAD_STDOUT", true),
        get_stdio("stderr", "OVERLOAD_STDERR", true),
    )
}

pub fn get_inherit_stdios() -> CmdStdio {
    CmdStdio(Stdio::inherit(), Stdio::inherit(), Stdio::inherit())
}
