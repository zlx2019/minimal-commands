use clap::Parser;
use crate::{commands::CommandLine, handler::handle};
mod commands;
mod handler;

///
/// Program Main
/// 
fn main() {
    // 解析命令行参数
    let command_line = CommandLine::parse();
    println!("{:?}", command_line);
    handle(command_line);
}
