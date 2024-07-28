use sqlx::SqlitePool;
use crate::{commands::{CommandLine, DelTerm, Poem, SubCommands}, db::{delete_by_id, delete_by_title, insert_poem, title_exists}};

/**
 * 程序的业务处理
 */
pub async fn handle(command_line: CommandLine, pool: SqlitePool) {
    if let Some(title) = command_line.title {
        println!("{title}")
    }
    // 子命令处理
    if let Some(sub_command) = command_line.sub_command {
        match sub_command {
            // 新增诗词
            SubCommands::Add(poem) => {
                handle_add_pome(&pool, &poem).await;
            },
            // 删除诗词
            SubCommands::Del(term) => {
                handler_remove_pome(&pool, &term).await;
            },
        }
    }
}



/**
 * 添加诗词
 */
pub async fn handle_add_pome(pool: &SqlitePool, poem: &Poem){
    // 检验标题是否已存在
    if title_exists(pool, &poem.title).await {
        eprintln!("\n 标题 [{}] 已存在!", poem.title);
        return ;
    }
    match insert_poem(pool, poem).await {
        Ok(row) => {
            println!("\n添加诗词成功. id: {}
            {} {}
            {}",row , poem.title, poem.author, poem.content);
        },
        Err(e) => {
            eprintln!("新增诗词失败: {e}");
        }
    }
}

/**
 * 删除诗词
 */
pub async fn handler_remove_pome(pool: &SqlitePool , term: &DelTerm){
    match (&term.title, term.id) {
        (Some(title), _) => {
            if let Ok(_) = delete_by_title(pool, title).await {
                println!("根据标题[{title}]删除诗词成功.");
            }
        }
        (_, Some(id)) => {
            if let Ok(_) = delete_by_id(pool, id as u32).await {   
                println!("根据Id[{id}]删除诗词成功");
            }
        }
        _=>{}
    }
}