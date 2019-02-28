/*

config structs and methods relating to the clients connection towards the appliances

*/

use colored::*;
use serde::{Serialize, Deserialize};
use serde_yaml;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SolaceApiConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub ok_emoji: String,
    pub err_emoji: String
}


// read the config file
pub fn readconfig(config: String) -> Result<SolaceApiConfig, Box<std::error::Error>> {
    let file = std::fs::File::open(config)?;
    let config_data: SolaceApiConfig = serde_yaml::from_reader(file)?;
    Ok(config_data)
}
