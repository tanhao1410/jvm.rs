#[allow(non_camel_case_types)]
pub struct ACONST_NULL {}

impl Instruction for ACONST_NULL {
    fn execute(&mut self,  frame: &mut Frame) {
        //frame.operand_stack().push_ref(Object::null());
        frame.operand_stack().push_slot(Slot::Nil());
    }
}

impl Debug for ACONST_NULL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}