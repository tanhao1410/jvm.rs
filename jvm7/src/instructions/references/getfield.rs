#[allow(non_camel_case_types)]
pub struct GET_FIELD {
    index: u16
}
impl GET_FIELD {
    pub fn new() -> Self {
        Self { index:0 }
    }
}

impl Instruction for GET_FIELD {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader){
        self.index = _reader.read_u16()
    }
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let guard = thread.read().unwrap();
        let rc = guard.current_frame();
let mut frame = rc.borrow_mut();
        let cp = frame.method.constant_pool();
        let current_class = cp.read().unwrap().get_class();
        //得到字段符号引用
        let mut guard = cp.write().unwrap();
        let field_ref = guard.get_constant_mut(self.index as usize);
        match field_ref {
            Constant::FieldRef(field_ref)=>{
                let field_guard = field_ref.field.as_ref().unwrap();
                let field = field_guard.read().unwrap();
                let ref_class = field.class.clone();

                if field.is_static() {
                    panic!("java.lang.IncompatibleClassChangeError");
                }

                let descriptor = field.descriptor.clone();
                let slot_id = field.slot_id;
                let stack = frame.operand_stack();

                match descriptor.chars().next() {
                    Some(c) => match c {
                        'Z' | 'B' | 'C' | 'S' | 'I' => {
                            let slot = stack.pop_slot();
                            //这里可能弹出的是空
                            match slot{
                                Slot::Nil()=>panic!("java.lang.NullPointerException"),
                                Slot::Ref(obj_ref)=>{
                                    stack.push_int(obj_ref.read().unwrap().fields.get_int(slot_id));
                                }
                                _=>panic!("put field error")
                            }
                        }
                        'F' => {
                            let slot = stack.pop_slot();
                            //这里可能弹出的是空
                            match slot{
                                Slot::Nil()=>panic!("java.lang.NullPointerException"),
                                Slot::Ref(obj_ref)=>{
                                    stack.push_float(obj_ref.read().unwrap().fields.get_float(slot_id));
                                }
                                _=>panic!("put field error")
                            }
                        }
                        'J' => {
                            let slot = stack.pop_slot();
                            //这里可能弹出的是空
                            match slot{
                                Slot::Nil()=>panic!("java.lang.NullPointerException"),
                                Slot::Ref(obj_ref)=>{
                                    stack.push_long(obj_ref.read().unwrap().fields.get_long(slot_id));
                                }
                                _=>panic!("put field error")
                            }
                        }
                        'D' => {
                            let slot = stack.pop_slot();
                            //这里可能弹出的是空
                            match slot{
                                Slot::Nil()=>panic!("java.lang.NullPointerException"),
                                Slot::Ref(obj_ref)=>{
                                    stack.push_double(obj_ref.read().unwrap().fields.get_double(slot_id));
                                }
                                _=>panic!("put field error")
                            }
                        }
                        'L' | '[' => {
                            let slot = stack.pop_slot();
                            //这里可能弹出的是空
                            match slot{
                                Slot::Nil()=>panic!("java.lang.NullPointerException"),
                                Slot::Ref(obj_ref)=>{
                                    stack.push_slot(obj_ref.read().unwrap().fields.get_slot(slot_id));
                                }
                                _=>panic!("put field error")
                            }
                        }
                        _ => panic!("impossible")
                    },
                    None => panic!("impossible")
                }
            },
            _=>panic!("get field 指令 error")
        }
    }
}

impl Debug for GET_FIELD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(getfield=)")
    }
}