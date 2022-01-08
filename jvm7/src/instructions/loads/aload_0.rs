#[allow(non_camel_case_types)]
pub struct ALOAD_0 {}

impl Instruction for ALOAD_0 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ALOAD::_aload(thread, 0)
    }
}

impl Debug for ALOAD_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}