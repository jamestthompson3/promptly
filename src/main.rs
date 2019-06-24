use colored::*;
use std::thread;

mod cwd;
mod vcs;

fn main() {
    println!("{}", construct_prompt())
}

fn construct_prompt() -> String {
    let cwdthread = thread::spawn(move || {
        let dir = cwd::cwd();
        return dir;
    });
    let vcsthread = thread::spawn(move || {
        let (branch, status) = vcs::vcs().unwrap_or(("".into(), "".into()));
        return (branch, status);
    });

    let workingdir = cwdthread.join().unwrap();
    let (branch, status) = vcsthread.join().unwrap();
    let separator = "┌".bright_black();
    let pointer = "└──>".bright_black();
    return format!(
        "{separator} {cwd} {branch} {status}\n{pointer}",
        separator = separator,
        cwd = workingdir,
        branch = branch,
        status = status,
        pointer = pointer
    );
}
