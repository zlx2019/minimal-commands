use clap::{Args, Parser, Subcommand};

/**
 * 命令行参数体
 */
#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct CommandLine{
    /// Options 选项
    /// 诗词标题
    #[arg(short, long)]
    pub title: Option<String>,
    /// Command 诗词的增删改查子命令
    #[command(subcommand)]
    pub sub_command: Option<SubCommands>,
}


/**
 * 诗词管理 子命令
 *  add 增加诗词
 *  del 删除诗词
 *  
 */
#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// 新增诗词
    #[command(name = "add", about = "新增诗词")]
    Add(Poem),
    
    /// 删除诗词，可根据 [编号|标题] 进行删除
    #[command(name = "del", about = "根据 [编号|标题] 删除诗词")]
    Del(DelTerm),
}

/**
 * 删除诗词 命令参数
 * 分组参数，只能选择输入 [id | title] 一项参数，不可同时选择或同时都不选择.
 */
#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct DelTerm{
    /// 诗词编号
    #[arg(long)]
    pub id: Option<usize>,
    /// 诗词标题
    #[arg(long)]
    pub title: Option<String>
}

/**
 * 添加诗词 命令参数
 */
#[derive(Debug, Args)]
pub struct Poem{
    /// 诗词标题
    pub title: String,
    /// 作者
    pub author: String,
    /// 正文
    pub content: String
}

