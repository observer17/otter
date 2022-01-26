use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[clap(name = "otter")]
#[clap(author = "wangxiao <wangxiao_hi@outlook.com>")]
#[clap(version = "0.0.1")]
#[clap(about = "Command Line Tools for me", long_about = None)]
struct Otter {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about="setup dev env")]
    Init,
}

fn setup_dev_tools() {
    println!("====== start setup dev tools ======");
    println!("fetch vim config");
    let mut output = Command::new("git")
        .args(["clone", "https://github.com/observer17/myvim.git", "~/.vim"])
        .output()
        .expect("failed to fetch vim config");
    println!("fetch vim config end");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    println!("start linking vimrc");
    output = Command::new("ln")
        .args(["-s", "~/.vim/.vimrc", ".vimrc"])
        .output()
        .expect("failed to link vimrc to your home");
    println!("link vimrc end");
}

fn main() {
    let cli = Otter::parse();
    match cli.command {
        Some(Commands::Init) => {
            setup_dev_tools();
        }
        None => {
            println!("no valid subcommand, use --help for information");
        }
    }
}
