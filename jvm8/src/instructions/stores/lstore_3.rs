#[allow(non_camel_case_types)]
pub struct LSTORE_3 {}

impl Instruction for LSTORE_3 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        LSTORE::_lstore(thread, 3);
    }
}
impl Debug for LSTORE_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}