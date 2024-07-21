use std::{env, path::Path};
use crate::types::*;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

/**
 * 连接 Sqlite 数据库，返回连接池
 */
pub async fn connect_database() -> Result<SqlitePool, DynamicError>{

    // 从环境变量中获取 数据库 信息
    let db_file = env::var("POME_DB_PATH")?;
    let db_url = env::var("DATABASE_URL")?;
    if !Path::new(&db_file).exists() {
        // 创建数据库
        println!("数据库不存在");
        Sqlite::create_database(&db_file).await?;
    }
    // let conn = SqliteConnection::connect(&db_url).await?;
    // 连接数据库,获取连接池
    let pool = SqlitePool::connect(&db_url).await?;
    let _ = create_table(&pool).await?;
    Ok(pool)
}


/**
 * 创建 `pomes` 数据表
 */
async fn create_table(pool: &SqlitePool) -> DefaultResult{
      let created = sqlx::query(
        "CREATE TABLE IF NOT EXISTS pomes (
           id INTEGER PRIMARY KEY,
           title  TEXT NOT NULL,
           author  TEXT NOT NULL,
           content TEXT NOT NULL
        )"
    ).execute(pool).await;
    match created {
        Ok(_) => {
            println!("Create table `pomes` success.");
            Ok(())
        },
        Err(e) => Err(e.into()),
    }
}


/**
 * 获取项目根路径
 */
#[warn(dead_code)]
fn _get_root_path() -> String{
     env::current_dir().unwrap().to_string_lossy().into_owned()
}