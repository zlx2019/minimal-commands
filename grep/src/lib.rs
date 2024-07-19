use std::{error::Error, fs};


// 主逻辑处理：读取文件内容，按行匹配
pub fn handler(command: Command) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(command.file_path)?;
    search(&command.key_word, &contents)
        .iter().for_each(|line| println!("{line}"));
    Ok(())
}

/// 命令行参数
pub struct Command{
    // 要检索的关键字
    pub key_word: String,
    // 要检索的文件
    pub file_path: String,
}

impl Command {
    /// 根据终端输入参数，构建 `Command`对象
    pub fn of(args: &[String]) -> Result<Command, &'static str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }
        let key_word = args[1].clone();
        let file_path = args[2].clone();
        Ok(Command{ key_word, file_path })
    }
}

/// 按行匹配内容，检索出包含关键字的行内容
pub fn search<'a>(key_word: &str, contents: &'a str) -> Vec<String>{
     contents.lines()
        .filter(|line| line.contains(&key_word))
        .map(|line|{
            line.replace(&key_word, &format!("\x1B[31m{}\x1B[0m", &key_word))
        })
        .collect::<Vec<String>>()
}


/// 单元测试
/// 测试驱动开发模式(TDD)
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let key_word = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(key_word, contents));
    }
}