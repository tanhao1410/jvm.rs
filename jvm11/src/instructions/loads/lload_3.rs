#[allow(non_camel_case_types)]
pub struct LLOAD_3 {}

impl Instruction for LLOAD_3 {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        LLOAD::_lload(thread, 31)
    }
}

impl Debug for LLOAD_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}