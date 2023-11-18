use std::{fs::*, io::*, path::Path, str::FromStr};
use serde::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Serial;
impl Serial {
    //==============================================================================================
    pub fn save_type_to_ron<S0: AsRef<str>, S1: AsRef<str>, T: Serialize>(data: &T, path: S0, file_name: S1, depth_limit: usize) {
        let path_string = path.as_ref().to_owned() + "/" + file_name.as_ref() + ".ron";
        let path = Path::new(&path_string);
        Self::save_from_ron_string(data, path, depth_limit);
    }

    pub fn load_type_from_ron<S0: AsRef<str>, S1: AsRef<str>, T: for<'a> Deserialize<'a>>(path: S0, file_name: S1) -> Option<T> {
        let path_string = path.as_ref().to_owned() + "/" + file_name.as_ref() + ".ron";
        let path = Path::new(&path_string);
        Self::load_from_ron_string(path)
    }

    pub fn load_string_from_ron<S0: AsRef<str>, S1: AsRef<str>>(path: S0, file_name: S1) -> Option<String> {
        let path_string = path.as_ref().to_owned() + "/" + file_name.as_ref() + ".ron";
        let path = Path::new(&path_string);
        Self::file_to_string(path)
    }

    pub fn load_type_or_default_from_ron<S0: AsRef<str>, S1: AsRef<str>, T: Default + Serialize + for<'a> Deserialize<'a>>(path: S0, file_name: S1, depth_limit: usize) -> T {
        if let Some(data) = Self::load_type_from_ron(&path, &file_name) {
            data
        } else {
            let data = T::default();
            Self::save_type_to_ron(&data, path, file_name, depth_limit);
            data
        }
    }

    //==============================================================================================
    fn save_from_ron_string<P: AsRef<Path>, T: Serialize>(data: &T, path: P, depth_limit: usize) {
        Self::create_file_path(&path);
        if let Ok(contents) = Serial::to_ron_string_pretty(data, depth_limit) {
            Serial::try_write_file(path, contents.as_bytes());
        }
    }

    fn load_from_ron_string<P: AsRef<Path>, T: for<'a> Deserialize<'a>>(path: P) -> Option<T> {
        if let Some(contents) = Serial::file_to_string(path) {
            if let Ok(data) = ron::from_str(&contents) {
                return Some(data);
            }
        }

        None
    }

    //==============================================================================================
    pub fn create_file_path<P: AsRef<Path>>(path: P) {
        let prefix = path.as_ref().parent().unwrap();
        if std::fs::create_dir_all(prefix).is_err() { println!("Failed to create path for {}", prefix.display()); }
    }

    pub fn create_directory_path<P: AsRef<Path>>(path: P) {
        if std::fs::create_dir_all(path.as_ref()).is_err() { println!("Failed to create path for {}", path.as_ref().display()); }
    }

    //==============================================================================================
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
    
    //==============================================================================================
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

    //==============================================================================================
    // pub fn file_paths_from_directory_recursive_exclusive(directory: String, extension: String) -> Vec<String> {

    // }

    /// `base_directory` will be added to the beginning of `search_directory`, but excluded from the paths returned.
    /// 
    /// Returns `(Vec<DirectoryPaths>, Vec<FilePaths>)`
    pub fn paths_from_directory<S0: AsRef<str>, S1: AsRef<str>>(base_directory: S0, search_directory: S1) -> (Vec<String>, Vec<String>) {
        let (mut directories, mut files) = (vec![], vec![]);
        
        if let Ok(entries) = read_dir(&(base_directory.as_ref().to_owned() + "/" + search_directory.as_ref())) {
            for entry in entries {
                let Ok(entry) = entry else { continue };
                let Some(entry_name) = Serial::try_get_entry_name(&entry) else { continue };
                let path = search_directory.as_ref().to_owned() + "/" + &entry_name;
                if entry.path().is_dir() { directories.push(path); } else { files.push(path); }
            }
        }

        (directories, files)
    }

    /// `base_directory` will be added to the beginning of `search_directory`, but excluded from the paths returned.
    pub fn file_paths_from_directory<S0: AsRef<str>, S1: AsRef<str>>(base_directory: S0, search_directory: S1) -> Vec<String> {
        Self::paths_from_directory(base_directory, search_directory).1
    }

    /// `base_directory` will be added to the beginning of `search_directory`, but excluded from the paths returned.
    pub fn file_paths_from_directory_recursive<S0: AsRef<str>, S1: AsRef<str>>(base_directory: S0, search_directory: S1) -> Vec<String> {
        let (mut directories, mut files) = Self::paths_from_directory(base_directory.as_ref(), search_directory.as_ref());

        loop {
            let Some(child_directory) = directories.pop() else { break };
            let (new_directories, new_files) = Self::paths_from_directory(base_directory.as_ref(), &child_directory);
            directories.extend(new_directories);
            files.extend(new_files);
        }

        files
    }

    pub fn try_get_entry_name(entry: &DirEntry) -> Option<String> {
        if let Some(file_stem) = entry.path().file_stem() {
            if let Some(stem_str) = file_stem.to_str() { Some(stem_str.to_string()) } else { None }
        } else {
            None
        }
    }
}