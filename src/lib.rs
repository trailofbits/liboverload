#[cfg(not(test))]
use ctor::ctor;

#[cfg(not(test))]
mod exec;

mod cmd;

#[cfg(not(test))]
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

#[cfg(not(test))]
#[ctor]
pub fn entry_point() {
    print_banner();
    let cmd_args = cmd::from_env()
        .expect("error parsing cmd from env")
        .or_else(|| cmd::from_file().expect("error parsing cmd from file"));
    if let Some(mut cmd) = cmd_args {
        exec::exec_command(cmd);
    }
}
