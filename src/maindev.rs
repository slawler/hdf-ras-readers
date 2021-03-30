#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod json_reader;
mod ras_reader;

// Sample json data with key:value map of RAS HDF contents.
// Current path `/home/rust/src/keys.json` assumes running in docker container.
const HDF_KEP_MAP: &str = "/home/rust/data/keys.json";

// Sample HDF file (not pushed to remote repo)
const SAMPLE_DATA: &str = "/home/rust/data/Base_R299.hdf";

// Dev Process for reading RAS HDF output Files
fn main() {
    println!("Reading sample data from: {}", SAMPLE_DATA);

    let hdf_keys = json_reader::get_ras_map(HDF_KEP_MAP).unwrap();
    println!("Access to the following keys available: {:?}", hdf_keys);

    ras_reader::get_event_data(&SAMPLE_DATA, &hdf_keys.ArkansasBC);
    ras_reader::get_event_data(&SAMPLE_DATA, &hdf_keys.Miss_MemphisBC)
}
