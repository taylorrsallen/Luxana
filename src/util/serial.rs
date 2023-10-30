use std::{fs::*, io::*, path::Path, str::FromStr};
use serde::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Serial;
impl Serial {
    //==================================================================================================
    pub fn parse_dirs_in_path<P: AsRef<Path>, T: FromStr>(path: P) -> Vec<T> {
        let mut parsed_dirs = vec![];
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let entry = if let Ok(entry) = entry { entry } else { continue };
                if entry.path().is_dir() {
                    let dir_path = entry.path();
                    let dir_file_stem = if let Some(file_stem) = dir_path.file_stem() { file_stem } else { continue };
                    let dir_file_stem_str = if let Some(file_stem_str) = dir_file_stem.to_str() { file_stem_str } else { continue };
                    let parsed_dir: T = if let Ok(parsed_dir) = dir_file_stem_str.trim().parse() { parsed_dir } else { continue };
                    parsed_dirs.push(parsed_dir);
                }
            }
        }

        parsed_dirs
    }

    //==================================================================================================
    pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }
    
    //==================================================================================================
    pub fn save_ron_file_to_path<S0: AsRef<str>, S1: AsRef<str>, T: Serialize>(data: &T, path: S0, file_name: S1, depth_limit: usize) {
        let path_string = path.as_ref().to_owned() + "/" + file_name.as_ref() + ".ron";
        let path = Path::new(&path_string);
        Self::save_as_ron_string(data, path, depth_limit);
    }

    pub fn load_ron_file_from_path<S0: AsRef<str>, S1: AsRef<str>, T: for<'a> Deserialize<'a>>(path: S0, file_name: S1) -> Option<T> {
        let path_string = path.as_ref().to_owned() + "/" + file_name.as_ref() + ".ron";
        let path = Path::new(&path_string);
        Self::load_as_ron_string(path)
    }

    pub fn load_ron_file_from_path_or_create_default<S0: AsRef<str>, S1: AsRef<str>, T: Default + Serialize + for<'a> Deserialize<'a>>(path: S0, file_name: S1, depth_limit: usize) -> T {
        if let Some(data) = Self::load_ron_file_from_path(&path, &file_name) {
            data
        } else {
            let data = T::default();
            Self::save_ron_file_to_path(&data, path, file_name, depth_limit);
            data
        }
    }

    //==================================================================================================
    fn save_as_ron_string<P: AsRef<Path>, T: Serialize>(data: &T, path: P, depth_limit: usize) {
        Self::create_path(&path);
        if let Ok(contents) = Serial::to_ron_string_pretty(data, depth_limit) {
            Serial::try_write_file(path, contents.as_bytes());
        }
    }

    fn load_as_ron_string<P: AsRef<Path>, T: for<'a> Deserialize<'a>>(path: P) -> Option<T> {
        if let Some(contents) = Serial::file_to_string(path) {
            if let Ok(data) = ron::from_str(&contents) {
                return Some(data);
            }
        }

        None
    }

    //==================================================================================================
    pub fn create_path<P: AsRef<Path>>(path: P) {
        let prefix = path.as_ref().parent().unwrap();
        if std::fs::create_dir_all(prefix).is_err() { println!("Failed to create path for {}", prefix.display()); }
    }

    //==================================================================================================
    pub fn file_to_bytes<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
        if let Ok(file) = File::open(path) {
            let mut buf_reader = BufReader::new(file);
            let mut bytes: Vec<u8> = vec![];
            
            if let Ok(_) = buf_reader.read_to_end(&mut bytes) {
                if bytes.len() != 0 {
                    return Some(bytes);
                }
            }
        }
    
        None
    }
    
    pub fn file_to_string<P: AsRef<Path>>(path: P) -> Option<String> {
        if let Ok(file) = File::open(path) {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            
            if let Ok(_) = buf_reader.read_to_string(&mut contents) {
                if contents.as_str().len() != 0 {
                    return Some(contents);
                }
            }
        }
    
        None
    }
    
    //==================================================================================================
    fn try_write_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> bool {
        if let Ok(mut file) = File::create(path) {
            if let Ok(_) = file.write_all(bytes) {
                return true;
            }
        }
    
        false
    }
    
    fn to_ron_string_pretty<T: Serialize>(data: &T, depth_limit: usize) -> ron::Result<String> {
        ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::new().depth_limit(depth_limit))
    }
}