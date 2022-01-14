#[allow(non_camel_case_types)]
pub struct ASTORE_1 {}

impl Instruction for ASTORE_1 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ASTORE::_astore(thread, 1);
    }
}

impl Debug for ASTORE_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}