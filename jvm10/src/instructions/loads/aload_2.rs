#[allow(non_camel_case_types)]
pub struct ALOAD_2 {}

impl Instruction for ALOAD_2 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        ALOAD::_aload(thread, 2)
    }
}

impl Debug for ALOAD_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}