# shibe
<h3>Dependencies: reqwest</h3>

## Usage:
* Call shibe::get_response(count, urls, https), where count is an i32 [1-100] of desired shibe pictures, urls is a bool for if you want a full url or just the filename, and https is a bool for if you want http or https urls
* Call dump_url(map) if you have a single picture, and dump_urls(map) if you have multiple, to return an Option<Box\<String>> or an Option<Box<Vec\<String>>>, respectively

