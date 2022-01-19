#[allow(non_camel_case_types)]
pub struct ILOAD_1 {}

impl Instruction for ILOAD_1 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ILOAD::_iload(thread, 1)
    }
}

impl Debug for ILOAD_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}