use chrono::{DateTime, FixedOffset, NaiveDateTime};
use chrono::format::StrftimeItems;
use toys::networks::ip::{get_ip_address_info_async, get_ip_info_async, get_weather};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    // 获取公网ip
    let ip_info = match get_ip_info_async().await {
        Ok(info)=> info,
        Err(_)=> {
            println!("\
\u{001b}[31m
|￣￣￣￣￣￣￣￣￣￣￣|
|  IP获取错误了鸭    |
|＿＿＿＿＿＿＿＿＿＿＿|
(\\__/)||
(•ㅅ•) ||
/ 　 づ v");
            // 终止程序
            std::process::exit(1);
        }
    };
    // 开启异步请求,根据ip获取地区名
    let address_task = get_ip_address_info_async(&*ip_info.query);

    // 开启异步请求,根据ip的经纬度获取天气信息
    let weather_task =  get_weather(ip_info.lat,ip_info.lon);

    // 阻塞等待获取地区名,如果出现错误则显示`Unknown`
    let address_name = address_task.await
        .map(|address|address.get_name())
        .unwrap_or(String::from("Unknown"));
    // 阻塞等待获取天气信息,如果出现错误则直接退出
    let weather_info = weather_task.await.unwrap_or_else(|_|{
        println!("\
\u{001b}[31m
|￣￣￣￣￣￣￣￣￣￣￣|
|  天气获取失败了鸭   |
|＿＿＿＿＿＿＿＿＿＿＿|
(\\__/)||
(•ㅅ•) ||
/ 　 づ v
");
        std::process::exit(1)
    });

    // 获取天气
    let weather = weather_info.weather.get(0).map(|weather| weather.desc.as_str())
        .unwrap_or("Unknown");
    // 获取气温
    let temp = weather_info.main;
    // 获取数据计算时间
    // 设置国内时区 UTC+8
    let time_zone:FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();
    // 根据时间戳,转为DateTime,再格式化为字符串
    let time_str = DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp_opt(weather_info.dt,0).unwrap(),time_zone)
        .format_with_items(StrftimeItems::new("%Y-%m-%d %H:%M:%S")).to_string();
    let lines = format!("\
\n
|￣￣￣￣￣￣￣￣￣￣￣￣￣￣
| \u{001b}[32m 所属地区: {} \u{001b}[0m
| \u{001b}[33m 天气: {} \u{001b}[0m
| \u{001b}[31m 气温: {}°~{}° \u{001b}[0m
| \u{001b}[34m 湿度: {}% \u{001b}[0m
| \u{001b}[36m 风速: {} \u{001b}[0m
| \u{001b}[35m 数据生成时间: {} \u{001b}[0m
|＿＿＿＿＿＿＿＿＿＿＿＿＿＿
(\\__/) ||
(•ㅅ•) ||
/ 　 づv
",address_name.trim(),weather,temp.temp_min,temp.temp_max,temp.humidity,weather_info.wind.speed,time_str);
    println!("{}",lines);
    std::process::exit(0);
}


#[cfg(test)]
mod tests{


    #[tokio::test]
    async fn test_work() -> Result<(),std::env::VarError>{
        println!("{}", std::env::var("OPENAI_API_KEY")?);
        Ok(())
    }

}