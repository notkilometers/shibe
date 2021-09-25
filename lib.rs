#![crate_type = "lib"]
#![crate_name ="shibe"]

extern crate reqwest;

pub enum Animal {
    Shiba,
    Cat,
    Bird,
}

pub fn get_response(pic_type: Animal, count: i32, urls: bool, https: bool) -> Result<Vec<String>, reqwest::Error> {
    let animal = match pic_type {
        Animal::Shiba => { "shibes" },
        Animal::Cat => { "cats" },
        Animal::Bird => { "birds" },
    };
    
    let resp = reqwest::blocking::get(format!("http://shibe.online/api/{}?count={}&urls={}&httpsUrls={}", animal, count, urls, https))?
        .json::<Vec<String>>()?;
    
    Ok(resp)
}
