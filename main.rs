fn main() {
    let single_response = shibe::get_response(1, true, true);
    let url = shibe::dump_url(single_response);
    println!("{:?}", url);
    
    let multiple_responses = shibe::get_response(2, true, true);
    let urls= shibe::dump_urls(multiple_responses);
    println!("{:?}", urls);
}
