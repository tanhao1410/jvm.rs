use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Read};

pub struct JarEntry{
    pub path : String
}

impl JarEntry{
    pub fn new(path:String)->Self{
        JarEntry{path}
    }

    pub(crate) fn read_class(&self, class_name: &str) -> Option<Vec<u8>> {
        let file = File::open(Path::new(&self.path)).unwrap();
        let reader = BufReader::new(file);
        let mut za = zip::ZipArchive::new(reader).unwrap();

        //let zf = za.by_name(dbg!(class_name));
        let zf = za.by_name(class_name);
        match zf {
            Err(_) => None,
            Ok(mut file) => {
                let mut v = Vec::new();
                let read_res = file.read_to_end(&mut v);
                if read_res.is_err() {
                    panic!("ZipEntry read file err {}", read_res.unwrap_err().to_string());
                }
                Some(v)
            }
        }
    }
}

impl ToString for JarEntry {
    fn to_string(&self) -> String {
        self.path.clone()
    }
}