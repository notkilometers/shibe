# shibe
<h3>Dependencies: reqwest, tokio (for async)</h3>

## Usage:
* Call shibe::get_response(count, urls, https), where count is an i32 [1-100] of desired shibe pictures, urls is a bool for if you want a full url or just the filename, and https is a bool for if you want http or https urls

* To add to a Discord Bot, put this in and send img_value as a message in your Handler: 
```Rust
let img_vector = shibe::get_response(shibe::Animal::Shiba,1, true, true)
  .await
  .unwrap();

let img_value = img_vector.last()
  .unwrap();
```
