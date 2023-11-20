use std::{fs::*, io::*, path::Path, str::FromStr};
use serde::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Serial;
impl Serial {
    /// Assumes that the last element in path is a file
    pub fn create_file_path<P: AsRef<Path>>(path: P) -> bool {
        let Some(prefix) = path.as_ref().parent() else { println!("Failed to get prefix path for {}", path.as_ref().display()); return false };
        if std::fs::create_dir_all(prefix).is_ok() { true } else { println!("Failed to create path for {}", prefix.display()); false }
    }

    /// Assumes that the last element in path is a directory
    pub fn create_directory_path<P: AsRef<Path>>(path: P) -> bool {
        if std::fs::create_dir_all(path.as_ref()).is_ok() { true } else { println!("Failed to create path for {}", path.as_ref().display()); false }
    }

    //==============================================================================================
    fn type_to_ron_string_with_depth_limit<T: Serialize>(data: &T, depth_limit: usize) -> ron::Result<String> {
        ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::new().depth_limit(depth_limit))
    }

    /// Returns false if the file fails to create or to write
    fn try_write_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> bool {
        let Ok(mut file) = File::create(path) else { return false };
        if file.write_all(bytes).is_ok() { true } else { false }
    }

    /// Returns false if the file fails to create or to write
    fn try_write_file_and_path<P: AsRef<Path>>(path: P, bytes: &[u8]) -> bool {
        Self::create_file_path(&path);
        Self::try_write_file(path, bytes)
    }

    //==============================================================================================
    pub fn try_get_bytes_from_path<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
        let mut buf_reader = if let Ok(file) = File::open(path) { BufReader::new(file) } else { return None };
        let mut bytes: Vec<u8> = vec![];
        if buf_reader.read_to_end(&mut bytes).is_err() { return None; }
        if bytes.len() != 0 { Some(bytes) } else { None }
    }
    
    pub fn try_get_string_from_path<P: AsRef<Path>>(path: P) -> Option<String> {
        let mut buf_reader = if let Ok(file) = File::open(path) { BufReader::new(file) } else { return None };
        let mut contents = String::new();
        if buf_reader.read_to_string(&mut contents).is_err() { return None; }
        if contents.as_str().len() != 0 { Some(contents) } else { None }
    }

    //==============================================================================================
    /// - `directory` should not end with `/`
    /// - `file_name` can contain directories, but should not start with `/`, and the end should be only the name of the desired file
    /// - `extension` should not include a `.`
    pub fn path_string<S0: AsRef<str>, S1: AsRef<str>, S2: AsRef<str>>(directory: S0, file_name: S1, extension: S2) -> String {
        directory.as_ref().to_owned() + "/" + file_name.as_ref() + "." + extension.as_ref()
    }

    //==============================================================================================
    /// - `directory` should not end with `/`
    /// - `file_name` can contain directories, but should not start with `/`, and the end should be only the name of the desired file
    /// - `depth_limit` max indentations
    pub fn save_type_to_ron_file<S0: AsRef<str>, S1: AsRef<str>, T: Serialize>(data: &T, directory: S0, file_name: S1, depth_limit: usize) {
        let Ok(contents) = Self::type_to_ron_string_with_depth_limit(data, depth_limit) else { return };
        Self::try_write_file_and_path(&Self::path_string(directory, file_name, "ron"), contents.as_bytes());
    }

    /// - `directory` should not end with `/`
    /// - `file_name` can contain directories, but should not start with `/`, and the end should be only the name of the desired file
    pub fn load_type_from_ron_file<S0: AsRef<str>, S1: AsRef<str>, T: for<'a> Deserialize<'a>>(directory: S0, file_name: S1) -> Option<T> {
        let Some(contents) = Self::load_string_from_ron_file(directory, file_name) else { return None };
        if let Ok(data) = ron::from_str(&contents) { Some(data) } else { None }
    }

    /// - `directory` should not end with `/`
    /// - `file_name` can contain directories, but should not start with `/`, and the end should be only the name of the desired file
    pub fn load_string_from_ron_file<S0: AsRef<str>, S1: AsRef<str>>(directory: S0, file_name: S1) -> Option<String> {
        Self::try_get_string_from_path(&Self::path_string(directory, file_name, "ron"))
    }

    /// - `directory` should not end with `/`
    /// - `file_name` can contain directories, but should not start with `/`, and the end should be only the name of the desired file
    pub fn remove_ron_file<S0: AsRef<str>, S1: AsRef<str>>(directory: S0, file_name: S1) {
        std::fs::remove_file(&Self::path_string(directory, file_name, "ron"));
    }

    //==============================================================================================
    pub fn try_get_entry_name(entry: &DirEntry) -> Option<String> {
        let path = entry.path();
        let Some(file_stem) = path.file_stem() else { return None };
        if let Some(stem_str) = file_stem.to_str() { Some(stem_str.to_string()) } else { None }
    }

    /// `base_directory` will be added to the beginning of `search_directory`, but excluded from the paths returned.
    /// 
    /// Returns `(Vec<DirectoryPaths>, Vec<FilePaths>)`
    pub fn paths_from_directory<S0: AsRef<str>, S1: AsRef<str>>(base_directory: S0, search_directory: S1) -> (Vec<String>, Vec<String>) {
        let (mut directories, mut files) = (vec![], vec![]);
        
        if let Ok(entries) = read_dir(&(base_directory.as_ref().to_owned() + "/" + search_directory.as_ref())) {
            for entry in entries {
                let Ok(entry) = entry else { continue };
                let Some(entry_name) = Self::try_get_entry_name(&entry) else { continue };
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
}