use crate::commands::{CommandLine, DelTerm, Poem, SubCommands};


/**
 * 主逻辑处理
 */
pub fn handle(command_line: CommandLine) {
    if let Some(title) = command_line.title {
        println!("{title}")
    }
    // 子命令处理
    if let Some(sub_command) = command_line.sub_command {
        match sub_command {
            // 新增诗词
            SubCommands::Add(poem) => handle_add_pome(&poem),
            // 删除诗词
            SubCommands::Del(term) => handler_del_pome(&term),
        }
    }
}


/**
 * 添加诗词
 */
pub fn handle_add_pome(pome: &Poem){
    println!("添加诗词，标题: {}, 作者: {}, 正文: {}", pome.title, pome.author, pome.content);
}

/**
 * 删除诗词
 */
pub fn handler_del_pome(term: &DelTerm){
    match (&term.title, term.id) {
        (Some(title), _) => {
            println!("根据标题[{title}]删除诗词");
        }
        (_, Some(id)) => {
            println!("根据Id[{id}]删除诗词");
        }
        _=>{}
    }
}