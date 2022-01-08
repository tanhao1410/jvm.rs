#[allow(non_camel_case_types)]
pub struct ALOAD_1 {}

impl Instruction for ALOAD_1 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ALOAD::_aload(thread, 1)
    }
}

impl Debug for ALOAD_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}