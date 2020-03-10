use std::env;
use std::path::Path;
use std::fs::File;
use std::time::Duration;

extern crate reqwest;
extern crate serde;
extern crate serde_json;


static SAUCE_API_URL: &'static str = "https://saucelabs.com/rest/v1/";
static API_STATUS_PATH: &'static str = "info/status";
static GET_JOBS_PATH: &'static str = "jobs";
static GET_JOB_PATH: &'static str = "jobs";
static STOP_JOB_PATH: &'static str = "{{USERNAME}}/jobs/{{JOB_ID}}/stop";
static UPLOADS_PATH: &'static str = "storage";
static TUNNELS_PATH: &'static str = "{{USERNAME}}/tunnels";
static TUNNEL_JOB_PATH: &'static str = "{{USERNAME}}/tunnels/{{TUNNEL_ID}}/";
static TUNNEL_JOBS_PATH: &'static str = "{{USERNAME}}/tunnels/{{TUNNEL_ID}}/num_jobs";
static SUPPORTED_PLATFORMS_PATH: &'static str = "info/platforms/{{AUTOMATION_API}}";

/**
 * returns the "healthcheck" api status from Saucelabs API
 */
pub fn get_api_status() -> Result<String, reqwest::Error> {

    let client = reqwest::blocking::Client::new();
    let body = client
        .get(&format!("{}{}", &SAUCE_API_URL, &API_STATUS_PATH))
        .timeout(Duration::from_secs(120))
        .send()?.text()?;

    Ok(body)
}

/**
 * returns platforms supported, by API (appium | webdriver)
 */
pub fn get_supported_platforms(automation_api: &str) -> Result<String, reqwest::Error> {

    let client = reqwest::blocking::Client::new();
    let body = client
        .get(&format!("{}{}", &SAUCE_API_URL, &SUPPORTED_PLATFORMS_PATH.replace("{{AUTOMATION_API}}", &automation_api)))
        .timeout(Duration::from_secs(120))
        .send()?.text()?;
    
    Ok(body)
}

pub fn get_tunnels() -> Result<String, reqwest::Error> {

    let (sauce_username, sauce_access_key) = load_sauce_credentials();

    //curl -u USERNAME:ACCESS_KEY https://saucelabs.com/rest/v1/USERNAME/tunnels
    let client = reqwest::blocking::Client::new();
    let body = client
        .get(&format!("{}{}", &SAUCE_API_URL, &TUNNELS_PATH.replace("{{USERNAME}}", &sauce_username)))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .send()?.text()?;

    Ok(body)
}

pub fn get_tunnel(tunnel_id: &str) -> Result<String, reqwest::Error> {

    let (sauce_username, sauce_access_key) = load_sauce_credentials();

    let client = reqwest::blocking::Client::new();
    let body = client
        .get(&format!("{}{}", &SAUCE_API_URL, &TUNNEL_JOB_PATH.replace("{{USERNAME}}", &sauce_username).replace("{{TUNNEL_ID}}", tunnel_id)))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .send()?.text()?;

    Ok(body)

}
pub fn get_tunnel_jobs(tunnel_id: &str) -> Result<String, reqwest::Error> { 

    let (sauce_username, sauce_access_key) = load_sauce_credentials();

    let client = reqwest::blocking::Client::new();
    let body = client
        .get(&format!("{}{}", &SAUCE_API_URL, &TUNNEL_JOBS_PATH.replace("{{USERNAME}}", &sauce_username).replace("{{TUNNEL_ID}}", tunnel_id)))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .send()?.text()?;

    Ok(body)

}

pub fn post_upload(filename: &str) -> Result<String, reqwest::Error> {

    let (sauce_username, sauce_access_key) = load_sauce_credentials();

    let path = Path::new(&filename);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let file = File::open(&filename).unwrap();
    let body = reqwest::blocking::Client::new()
        .post(&format!("{}{}/{}/{}?overwrite=true", &SAUCE_API_URL, &UPLOADS_PATH, &sauce_username, &file_name))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .body(file)
        .send()?.text()?;
    
    Ok(body)
}

pub fn get_uploads() -> Result<String, reqwest::Error> {

    let (sauce_username, sauce_access_key) = load_sauce_credentials();

    let client = reqwest::blocking::Client::new();
    let body = client
        .get(&format!("{}{}/{}", &SAUCE_API_URL, &UPLOADS_PATH, &sauce_username))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .send()?.text()?;

    Ok(body)
}

/**
 * return a list of jobs
 */
pub fn get_jobs(max: &str) -> Result<String, reqwest::Error> {

    let (sauce_username, sauce_access_key) = load_sauce_credentials();
    
    let jobs_params = format!("?limit={}&full=true", max);
    let client = reqwest::blocking::Client::new();
    let body = client
        .get(&format!("{}{}{}", &SAUCE_API_URL, &GET_JOBS_PATH, &jobs_params))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .send()?.text()?;

    Ok(body)
}

/**
 * get a specific jobs details
 */
pub fn get_job(job_id: &str) -> Result<String, reqwest::Error>  {

    let (sauce_username, sauce_access_key) = load_sauce_credentials();
    
    let client = reqwest::blocking::Client::new();
    let body = client
        .get(&format!("{}{}/{}", &SAUCE_API_URL, &GET_JOB_PATH, &job_id))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .send()?.text()?;
    
    Ok(body)
}

/**
 * stop a specific job
 */
pub fn stop_job(job_id: &str) -> Result<String, reqwest::Error>  {

    let (sauce_username, sauce_access_key) = load_sauce_credentials();

    let client = reqwest::blocking::Client::new();
    let body = client
        .put(&format!( "{}{}", &SAUCE_API_URL, &STOP_JOB_PATH.replace("{{USERNAME}}", &sauce_username).replace("{{JOB_ID}}", &job_id) ))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .send()?.text()?;

    Ok(body)
}

/**
 * get a particular job asset file, valid file names:
 * from their docs (I believe they are not correct/incomplete, at least the 'selenium host log?' is missing, there is another log file)
 *  selenium-server.log, video.mp4, XXXXscreenshot.png (where XXXX is a number between 0000 and 9999), final_screenshot.png
 */
pub fn get_job_asset_file(job_id: &str, asset_filename: &str) -> String {
    
    let (sauce_username, sauce_access_key) = load_sauce_credentials();
    
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&format!("{}{}/jobs/{}/assets/{}", &SAUCE_API_URL, &sauce_username, &job_id, &asset_filename))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .send().unwrap();
    let body = resp.text().unwrap();
    
    body
}

/**
 * get the asset list associated with a particular job ID
 */
pub fn get_job_asset_list(job_id: &str) -> serde_json::Value{
    
    let (sauce_username, sauce_access_key) = load_sauce_credentials();
    
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&format!("{}{}/jobs/{}/assets", &SAUCE_API_URL, &sauce_username, &job_id))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .timeout(Duration::from_secs(120))
        .send().unwrap();
    let body = resp.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
    
    json
}

/**
 * Load the saucelabs credentials from environment variables
 */
fn load_sauce_credentials() -> (String,String) {

    let sauce_username: String;
    match env::var("SAUCE_USERNAME") {
        Ok(v) => sauce_username = (*v).to_string(),
        Err(e) => {
            println!("Error reading SAUCE_USERNAME: {}", e);
            std::process::exit(1);
        }
    }

    let sauce_access_key:  String;
    match env::var("SAUCE_ACCESS_KEY") {
        Ok(v) => sauce_access_key = (*v).to_string(),
        Err(e) => {
            println!("Error reading SAUCE_ACCESS_KEY: {}", e);
            std::process::exit(1);
        }
    }

    (sauce_username, sauce_access_key)
}