#![crate_type = "lib"]
#![crate_name ="shibe"]

extern crate reqwest;

pub fn get_response(count: i32, urls: bool, https: bool) -> Result<Vec<String>, reqwest::Error> {
    let resp = reqwest::blocking::get(format!("http://shibe.online/api/shibes?count={}&urls={}&httpsUrls={}", count, urls, https))?
        .json::<Vec<String>>()?;
    Ok(resp)
}
