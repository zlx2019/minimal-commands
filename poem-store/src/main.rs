
use clap::Parser;
use dotenvy::dotenv;
use types::DefaultResult;
use crate::{commands::CommandLine, handler::handle, db::connect_database};

mod commands;
mod handler;
mod db;
mod types;

///
/// Program Main
/// 
#[tokio::main]
async fn main() -> DefaultResult{
    // 解析命令行参数
    let command_line = CommandLine::parse();
    dotenv()?;
    // 连接数据库
    let pool =  connect_database().await?;
    // 程序处理
    handle(command_line, &pool);
    Ok(())
}
