use std::{env, fs};

pub fn from_env() -> Result<Option<Vec<String>>, shell_words::ParseError> {
    let cmd_str = env::var("OVERLOAD_CMD").ok();
    if let Some(cmd) = cmd_str {
        let cmd_split = shell_words::split(&cmd)?;
        Ok(Some(cmd_split))
    } else {
        Ok(None)
    }
}

pub fn from_file() -> Result<Option<Vec<String>>, shell_words::ParseError> {
    let path = env::var("OVERLOAD_CMD_FILE").unwrap_or("/tmp/overload.cmd".to_owned());
    let cmd_str = fs::read_to_string(path).ok();
    if let Some(cmd) = cmd_str {
        let cmd_split = shell_words::split(&cmd)?;
        Ok(Some(cmd_split))
    } else {
        Ok(None)
    }
}
