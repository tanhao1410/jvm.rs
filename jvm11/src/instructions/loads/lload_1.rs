#[allow(non_camel_case_types)]
pub struct LLOAD_1 {}

impl Instruction for LLOAD_1 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        LLOAD::_lload(thread, 1)
    }
}

impl Debug for LLOAD_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}