#[allow(non_camel_case_types)]
pub struct FSTORE_2 {}

impl Instruction for FSTORE_2 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        FSTORE::_fstore(thread, 2);
    }
}

impl Debug for FSTORE_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}