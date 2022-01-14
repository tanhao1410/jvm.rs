use crate::utils::file_util;
use std::path::Path;
use std::fs::File;

pub struct DirEntry {
    pub path: String
}

impl DirEntry {
    pub fn new(path: String) -> Self {
        DirEntry { path }
    }

    pub(crate) fn read_class(&self, class_name: &str) -> Option<Vec<u8>> {
        let pb = Path::new(&self.path).join(class_name);
        let path = pb.as_path();
        if path.is_file() {
            let file = File::open(path).unwrap();
            if let Ok(res) = file_util::read_file(&file) {
                return Some(res);
            }
        }
        None
    }
}


impl ToString for DirEntry {
    fn to_string(&self) -> String {
        self.path.to_string()
    }
}