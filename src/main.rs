use serde_json::Value;
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
        parse_json(cached_api_response, dota_app_id);
    } else {
        println!("unable to read cached resource");
    }


}

fn parse_json(cached_api_response: String, app_id: i64) {
    let boxed_initial_parse = serde_json::from_str(&cached_api_response);
    if boxed_initial_parse.is_err() {
        return println!("{}", boxed_initial_parse.err().unwrap().to_string());
    }
    let mut json: Value = boxed_initial_parse.unwrap();

    let mut app_details_wrapped = json[app_id.to_string()].take();

    let mut is_success = app_details_wrapped["success".to_string()].take();
    if is_success.take().as_bool().unwrap() == false {
        println!("{}", "steampowered api returned failed response".to_string());
    }

    let mut app_details : Value = app_details_wrapped["data"].take();
    let mut indendation = 0;
    parse_json_object(&app_details, indendation)
}

fn parse_json_object(json: &Value, mut indentation: i64) {
    for (key, value) in json.as_object().unwrap() {
        let literal = " ".repeat(indentation as usize);
        println!("{}{}", literal, key);

        if value.is_object() {
            indentation = indentation + 1;
            parse_json_object(value, indentation);
            indentation = indentation - 1;
        }

    }
}
