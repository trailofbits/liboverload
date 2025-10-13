use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, fs};

use ctor::ctor;

pub fn print_banner() {
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

pub fn cmd_from_env() -> Result<Option<Vec<String>>, shell_words::ParseError> {
    let cmd_str = env::var("OVERLOAD_CMD").ok();
    if let Some(cmd) = cmd_str {
        let cmd_split = shell_words::split(&cmd)?;
        Ok(Some(cmd_split))
    } else {
        Ok(None)
    }
}

pub fn cmd_from_file() -> Result<Option<Vec<String>>, shell_words::ParseError> {
    let path = env::var("OVERLOAD_CMD_FILE").unwrap_or("/tmp/overload.cmd".to_owned());
    let cmd_str = fs::read_to_string(path).ok();
    if let Some(cmd) = cmd_str {
        let cmd_split = shell_words::split(&cmd)?;
        Ok(Some(cmd_split))
    } else {
        Ok(None)
    }
}

#[cfg(not(test))]
#[ctor]
pub fn entry_point() {
    print_banner();
    let cmd_args = cmd_from_env()
        .expect("error parsing cmd from env")
        .or_else(|| cmd_from_file().expect("error parsing cmd from file"));
    if let Some(mut cmd) = cmd_args {
        let args = cmd.split_off(1);
        Command::new(&cmd[0])
            .args(args)
            .env_remove("LD_PRELOAD")
            .exec();
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn cmd_parsing_works() {
        temp_env::with_var("OVERLOAD_CMD", Some("echo hello world 'hi world'"), || {
            let cmd = cmd_from_env().expect("error parsing args");

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
            let cmd = cmd_from_file().expect("error parsing args");

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
