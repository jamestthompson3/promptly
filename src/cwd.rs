use colored::*;
use std::env;

pub fn cwd() -> colored::ColoredString {
    let mut path = format!("{}", env::current_dir().unwrap_or("".into()).display());
    let home = env::var("HOME").unwrap();
    path = path.replace(&home[..], "~");
    let cwd_color = env::var("CWD_COLOR").unwrap_or("bright cyan".into());
    return path.color(cwd_color).bold();
}
