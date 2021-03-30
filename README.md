# hdf-ras-readers
Testbed for reading data from ras hdf outpt files

---

### Contents

Environment 
 - `Dockerfile` (if you are like me and don't want to install rust & dependencies locally)
 - `docker-compose`
 - `hot-reloader.sh` Automatically recompiles and runs `main` on *save* for any file in __`/src`__. 
 

Rust *(.rs)*
- `Cargo.toml` for package management
 - `main` The development workhorse.
 - `ras_reader` Functions using the hdf5 crate for interrogating the RAS HDF file.
 - `json_reader` Functions for reading keys that map the RAS HDF file.

 Data Files
 - `keys.json` Placeholder for RAS hdf key map (currently has hardcoded paths)
 - `Base_R299.hdf` Local sample data  used for dev. 

 *NOTE*:  HDF files stored in the __`/data`__ directory should not be pushed to the remote repository. For local dev/testing you will need to replace this with a RAS HDF file and update the `keys.json` to reflect the data in that file (until this module is more mature).

---

Reference

HDF Crate [Docs](https://docs.rs/hdf5/0.7.0/hdf5/)

HDF Crate [Source](https://github.com/aldanor/hdf5-rust)