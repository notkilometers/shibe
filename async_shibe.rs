#![crate_type = "lib"]
#![crate_name ="shibe"]

extern crate reqwest;
extern crate tokio;

pub fn dump_url(map: Result<String, Box<dyn std::error::Error>>) -> Option<Box<String>> {   
    match map {
        Ok(v) => {
            Some(Box::new(v.replace("\"", "")))
        },
        Err(e) => {
            println!("{}", e);
            None
        },
    }
}

pub fn dump_urls(map: Result<String, Box<dyn std::error::Error>>) -> Option<Box<Vec<String>>> {
    match map {
        Ok(v) => {
            let split = v.split(",");
            let mut vec = Vec::<String>::new();
            for c in split {
                vec.push(c.replace("\"", ""));
            }
            Some(Box::new(vec))
        },
        Err(e) => {
            println!("{}", e);
            None
        },
    }
}

pub async fn get_response(count: i32, urls: bool, https: bool) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!("http://shibe.online/api/shibes?count={}&urls={}&httpsUrls={}", count, urls, https))
        .await?
        .text()
        .await?;
    Ok(resp)
}
