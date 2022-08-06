use std::collections::HashSet;
use serde_json::Value;
use std::fs::read_to_string;
use std::path::Path;
use steam_webapi_rust_sdk::store_steampowered_com::appdetails::get_resource_filepath;
use steam_webapi_rust_sdk::util::get_cache_dir_path;

fn main() {
    println!("Hello, world!");

    let mut app_id_list: Vec<i64> = vec![730 as i64, 570 ];
    let mut app_details_structure : HashSet<String> = HashSet::new();


    let already_processed_app_id_list_path = [get_cache_dir_path(), "/".to_string(), "processed_app_id_list.json".to_string()].join("");
    let file_exists = Path::new(already_processed_app_id_list_path.as_str()).is_file();
    if file_exists {
        let serialized_string = read_to_string(&already_processed_app_id_list_path).unwrap();
        if serialized_string.len() > 0 {
            app_id_list = serde_json::from_str(serialized_string.as_str()).unwrap();
        }
    }


    for app_id in app_id_list.iter() {
        let boxed_processing_result = process_app_details(app_id.to_owned());
        if boxed_processing_result.is_err() {
            //println!("{}", boxed_processing_result.err().unwrap());
        } else {
            let resulting_set = boxed_processing_result.unwrap();
            app_details_structure.extend(resulting_set);
        }
    }

    println!("\n!!!\n!!!\n");

    let mut as_vector : Vec<&String> = app_details_structure.iter().collect();
    as_vector.sort_by(|a, b| b.cmp(a));

    for path in &as_vector {
        println!("{}", path)
    }
    println!("total properties count: {}", as_vector.len());
}

fn process_app_details(app_id: i64) -> Result<HashSet<String>, String> {
    let resource_file_path = get_resource_filepath(app_id);
    let boxed_read = read_to_string(resource_file_path);
    let is_readable = boxed_read.is_ok();
    if is_readable {
        let cached_api_response = boxed_read.unwrap();
        //println!("{}", cached_api_response);
        return parse_json(cached_api_response, app_id);
    } else {
        let error = "unable to read cached resource";
        //println!("{}", &error);
        return Err(error.to_string())
    }
}

fn parse_json(cached_api_response: String, app_id: i64) -> Result<HashSet<String>, String> {
    let mut app_details_structure : HashSet<String> = HashSet::new();

    let boxed_initial_parse = serde_json::from_str(&cached_api_response);
    if boxed_initial_parse.is_err() {
        let error = boxed_initial_parse.err().unwrap().to_string();
        println!("{}", &error);
        return Err(error);
    }
    let mut json: Value = boxed_initial_parse.unwrap();

    let mut app_details_wrapped = json[app_id.to_string()].take();

    let mut is_success = app_details_wrapped["success".to_string()].take();
    if is_success.take().as_bool().unwrap() == false {
        println!("{}", "steampowered api returned failed response".to_string());
    }

    let mut app_details : Value = app_details_wrapped["data"].take();
    let mut indendation = 0;
    let mut path = "";
    parse_json_object(&app_details, indendation, path, &mut app_details_structure);
    //println!("\n\n\n");

    let mut as_vector : Vec<&String> = app_details_structure.iter().collect();
    as_vector.sort_by(|a, b| b.cmp(a));

    for path in &as_vector {
        //println!("{}", path)
    }
    //println!("total properties count: {}", as_vector.len());

    Ok(app_details_structure)

}

fn parse_json_object(json: &Value, mut indentation: i64, mut path: &str, app_details_structure: &mut HashSet<String>) {
    for (key, value) in json.as_object().unwrap() {
        let literal = " ".repeat(indentation as usize);

        let mut property_type = "";
        let is_object = value.is_object();
        let is_array = value.is_array();
        let is_i64 = value.is_i64();
        let is_f64 = value.is_f64();
        let is_bool = value.is_boolean();
        let is_string = value.is_string();

        if is_string {
            property_type = "String";
        } else if is_object {
            property_type = "Object";
        } else if is_array {
            property_type = "[0]";
        } else if is_i64 {
            property_type = "i64";
        } else if is_f64 {
            property_type = "f64";
        } else if is_bool {
            property_type = "bool"
        }
        let new_path_and_type = format!("{}<{}>", key, property_type);
        let mut total_path = [&path, new_path_and_type.as_str()].join("");
        //println!("{}{}", &literal, &new_path_and_type);
        let clone : String = total_path.clone();

        let has_type = is_string || is_object || is_array || is_i64 || is_f64 || is_bool;
        if has_type {
            app_details_structure.insert(clone);
        }



        if value.is_object() {
            indentation = indentation + 1;
            parse_json_object(value, indentation, &total_path, app_details_structure);
            indentation = indentation - 1;
        }

        if value.is_array() {
            let as_array = value.as_array().unwrap();
            //println!("{} array length: {}, showing first item", &literal, as_array.len());

            if as_array.len() > 0 {
                let first_item = as_array.get(0).unwrap();

                if first_item.is_object() {
                    indentation = indentation + 1;
                    parse_json_object(first_item, indentation, &total_path, app_details_structure);
                    indentation = indentation - 1;
                }

            }

        }

    }
}
