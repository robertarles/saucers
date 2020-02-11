extern crate clap;
use clap::{Arg, App, SubCommand};
extern crate reqwest;
// use reqwest::Client;
// use futures::executor::block_on;
extern crate serde;
extern crate serde_json;
use std::env;

static SAUCE_API_URL: &'static str = "https://saucelabs.com/rest/v1/";
static API_STATUS_PATH: &'static str = "info/status";
static GET_JOBS_PATH: &'static str = "/jobs";
static GET_JOB_PATH: &'static str = "/jobs";

fn main() {

    // handle the CLI args
    let matches = App::new("args")
        .subcommand(SubCommand::with_name("apistatus")
            .about("Get the current API status."))
        .subcommand(SubCommand::with_name("job")
            .about("Get job data.")
            .arg(Arg::with_name("id")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("job ID")
            )
        )
        .subcommand(SubCommand::with_name("jobs")
            .about("Get a list of job data.")
            .arg(Arg::with_name("max")
                .short("m")
                .takes_value(true)
                .default_value("10")
                .help("max number of jobs to return")
            )
        )
        .subcommand(SubCommand::with_name("assetfile")
            .about("Get a list of job data.")
            .arg(Arg::with_name("filename")
                .short("f")
                .takes_value(true)
                .required(true)
                .default_value("10")
                .help("name of job asset file to return")
            )
            .arg(Arg::with_name("id")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("job ID")
            )
        )
        .get_matches();
    
    match matches.subcommand_name() {
        Some("apistatus") => get_api_status(),
        Some("job") => {
            let job_args = matches.subcommand_matches("job").unwrap();
            let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
            get_job(&job_id[..])
        },
        Some("assetfile") => {
            let job_args = matches.subcommand_matches("assetfile").unwrap();
            let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
            let asset_filename = job_args.value_of("filename").unwrap(); // required arg, safe to simply unwrap
            get_job_asset_file(&job_id[..], &asset_filename[..])
        },
        Some("jobs") => {
            // set a default 'max'
            let mut max = String::from("10");
            // get the args from the subcommand, if max is provided, use its val
            let jobs_args = matches.subcommand_matches("jobs").unwrap();
            if jobs_args.is_present("max") {
                max = jobs_args.value_of("max").unwrap().to_string();
            };
            get_jobs(&max[..])
        },
        None => println!("No subcommand was used"),
        _ => println!("Subcommand not implemented!"),
    }

}

fn get_api_status()  {

    let resp = reqwest::blocking::get(&format!("{}{}", &SAUCE_API_URL, &API_STATUS_PATH)).unwrap();
    //println!("API response: \n{:?}", resp.status());

    let body = resp.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");

    println!("{}", json);

}

fn get_jobs(max: &str) {
    
    let sauce_username = env::var("SAUCE_USERNAME").unwrap();
    let sauce_access_key = env::var("SAUCE_ACCESS_KEY").unwrap();
    
    let jobs_params = format!("?limit={}&full=true", max);
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&format!("{}{}{}", &SAUCE_API_URL, &GET_JOBS_PATH, &jobs_params))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .send().unwrap();
    let body = resp.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
    println!("{}", json);
}

fn get_job(job_id: &str) {
    
    let sauce_username = env::var("SAUCE_USERNAME").unwrap();
    let sauce_access_key = env::var("SAUCE_ACCESS_KEY").unwrap();
    
    // TODO: alter from get_jobs() code to get_job()
    // TODO: handle job ID param
    // TODO: ensure url is correct
    
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&format!("{}{}/{}", &SAUCE_API_URL, &GET_JOB_PATH, &job_id))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .send().unwrap();
    let body = resp.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
    println!("{}", json);
}

fn get_job_asset_file(job_id: &str, asset_filename: &str) {
    
    let sauce_username = env::var("SAUCE_USERNAME").unwrap();
    let sauce_access_key = env::var("SAUCE_ACCESS_KEY").unwrap();
    
    // TODO: alter from get_jobs() code to get_job()
    // TODO: handle job ID param
    // TODO: ensure url is correct
    
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&format!("{}{}/jobs/{}/assets/{}", &SAUCE_API_URL, &sauce_username, &job_id, &asset_filename))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .send().unwrap();
    let body = resp.text().unwrap();
    println!("{}", body);
}