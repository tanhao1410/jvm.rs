use crate::classpath::dir_entry::DirEntry;
use crate::classpath::jar_entry::JarEntry;
use crate::classpath::composite_entry::CompositeEntry;
use crate::classpath::wildcard_entry::WildCardEntry;

use crate::utils::path_utils::SEPARATOR;
use std::fmt::{Debug, Formatter};

//对于java中一个接口多个实现的，可以用enum来代替，因为vec中不能放trait，只能放具体object，这与java不同，java的集合里是可以直接放接口的
pub enum Entry{
    DirEntry(DirEntry),
    JarEntry(JarEntry),
    CompositeEntry(CompositeEntry),
    WildCardEntry(WildCardEntry)
}

impl Entry{
    pub fn new(path:String)->Entry{
        if path.contains(SEPARATOR){
            Entry::CompositeEntry(CompositeEntry::new(path))
        }else if path.ends_with('*'){
            Entry::WildCardEntry(WildCardEntry::new(path))
        }else if path.ends_with(".jar"){
            Entry::JarEntry(JarEntry::new(path))
        }else{
            Entry::DirEntry(DirEntry::new(path))
        }
    }

    pub fn read_class(&self, class_name: &str) -> Option<Vec<u8>> {
        static CLASSNAME_SUFFIX: &str = ".class";
        let class_name = &if class_name.ends_with(CLASSNAME_SUFFIX) {
            class_name.to_string()
        } else {
            class_name.to_string() + CLASSNAME_SUFFIX
        };
        //match dbg!(self) {
        match self {
            Entry::CompositeEntry(e) => e.read_class(class_name),
            Entry::WildCardEntry(e) => e.read_class(class_name),
            Entry::JarEntry(e) => e.read_class(class_name),
            Entry::DirEntry(e) => e.read_class(class_name),
        }
    }

}

impl ToString for Entry{
    fn to_string(&self) -> String {
        match self {
            Entry::CompositeEntry(e) => "Entry::CompositeEntry[".to_owned() + &e.to_string() + "]",
            Entry::WildCardEntry(e) => "Entry::WildCardEntry[".to_owned() + &e.to_string() + "]",
            Entry::JarEntry(e) => "Entry::JarEntry[".to_owned() + &e.to_string() + "]",
            Entry::DirEntry(e) => "Entry::DirEntry[".to_owned() + &e.to_string() + "]",
        }
    }
}


impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}