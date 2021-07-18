use std::io;
use async_std::task;
use serde_json::Value;
use surf;


async fn fetch(url: &str) -> Result<String, surf::Exception> {
    surf::get(url).recv_string().await
}

async fn execute() {

    //taking user parameters
    let mut city_name = String::new();
    println!("Enter city name");
    io::stdin().read_line(&mut city_name)
    .expect("Sorry can't read that");

    let city_name = city_name.trim();

    let api_key = "3770c89821b6000981072f41e3ec419f";
    let api = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city_name ,api_key);

    // println!("{}", api);
    // let res = fetch(&api).await;
    // let json = serde_json::Value::String(serde_json::to_string(res.as_str()).unwrap());

    match fetch(&api).await {
        Ok(s) => {
            // serde_json::Value::String(serde_json::to_string(s.as_str()).unwrap())
            // let rjson: Value = serde_json::Value::String(serde_json::to_string(&s).unwrap());
            
            // s is a string we need to convert that into a JSON object
            let json_str: Value = serde_json::from_str(s.as_str()).unwrap();
            if json_str["cod"] == 200 {
                println!("Weather now in {}\nDetails: {}",json_str["name"],json_str["weather"][0] );
            } else {
                println!("{}", json_str["message"]);
            }
        },
        Err(e) => println!("Error in fetching data: {}", e),
    };

}

fn main() {
    task::block_on(execute());
}