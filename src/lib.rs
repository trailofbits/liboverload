#[cfg(not(test))]
use ctor::ctor;
use std::ffi::c_uint;

mod cmd;
mod exec;

fn print_banner() {
    use std::env;

    if env::var("OVERLOAD_SILENT").ok() == Some("1".to_owned()) {
        return;
    }

    println!(
        r"
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
    )
}

fn entry_point() {
    print_banner();
    let cmd_args = cmd::from_env()
        .expect("error parsing cmd from env")
        .or_else(|| cmd::from_file().expect("error parsing cmd from file"));
    if let Some(cmd) = cmd_args {
        exec::exec_command(cmd);
    }
}

#[cfg(not(test))] // needed to not run the library code during testing
#[ctor]
pub fn preload() {
    // LD_PRELOAD entrypoint
    entry_point();
}

#[unsafe(no_mangle)]
pub extern "C" fn la_version(version: c_uint) -> c_uint {
    // LD_AUDIT entrypoint
    entry_point();
    version
}
