#[cfg(not(test))]
use ctor::ctor;

#[cfg(not(test))]
mod exec;

mod cmd;

#[cfg(not(test))]
fn print_banner() {
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn cmd_parsing_works() {
        temp_env::with_var("OVERLOAD_CMD", Some("echo hello world 'hi world'"), || {
            let cmd = cmd::from_env().expect("error parsing args");

            assert_eq!(
                Some(
                    vec!["echo", "hello", "world", "hi world"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                ),
                cmd
            );
        })
    }

    #[test]
    fn cmd_file_parsing_works() {
        let asset_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/assets/cmd-file");
        temp_env::with_var("OVERLOAD_CMD_FILE", Some(asset_path), || {
            let cmd = cmd::from_file().expect("error parsing args");

            assert_eq!(
                Some(
                    vec!["echo", "hello", "world", "hi world"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                ),
                cmd
            );
        })
    }
}
