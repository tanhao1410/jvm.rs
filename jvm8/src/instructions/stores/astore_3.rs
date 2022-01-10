#[allow(non_camel_case_types)]
pub struct ASTORE_3 {}

impl Instruction for ASTORE_3 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ASTORE::_astore(thread, 3);
    }
}

impl Debug for ASTORE_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}