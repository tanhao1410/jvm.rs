#[allow(non_camel_case_types)]
pub struct FLOAD_0 {}

impl Instruction for FLOAD_0 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        FLOAD::_fload(thread, 0)
    }
}

impl Debug for FLOAD_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}