use std::{env, path::Path};
use crate::{commands::Poem, types::*};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

/**
 * 连接 Sqlite 数据库，获取连接池
 */
pub async fn connect_database() -> Result<SqlitePool, DynamicError>{
    // 从环境变量中获取 数据库 信息
    let db_file = env::var("POME_DB_PATH")?;
    let db_url = env::var("DATABASE_URL")?;
    if !Path::new(&db_file).exists() {
        // 创建数据库
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
    let check = sqlx::query("select name from sqlite_master where type = 'table' and name = 'poems';")
        .fetch_optional(pool)
        .await?;
    if check.is_some() {
        // 表已存在
        return Ok(());
    }
      let created = sqlx::query(
        "CREATE TABLE IF NOT EXISTS poems (
           id INTEGER PRIMARY KEY,
           title  TEXT NOT NULL,
           author  TEXT NOT NULL,
           content TEXT NOT NULL
        )"
    ).execute(pool).await;
    match created {
        Ok(_) => {
            Ok(())
        },
        Err(e) => Err(e.into()),
    }
}

// 查看标题是否已存在
pub async fn title_exists(pool: &SqlitePool, title: &str) -> bool {
    let res = sqlx::query!(
        "
        select count(1) as count
        from poems where title = ?
        ",
        title
    ).fetch_one(pool).await;
    match res {
        Ok(row)=> row.count > 0,
        Err(_) => false
    }
}


// 新增一首诗词
pub async fn insert_poem(pool: &SqlitePool, poem: &Poem) -> Result<i64, DynamicError>{
    let row_id = sqlx::query("insert into poems (title, author, content) values ($1, $2, $3)")
        .bind(&poem.title)
        .bind(&poem.author)
        .bind(&poem.content)
        .execute(pool)
        .await?.last_insert_rowid();
    Ok(row_id)
}


// 根据标题删除诗词
pub async fn delete_by_title(pool: &SqlitePool, title: &str)-> DefaultResult{
    sqlx::query!(
        r#"
            delete from poems
            where title = ?1
        "#,
        title
    ).execute(pool)
    .await?;
    Ok(())
}

// 根据id删除诗词
pub async fn delete_by_id(pool: &SqlitePool, id: u32) -> DefaultResult{
    sqlx::query!(
        r#"
            delete from poems
            where id = ?1
        "#,
        id
    ).execute(pool).await?;
    Ok(())
}

/**
 * 获取项目根路径
 */
#[warn(dead_code)]
fn _get_root_path() -> String{
     env::current_dir().unwrap().to_string_lossy().into_owned()
}