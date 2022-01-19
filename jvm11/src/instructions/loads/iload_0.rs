#[allow(non_camel_case_types)]
pub struct ILOAD_0 {}

impl Instruction for ILOAD_0 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ILOAD::_iload(thread, 0)
    }
}

impl Debug for ILOAD_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}