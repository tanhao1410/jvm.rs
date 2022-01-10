#[allow(non_camel_case_types)]
pub struct DLOAD_1 {}

impl Instruction for DLOAD_1 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        DLOAD::_dload(thread, 1)
    }
}

impl Debug for DLOAD_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}