#[allow(non_camel_case_types)]
pub struct FLOAD_3 {}

impl Instruction for FLOAD_3 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        FLOAD::_fload(thread, 3)
    }
}

impl Debug for FLOAD_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}