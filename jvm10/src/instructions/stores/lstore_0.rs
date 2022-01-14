#[allow(non_camel_case_types)]
pub struct LSTORE_0 {}

impl Instruction for LSTORE_0 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        LSTORE::_lstore(thread, 0);
    }
}
impl Debug for LSTORE_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}