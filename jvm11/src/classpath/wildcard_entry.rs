use std::fs;
use crate::classpath::composite_entry::CompositeEntry;

pub struct WildCardEntry{
    entry: CompositeEntry
}

impl WildCardEntry{
    // xx/xx/*
    pub fn new(mut path:String)->Self{

        path.remove(path.len() - 1);//去掉*

        let read_dir = fs::read_dir(path.as_str()).unwrap();

        let mut paths = Vec::new();

        //将该目录下的所有jar包加入
        for dir_entry in read_dir {
            let path_buf = dir_entry.unwrap().path();
            if path_buf.is_file() {
                //此处的file_name，是只有文件名，需要补上路径
                let file_name = path_buf.file_name().unwrap().to_str().unwrap();
                if file_name.ends_with(".jar") || file_name.ends_with(".JAR") {
                    paths.push(path_buf.to_str().unwrap().to_string());
                }
            }
        }
        // println!("{:?}", paths);
        Self { entry: CompositeEntry::new_by_paths(paths) }
    }

    pub fn read_class(&self, class_name: &str) -> Option<Vec<u8>> {
        self.entry.read_class(class_name)
    }
}

impl ToString for WildCardEntry {
    fn to_string(&self) -> String {
        self.entry.to_string()
    }
}