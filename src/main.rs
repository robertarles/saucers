mod api_client;
use std::collections::HashMap;

extern crate clap;
use clap::{Arg, App, SubCommand};
// use reqwest::Client;
// use futures::executor::block_on;

fn main() {

    // handle the CLI args
    let matches = App::new("args")
        .subcommand(SubCommand::with_name("apistatus")
            .about("Get the current saucelabs API status."))
        .subcommand(SubCommand::with_name("job")
            .about("Get data about a particular job ID.")
            .arg(Arg::with_name("id")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("job ID")
            )
            .arg(Arg::with_name("formatted")
                .short("o")
                .takes_value(true)
                .required(false)
                .help("Request output in columnar format, requires field names to include in table (e.g. \"id,status,passed\")")
            )
        )
        .subcommand(SubCommand::with_name("jobs")
            .about("Get a list of jobs data.")
            .arg(Arg::with_name("max")
                .short("m")
                .takes_value(true)
                .default_value("10")
                .help("max number of jobs to return")
            )
        )
        .subcommand(SubCommand::with_name("assetfile")
            .about("Get a file asset associated with a particular job ID. (logs, screenshots, etc.) See the 'assetlist' subcommand.")
            .arg(Arg::with_name("filename")
                .short("f")
                .takes_value(true)
                .required(true)
                .default_value("10")
                .help("name of asset file to return")
            )
            .arg(Arg::with_name("id")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("ID of the job")
            )
        )
        .subcommand(SubCommand::with_name("assetlist")
            .about("Get the asset list associated with a particular job ID")
            .arg(Arg::with_name("id")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("ID of the job")
            )
        )
        .get_matches();
    
    // select the subcommand to run
    match matches.subcommand_name() {
        Some("apistatus") => {
            api_client::get_api_status();
        },
        Some("job") => {
            let job_args = matches.subcommand_matches("job").unwrap();
            call_get_job(job_args);
        },
        Some("assetfile") => {
            let job_args = matches.subcommand_matches("assetfile").unwrap();
            call_get_job_asset_file(job_args);
        },
        Some("assetlist") => {
            let job_args = matches.subcommand_matches("assetlist").unwrap();
            call_get_job_asset_list(job_args);
        },
        Some("jobs") => {
            // get the args from the subcommand, if max is provided, use its val
            let job_args = matches.subcommand_matches("jobs").unwrap();
            call_get_jobs(job_args);
        },
        None => println!("No subcommand was used"),
        _ => println!("Subcommand not implemented!"),
    }

    fn call_get_job(job_args: &clap::ArgMatches){
        let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
        let json_response = api_client::get_job(&job_id[..]);
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn print_formatted(json: serde_json::Value, field_names: Vec<&str>) {
        //println!("[DEBUG] passed to print_formatted(): {}\n{:#?}", json, field_names);
        let mut field_vals_vec: std::vec::Vec<std::collections::HashMap<String,String>> = vec![];
        let mut field_lens: std::collections::HashMap<String,usize> = HashMap::new();
        for field in &field_names{
            field_lens.insert(field.to_string(), field.len());
        }
        if json.is_array() {
            for json_item in json.as_array().unwrap().iter(){
                println!("[DEBUG] json_item {:#?}", json_item);
                for field_name in &field_names {
                    println!("{:#?}", field_name);
                    //if json_item.as_object().unwrap().is_present(field_name){
                        // put field_name value in VALUES map
                        // /let mut fields_map: std::collections::HashMap<String,String> = HashMap::new();
                        // if len > field_lens[field_name] then field_lens[fields] = field_name.len
                    //}
                }
                // place VALUES map on fields_map
            }
        } else {   // else
            for field_name in &field_names {
                // println!("{:#?}", field_name);
                let mut fields_map: std::collections::HashMap<String,String> = HashMap::new();
                let json_object = json.as_object().unwrap();
                if json_object.contains_key(&field_name.to_string()) {
                    // println!("[DEBUG] json has key {}", &field_name.to_string());
                    fields_map.insert(field_name.to_string(), json_object.get(&field_name.to_string()).unwrap().to_string());
                    field_vals_vec.push(fields_map);
                    let current_val_len = json_object.get(&field_name.to_string()).unwrap().to_string().len();
                    let record_len = field_lens.get(&field_name.to_string()).unwrap();
                    if &current_val_len > record_len { 
                        field_lens.insert(field_name.to_string(), current_val_len); 
                    }
                }

            }
            println!("[DEBUG] field vals stored {:#?}", field_vals_vec);
            println!("[DEBUG] field len records {:#?}", field_lens);
            // place VALUES map on fields_map
        }
        // print table
        // for field_name in field_names
            //print("{:field_lens[field_name]+1}",field_name)
        // println!("\n{:repeated sum of field name + num of field names}", "-")
        // for each entry in fields_map
            // for each field_name in field_names
                // print("{:+repeated ${field_lens[field_name]-val.len} spaces}", val)
            //println!()
    }

    fn call_get_job_asset_file(job_args: &clap::ArgMatches){
        let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
        let asset_filename = job_args.value_of("filename").unwrap(); // required arg, safe to simply unwrap
        let text_response = api_client::get_job_asset_file(&job_id[..], &asset_filename[..]);
        println!("{}", text_response);
    }

    fn call_get_job_asset_list(job_args: &clap::ArgMatches){
        let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
        let json_response = api_client::get_job_asset_list(&job_id[..]);
        println!("{}", json_response);
    }

    fn call_get_jobs(job_args: &clap::ArgMatches){
        // set a default 'max'
        let mut max = String::from("10");
        if job_args.is_present("max") {
            max = job_args.value_of("max").unwrap().to_string();
        };
        let json_response = api_client::get_jobs(&max[..]);
        println!("{}", json_response);
    }
}