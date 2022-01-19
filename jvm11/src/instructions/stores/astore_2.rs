#[allow(non_camel_case_types)]
pub struct ASTORE_2 {}

impl Instruction for ASTORE_2 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ASTORE::_astore(thread, 2);
    }
}

impl Debug for ASTORE_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}