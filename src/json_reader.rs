extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// Placeholder Struct for Known HDF-RAS Paths
// stored in `keys.json`
#[derive(serde::Deserialize, Debug)]
pub struct RasPath {
    pub ArkansasBC: String,
    pub Miss_MemphisBC: String,
    pub WhiteBC: String,
    pub YazooBC: String,
}

// Read keys.json data into to RasPath struct
pub fn get_ras_map<P: AsRef<Path>>(path: P) -> Result<RasPath, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let path_map = serde_json::from_reader(reader)?;

    Ok(path_map)
}
