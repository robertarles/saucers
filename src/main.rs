mod api_client;

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
        Some("apistatus") => api_client::get_api_status(),
        Some("job") => {
            let job_args = matches.subcommand_matches("job").unwrap();
            let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
            api_client::get_job(&job_id[..])
        },
        Some("assetfile") => {
            let job_args = matches.subcommand_matches("assetfile").unwrap();
            let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
            let asset_filename = job_args.value_of("filename").unwrap(); // required arg, safe to simply unwrap
            api_client::get_job_asset_file(&job_id[..], &asset_filename[..])
        },
        Some("assetlist") => {
            let job_args = matches.subcommand_matches("assetlist").unwrap();
            let job_id = job_args.value_of("id").unwrap(); // required arg, safe to simply unwrap
            api_client::get_job_asset_list(&job_id[..])
        },
        Some("jobs") => {
            // set a default 'max'
            let mut max = String::from("10");
            // get the args from the subcommand, if max is provided, use its val
            let jobs_args = matches.subcommand_matches("jobs").unwrap();
            if jobs_args.is_present("max") {
                max = jobs_args.value_of("max").unwrap().to_string();
            };
            api_client::get_jobs(&max[..])
        },
        None => println!("No subcommand was used"),
        _ => println!("Subcommand not implemented!"),
    }

}