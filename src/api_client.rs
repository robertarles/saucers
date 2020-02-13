use std::env;

extern crate reqwest;
extern crate serde;
extern crate serde_json;


static SAUCE_API_URL: &'static str = "https://saucelabs.com/rest/v1/";
static API_STATUS_PATH: &'static str = "info/status";
static GET_JOBS_PATH: &'static str = "/jobs";
static GET_JOB_PATH: &'static str = "/jobs";
static UPLOADS_PATH: &'static str = "/storage";

/**
 * returns the "healthcheck" api status from Saucelabs API
 */
pub fn get_api_status() -> serde_json::Value {

    let resp = reqwest::blocking::get(&format!("{}{}", &SAUCE_API_URL, &API_STATUS_PATH)).unwrap();
    //println!("API response: \n{:?}", resp.status());

    let body = resp.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");

    json
}

//apiURL UPLOADS_PATH/username
pub fn get_uploads() -> serde_json::Value {

    let sauce_username = env::var("SAUCE_USERNAME").unwrap();
    let sauce_access_key = env::var("SAUCE_ACCESS_KEY").unwrap();

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&format!("{}{}/{}", &SAUCE_API_URL, &UPLOADS_PATH, &sauce_username))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .send().unwrap();
    let body = resp.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");

    json
}

/**
 * return a list of jobs
 */
pub fn get_jobs(max: &str) -> serde_json::Value {
    
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

    json
}

/**
 * get a specific jobs details
 */
pub fn get_job(job_id: &str) -> serde_json::Value {
    
    let sauce_username = env::var("SAUCE_USERNAME").unwrap();
    let sauce_access_key = env::var("SAUCE_ACCESS_KEY").unwrap();
    
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&format!("{}{}/{}", &SAUCE_API_URL, &GET_JOB_PATH, &job_id))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .send().unwrap();
    let body = resp.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
    
    json
}


/**
 * get a particular job asset file, valid file names:
 * from their docs (I believe they are not correct/incomplete, at least the 'selenium host log?' is missing, there is another log file)
 *  selenium-server.log, video.mp4, XXXXscreenshot.png (where XXXX is a number between 0000 and 9999), final_screenshot.png
 */
pub fn get_job_asset_file(job_id: &str, asset_filename: &str) -> String {
    
    let sauce_username = env::var("SAUCE_USERNAME").unwrap();
    let sauce_access_key = env::var("SAUCE_ACCESS_KEY").unwrap();
    
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&format!("{}{}/jobs/{}/assets/{}", &SAUCE_API_URL, &sauce_username, &job_id, &asset_filename))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .send().unwrap();
    let body = resp.text().unwrap();
    
    body
}

/**
 * get the asset list associated with a particular job ID
 */
pub fn get_job_asset_list(job_id: &str) -> serde_json::Value{
    
    // apiURL+"/"+username+"/jobs/"+jobID+"/assets
    let sauce_username = env::var("SAUCE_USERNAME").unwrap();
    let sauce_access_key = env::var("SAUCE_ACCESS_KEY").unwrap();
    
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&format!("{}{}/jobs/{}/assets", &SAUCE_API_URL, &sauce_username, &job_id))
        .basic_auth(sauce_username.clone(), Some(sauce_access_key.clone()))
        .send().unwrap();
    let body = resp.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
    
    json
}