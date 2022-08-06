use std::fs::read_to_string;
use steam_webapi_rust_sdk::store_steampowered_com::appdetails::get_resource_filepath;

fn main() {
    println!("Hello, world!");

    let dota_app_id = 570;

    let resource_file_path = get_resource_filepath(dota_app_id);
    println!("{}", resource_file_path);

    let boxed_read = read_to_string(resource_file_path);
    let is_readable = boxed_read.is_ok();
    if is_readable {
        let cached_api_response = boxed_read.unwrap();
        println!("{}", cached_api_response);
    } else {
        println!("unable to read cached resource");
    }
}
