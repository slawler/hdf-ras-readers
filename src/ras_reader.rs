extern crate hdf5;
extern crate ndarray;

use ras_reader::hdf5::Dataset;
use ras_reader::hdf5::Error as HDFError;
use ras_reader::hdf5::Error;
use ras_reader::ndarray::Array2;

// Dev function for using the hdf5 library with RAS HDF files.
// `file_path`: is the reference for a path string to a RAS HDF output file
// `hdf_key`: is the reference for a key storing data in the RAS HDF output file
pub fn get_event_data(file_path: &str, hdf_key: &str) {
    if let Ok(file) = hdf5::File::open(file_path) {
        let data: std::result::Result<hdf5::Dataset, Error> = file.dataset(hdf_key);

        let d = data.unwrap();
        println!("Data Array Shape: {:?}", d.shape());
        println!("Data Array Dimensions: {:?}", d.ndim());
        println!("Data Size: {:?}", d.size());
        println!("Storage Size: {:?}", d.storage_size());
        println!("Data Type: {:?}", d);

        let values: std::result::Result<Array2<i32>, Error> = d.read_2d();
        println!("{:?}", values);
    } else {
        println!("No file found at {}", file_path)
    }
}

/*----------HDF LIST DATASETS -----------*/

// Read contents of an HDF file and create a list of all datasets
// TODO: Function needs error handling
pub fn list_datasets(file_path: &str) -> Result<Vec<String>, Error> {
    // Instantiate a vector to store results
    let mut datasets = Vec::new();

    // Open HDF file as a file object
    if let Ok(file) = hdf5::File::open(file_path) {
        // Start with the top level groups and recurse over each
        for obj in file.member_names().unwrap().iter() {
            get_subs(&obj, &file, &mut datasets);
        }
    } else {
        println!("No HDF file found at {}", file_path);
    }

    Ok(datasets)
}


// Recursive function returning all datasets in the HDF file. Called by `list_datasets`
pub fn get_subs(key: &str, f: &hdf5::File, v: &mut Vec<String>) -> Result<(), HDFError> {
    // Try to access the group, if successful, build keys using member names
    if let Ok(next_level) = f.group(key) {
        // Count members in current level
        let n = next_level.len() as usize;

        // Iterate through member names if there are multiple
        if n > 0 {
            // Construct HDF key for the current level group or dataset
            for obj in next_level.member_names().unwrap().iter() {
                let new_key = format!("{}/{}", key, obj);
                // If `new_key` is a group, recurse through this level
                if let Ok(new_group) = f.group(&new_key) {
                    get_subs(&new_key, &f, v);

                // If `new_key` is not a group, assume it is a dataset & append to `v` (list of datasets)
                // Additional features could be added here
                // (e.g. add data type & dimensions to return a map instead of a vector?)
                } else {
                    v.push(new_key.to_string());
                }
            }
            Ok(())
        } else {
            // May want to do more than ignore these?
            // println!("No Next Level for : {:?}:: Empty Group?", key);
            Ok(())
        }
    } else {
        // Placeholder to flag something the current logic is missing.
        // In a perfect world, this never shows up. Maybe replace with a panic?
        println!("These aren't the drones you're looking for...");
        Ok(())
    }
}
