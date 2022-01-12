#[allow(non_camel_case_types)]
pub struct ASTORE_0 {}

impl Instruction for ASTORE_0 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ASTORE::_astore(thread, 0);
    }
}

impl Debug for ASTORE_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}