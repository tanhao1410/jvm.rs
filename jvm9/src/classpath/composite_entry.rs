use crate::utils::path_utils::{SEPARATOR_STR, SEPARATOR};
use crate::classpath::entry::Entry;

pub struct CompositeEntry {
    entrys: Vec<Entry>
}

impl CompositeEntry {
    pub fn new(path: String) -> Self {
        //按;切割
        let paths = path
            .split(SEPARATOR)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Self::new_by_paths(paths)
    }

    pub(crate) fn new_by_paths(paths: Vec<String>) -> CompositeEntry {
        let mut entrys = Vec::new();
        for p in paths {
            entrys.push(Entry::new(p));
        }
        Self::new_by_entrys(entrys)
    }

    fn new_by_entrys(entrys: Vec<Entry>) -> CompositeEntry {
        Self { entrys }
    }

    pub(crate) fn read_class(&self, class_name: &str) -> Option<Vec<u8>> {
        for entry in &self.entrys {
            let res = entry.read_class(class_name);
            if res.is_some() {
                return res;
            }
        }
        return None;
    }
}

impl ToString for CompositeEntry {
    fn to_string(&self) -> String {
        let mut strs = Vec::new();
        for entry in &self.entrys {
            strs.push(entry.to_string());
        }
        strs.join(SEPARATOR_STR)
    }
}