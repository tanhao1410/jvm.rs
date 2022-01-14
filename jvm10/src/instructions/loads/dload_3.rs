#[allow(non_camel_case_types)]
pub struct DLOAD_3 {}

impl Instruction for DLOAD_3 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        DLOAD::_dload(thread, 3)
    }
}

impl Debug for DLOAD_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}