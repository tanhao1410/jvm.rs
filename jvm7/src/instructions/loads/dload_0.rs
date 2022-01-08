#[allow(non_camel_case_types)]
pub struct DLOAD_0 {}

impl Instruction for DLOAD_0 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        DLOAD::_dload(thread, 0)
    }
}

impl Debug for DLOAD_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}