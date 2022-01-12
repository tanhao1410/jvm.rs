use crate::classpath::entry::Entry;
use std::ops::Add;

pub struct ClassPath {
    boot_classpath: Entry,
    ext_classpath: Entry,
    user_classpath: Entry
}

impl ClassPath {
    pub fn parse(jre_path : String,cp_option: String) -> ClassPath {
        Self {
            boot_classpath: Entry::new(jre_path.clone().add("/lib/*")),
            ext_classpath: Entry::new(jre_path.add("/lib/ext/*")),
            user_classpath: Entry::new(cp_option) ,
        }
    }
    pub fn read_class(&self, class_name: String) -> Option<Vec<u8>> {
        if let Some(res) = self.boot_classpath.read_class(&class_name){
            return Some(res);
        }
        if let Some(res) = self.ext_classpath.read_class(&class_name){
            return Some(res);
        }
        self.user_classpath.read_class(&class_name)
    }
}

impl ToString for ClassPath {
    fn to_string(&self) -> String {
        self.user_classpath.to_string()
    }
}