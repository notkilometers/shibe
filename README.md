# shibe
<h3>Dependencies: reqwest, tokio (for async)</h3>

## Usage:
* Call shibe::get_response(count, urls, https), where count is an i32 [1-100] of desired shibe pictures, urls is a bool for if you want a full url or just the filename, and https is a bool for if you want http or https urls
* Call dump_url(map) if you have a single picture, and dump_urls(map) if you have multiple, to return an Option<Box\<String>> or an Option<Box<Vec\<String>>>, respectively

* To add to a Discord Bot, put this in and send img as a message: 
```Rust
let img = async {
                // grab single image from shibe.online db
                let response = shibe::get_response(1, true, true).await;
                // get the url from it, unwrap it
                let image = shibe::dump_url(response).unwrap();
                // replace the leading and trailing square brackets
                image.replace("[", "").replace("]", "")
            }.await;
```
