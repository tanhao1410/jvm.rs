#[allow(non_camel_case_types)]
pub struct FSTORE_0 {}

impl Instruction for FSTORE_0 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        FSTORE::_fstore(thread, 0);
    }
}

impl Debug for FSTORE_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}