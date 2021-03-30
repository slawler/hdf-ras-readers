#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate hdf5;
extern crate ndarray;

// mod json_reader;
mod ras_reader;

use hdf5::Dataset;
use hdf5::Error as HDFError;
use hdf5::PropertyList;

use ndarray::Array2;
use std::io::{Error, ErrorKind};
use std::result::Result;

// Sample HDF file (not pushed to remote repo)
const SAMPLE_DATA: &str = "/home/rust/data/Base_R299.hdf";

// Dev Process for reading RAS HDF output Files
fn main() {
    // Comment this out for dev/debug to see hdf error code printed to stdout
    let _e = hdf5::silence_errors();
    println!("\nReading sample data from: {}\n", SAMPLE_DATA);

    // Recurse through file to get a list of available datasets
    if let Ok(hdf_datasets) = ras_reader::list_datasets(&SAMPLE_DATA) {
        let hdf_key = &hdf_datasets[0];

        if let Ok(file) = hdf5::File::open(&SAMPLE_DATA) {
            // PRINT ALL MEMBERS RECURSIVELY OPTION: Set to 1
            if 1 == 0 {
                let mut i: u64 = 0;
                for key in hdf_datasets {
                    let data: Result<hdf5::Dataset, HDFError> = file.dataset(&key);
                    println!("{:?} {} {}", i, data.unwrap().ndim(), key);
                    i = i + 1;
                }
            }

            let test_key: &str = "Results/Unsteady/Output/Output Blocks/DSS Profile Output/Unsteady Time Series/Time";
            let data: Result<hdf5::Dataset, HDFError> = file.dataset(&test_key);
            let obj = data.unwrap();
            // let dataArray = obj.read_raw();
            println!("{:?} {:?} {:?}", obj.ndim(), obj.shape(), obj.comment());
        }
    }
}
