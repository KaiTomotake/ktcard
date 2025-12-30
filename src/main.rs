use clap::{Command, Arg, ArgAction};
use colored::Colorize;

fn main() {
    let matches = Command::new("Card")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg_required_else_help(true)
        .arg(
            Arg::new("profile")
                .long("profile")
                .short('p')
                .help("Shows the embedded digital profile information")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("appinfo")
                .short('i')
                .help("Displays application information")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    if matches.get_flag("profile") && !matches.get_flag("appinfo") {
        println!("{}", "===== プロフィール =====".bold().blue());
        println!("{} {}", "名前:".bold(), "甲斐 智丈".cyan());
        println!("{} {}", "ふりがな:".bold(), "かい ともたけ".cyan());
        println!("{} {}", "メール:".bold(), "kaitomotake@gmail.com".yellow());
        println!("{} {}", "Webサイト:".bold(), "https://modern-sys.dev/".green().underline());
        println!("{} {}", "Github:".bold(), "https://github.com/KaiTomotake".green().underline());
        println!("{}", "========================".bold().blue());
    } else if !matches.get_flag("profile") && matches.get_flag("appinfo") {
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue());
        println!("{}", "KTCARD".bold().bright_cyan());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue());
        println!("{} {}", "リポジトリ:".bold(), "https://github.com/KaiTomotake/ktcard".green().underline());
        println!("{} {}", "バージョン:".bold(), env!("CARGO_PKG_VERSION").yellow());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue());
    }
}
