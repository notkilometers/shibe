#![crate_type = "lib"]
#![crate_name ="shibe"]

extern crate reqwest;

pub fn dump_urls(map: Result<String, reqwest::Error>) -> Option<Box<Vec<String>>> {
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

pub fn dump_url(map: Result<String, reqwest::Error>) -> Option<Box<String>> {
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

pub fn get_response(count: i32, urls: bool, https: bool) -> Result<String, reqwest::Error> {
    //let url = format!("http://shibe.online/api/shibes?count={}&urls={}&httpsUrls={}", count, urls, https);
    let resp = reqwest::blocking::get(format!("http://shibe.online/api/shibes?count={}&urls={}&httpsUrls={}", count, urls, https))?
        .text()?;
    Ok(resp)
}
