#[allow(non_camel_case_types)]
pub struct ISTORE_0 {}

impl Instruction for ISTORE_0 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ISTORE::_istore(thread, 0);
    }
}
impl Debug for ISTORE_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}