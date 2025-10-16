use std::{
    env,
    ffi::c_uint,
    process::{self},
};

#[cfg(all(not(test), feature = "ld_preload"))]
use ctor::ctor;
use env_logger::Builder;
use log::{info, LevelFilter};

mod cmd;
mod exec;
mod io;

fn print_banner() {
    info!(
        r"reporting from liboverload
 ________________
< DOING OVERLOAD >
 ----------------
   \
    \  .——————.
      .———.    \
     (     )   +——\
      `———´    |  |
      |        |  |
      |   __   +——/
      \__/  \__/    "
    );
}

fn entry_point(method: &str) {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .parse_env("OVERLOAD_LOG")
        .init();
    info!("initialized via {}", method);

    let original_cmd: Vec<String> = env::args().collect();
    info!(
        "this process is id {}, cmd: {:?}",
        process::id(),
        original_cmd
    );

    print_banner();

    let stdio = io::get_process_stdios();

    let cmd_args = cmd::from_env()
        .expect("error parsing cmd from env")
        .or_else(|| cmd::from_file().expect("error parsing cmd from file"));
    if let Some(cmd) = cmd_args {
        exec::exec_command(cmd, stdio);
    } else {
        info!("bailing out and re-executing without liboverload (might fail!)");
        exec::exec_command(original_cmd, io::get_inherit_stdios());
    }
}

#[cfg(all(not(test), feature = "ld_preload"))] // needed to not run the library code during testing
#[ctor]
pub fn preload() {
    // LD_PRELOAD entrypoint
    entry_point("LD_PRELOAD");
}

#[unsafe(no_mangle)]
#[cfg(feature = "ld_audit")]
pub extern "C" fn la_version(version: c_uint) -> c_uint {
    // LD_AUDIT entrypoint
    entry_point("LD_AUDIT");
    version
}
