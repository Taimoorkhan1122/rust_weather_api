use async_std::task;
use surf;

async fn fetch(url: &str) -> Result<String, surf::Exception> {
    surf::get(url).recv_string().await
}

async fn execute() {

    let city_name = "Karachi";
    let api_key = "3770c89821b6000981072f41e3ec419f"
    let mut api = format!("api.openweathermap.org/data/2.5/weather?q={}&appid={",
        city_name ,api_key})

    println!("{}"j, api);

    // match fetch("").await {
    //     Ok(s) =>
    //     Err(e) =>
    // }
}

fn main() {
    task::block_on(execute());
}