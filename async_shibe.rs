#![crate_type = "lib"]
#![crate_name ="shibe"]

extern crate reqwest;
extern crate tokio;

pub async fn get_response(count: i32, urls: bool, https: bool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!("http://shibe.online/api/shibes?count={}&urls={}&httpsUrls={}", count, urls, https))
        .await?
        .json::<Vec<String>>()
        .await?;
    Ok(resp)
}
