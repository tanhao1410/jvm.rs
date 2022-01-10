#[allow(non_camel_case_types)]
pub struct ILOAD_2 {}

impl Instruction for ILOAD_2 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ILOAD::_iload(thread, 2)
    }
}

impl Debug for ILOAD_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}