mod api_client;
use std::collections::HashMap;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {

    // handle the CLI args
    let matches = App::new("saucers")
        .version("0.9")
        .about("\nSaucelabs api util.\nIMPORTANT: This program expects you to have already set your saucelabs credentials in the environment variables SAUCE_USERNAME and SAUCE_ACCESS_KEY (e.g in your .bashrc or .zshrc, or Windows system properties)")
        .subcommand(SubCommand::with_name("apistatus")
            .about("Get the current saucelabs API status.")
            .arg(Arg::with_name("formatted")
                .short("o")
                .takes_value(true)
                .required(false)
                .help("Request output in columnar format, requires field names to include in table (e.g. \"id,status,passed\")")
            )
        )
        .subcommand(SubCommand::with_name("supportedplatforms")
            .about("Get a list of objects describing all the OS and browser platforms currently supported on a Sauce Labs API (e.g. appium or webdriver)")
            .arg(Arg::with_name("formatted")
                .short("o")
                .takes_value(true)
                .required(false)
                .help("Request output in columnar format, requires field names to include in table (e.g. \"id,status,passed\")")
            )
            .arg(Arg::with_name("api")
                .short("a")
                .takes_value(true)
                .required(true)
                .help("Specify the API to report on ( all | webdriver | appium )")
            )
        )
        .subcommand(SubCommand::with_name("uploads")
            .about("Get a list of files that have been uploaded to sauce storage.")
            .arg(Arg::with_name("formatted")
                .short("o")
                .takes_value(true)
                .required(false)
                .help("Request output in columnar format, requires field names to include in table (e.g. \"id,status,passed\")")
            )
        )
        .subcommand(SubCommand::with_name("upload")
            .about("Upload a file to your accounts sauce storage.")
            .arg(Arg::with_name("filename")
                .short("f")
                .takes_value(true)
                .required(true)
                .help("Filename, with path, to upload to sauce storage.")
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
            .arg(Arg::with_name("formatted")
                .short("o")
                .takes_value(true)
                .required(false)
                .help("Request output in columnar format, requires field names to include in table (e.g. \"id,status,passed\")")
            )
        )
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
        .subcommand(SubCommand::with_name("stopjob")
            .about("Terminates a running job")
            .arg(Arg::with_name("id")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("job ID")
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
            .arg(Arg::with_name("formatted")
                .short("o")
                .takes_value(true)
                .required(false)
                .help("Request output in columnar format, requires field names to include in table (e.g. \"id,status,passed\")")
            )
        )
        .subcommand(SubCommand::with_name("tunnels")
            .about("Get a list of tunnels available to the user account")
            .arg(Arg::with_name("formatted")
                .short("o")
                .takes_value(true)
                .required(false)
                .help("Request output in columnar format, requires field names to include in table (e.g. \"id,status,passed\")")
            )
        )
        .subcommand(SubCommand::with_name("tunnel")
            .about("Get information for a tunnel given its ID")
            .arg(Arg::with_name("tunnel_id")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("ID of the tunnel")
            )
            .arg(Arg::with_name("formatted")
                .short("o")
                .takes_value(true)
                .required(false)
                .help("Request output in columnar format, requires field names to include in table (e.g. \"id,status,passed\")")
            )
        )
        .subcommand(SubCommand::with_name("tunneljobs")
            .about("Get the number of jobs that are running through the tunnel over the past 60 seconds")
            .arg(Arg::with_name("tunnel_id")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("ID of the tunnel")
            )
            .arg(Arg::with_name("formatted")
                .short("o")
                .takes_value(true)
                .required(false)
                .help("Request output in columnar format, requires field names to include in table (e.g. \"id,status,passed\")")
            )
        )
        .get_matches();
    
    // select the subcommand to run
    match matches.subcommand_name() {
        Some("apistatus") => {
            let job_args = matches.subcommand_matches("apistatus").unwrap();
            call_get_api_status(job_args);
        },
        Some("supportedplatforms") => {
            let job_args = matches.subcommand_matches("supportedplatforms").unwrap();
            call_get_supported_platforms(job_args);
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
            let job_args = matches.subcommand_matches("jobs").unwrap();
            call_get_jobs(job_args);
        },
        Some("job") => {
            let job_args = matches.subcommand_matches("job").unwrap();
            call_get_job(job_args);
        },
        Some("stopjob") => {
            let job_args = matches.subcommand_matches("stopjob").unwrap();
            call_stop_job(job_args);
        },
        Some("uploads") => {
            let job_args = matches.subcommand_matches("uploads").unwrap();
            call_get_uploads(job_args);
        },
        Some("upload") => {
            let job_args = matches.subcommand_matches("upload").unwrap();
            call_post_upload(job_args);
        },
        Some("tunnels") => {
            let job_args = matches.subcommand_matches("tunnels").unwrap();
            call_get_tunnels(job_args);
        },
        Some("tunnel") => {
            let job_args = matches.subcommand_matches("tunnel").unwrap();
            call_get_tunnel(job_args);
        },
        Some("tunneljobs") => {
            let job_args = matches.subcommand_matches("tunneljobs").unwrap();
            call_get_tunnel_jobs(job_args);
        },
        None => println!("No subcommand was used.\nUse --help for subcommand help."),
        _ => println!("Subcommand not implemented!\nUse --help for subcommand help."),
    }

    fn call_post_upload(job_args: &clap::ArgMatches){
        let filename = job_args.value_of("filename").unwrap();
        let json_response = api_client::post_upload(filename);
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn call_get_api_status(job_args: &clap::ArgMatches){
        let json_response = api_client::get_api_status();
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn call_get_supported_platforms(job_args: &clap::ArgMatches){
        let api = job_args.value_of("api").unwrap();
        let json_response = api_client::get_supported_platforms(api);
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn call_get_tunnels(job_args: &clap::ArgMatches){
        let json_response = api_client::get_tunnels();
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn call_get_tunnel(job_args: &clap::ArgMatches){
        let tunnel_id = job_args.value_of("tunnel_id").unwrap();
        let json_response = api_client::get_tunnel(tunnel_id);
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn call_get_tunnel_jobs(job_args: &clap::ArgMatches){
        let tunnel_id = job_args.value_of("tunnel_id").unwrap();
        let json_response = api_client::get_tunnel_jobs(tunnel_id);
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn call_get_uploads(job_args: &clap::ArgMatches){
        let json_response = api_client::get_uploads();
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            let json_file_array = json_response.get("files").unwrap();
            print_formatted(&json_file_array, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn call_get_jobs(job_args: &clap::ArgMatches){
        // set a default 'max'
        let mut max = String::from("10");
        if job_args.is_present("max") {
            max = job_args.value_of("max").unwrap().to_string();
        };
        let json_response = api_client::get_jobs(&max[..]);
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn call_get_job(job_args: &clap::ArgMatches){
        let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
        let json_response = api_client::get_job(&job_id[..]);
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }

    fn call_stop_job(job_args: &clap::ArgMatches){
        let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
        let json_response = api_client::stop_job(&job_id[..]);
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
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
        if job_args.is_present("formatted") {
            let format_fields_string = job_args.value_of("formatted").unwrap();
            let format_fields_string_cleaned = format_fields_string.replace(" ","");
            let format_fields = format_fields_string_cleaned.split(",").collect();
            print_formatted(&json_response, format_fields);
        } else {
            println!("{}", json_response);
        }
    }



    fn print_formatted(json: &serde_json::Value, field_names: Vec<&str>) {
        //println!("[DEBUG] passed to print_formatted(): {}\n{:#?}", json, field_names);
        let mut field_vals_vec: std::vec::Vec<std::collections::HashMap<String,String>> = vec![];
        let mut field_lens: std::collections::HashMap<String,usize> = HashMap::new();
        for field in &field_names{
            field_lens.insert(field.to_string(), field.len());
        }
        if json.is_array() {
            for json_item in json.as_array().unwrap().iter(){
                let mut fields_map: std::collections::HashMap<String,String> = HashMap::new();
                for field_name in &field_names {
                    // println!("{:#?}", field_name);
                    let json_object = json_item.as_object().unwrap();
                    if json_object.contains_key(&field_name.to_string()) {
                        let field_val = json_object.get(&field_name.to_string()).unwrap().to_string().replace("\"", "");
                        fields_map.insert(field_name.to_string(), field_val);
                        let current_val_len = json_object.get(&field_name.to_string()).unwrap().to_string().len();
                        let record_len = field_lens.get(&field_name.to_string()).unwrap();
                        if &current_val_len > record_len { 
                            field_lens.insert(field_name.to_string(), current_val_len); 
                        }
                    }
    
                }
                field_vals_vec.push(fields_map);
            }
        } else {
            let mut fields_map: std::collections::HashMap<String,String> = HashMap::new();
            for field_name in &field_names {
                // println!("{:#?}", field_name);
                let json_object = json.as_object().unwrap();
                if json_object.contains_key(&field_name.to_string()) {
                    let field_val = json_object.get(&field_name.to_string()).unwrap().to_string().replace("\"", "");
                    fields_map.insert(field_name.to_string(), field_val);
                    let current_val_len = json_object.get(&field_name.to_string()).unwrap().to_string().len();
                    let record_len = field_lens.get(&field_name.to_string()).unwrap();
                    if &current_val_len > record_len { 
                        field_lens.insert(field_name.to_string(), current_val_len); 
                    }
                }

            }
            field_vals_vec.push(fields_map);
        }
        // print table
        for field_name in &field_names {
            print!("{val:<width$}",width=field_lens[&field_name[..]]+1, val=field_name)
        }
        let mut sum_of_widths = 0;
        for field in field_lens.iter() {
            sum_of_widths += field.1;
        }
        // add the right-pad width
        sum_of_widths += field_lens.len();
        println!("\n{:->width$}", "-", width=sum_of_widths); //repeated sum of field name + num of field names
        // for each entry in fields_map
        for entry in field_vals_vec.iter() {
            for field_name in &field_names {
                //println!("[DEBUG] current vals_vec entry {:#?}, field_name {}", &entry, &field_name);
                let record_width = field_lens[&field_name[..]];
                // if field name from user is not in saucelabs returned object, print blank space in table
                if entry.contains_key(&field_name[..]){
                    let field_val: String = entry.get(&field_name[..]).unwrap().to_string();
                    print!("{:<width$}", &field_val, width=record_width+1);
                }else{
                    print!("{:<width$}", "", width=record_width+1);
                }
            }
            println!();
        }
        println!();
    }
}