#[allow(non_camel_case_types)]
pub struct DSTORE_3 {}

impl Instruction for DSTORE_3 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        DSTORE::_dstore(thread, 3);
    }
}

impl Debug for DSTORE_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}