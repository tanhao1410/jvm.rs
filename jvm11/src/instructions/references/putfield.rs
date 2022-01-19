#[allow(non_camel_case_types)]
pub struct PUT_FIELD {
    index: u16
}

impl PUT_FIELD {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for PUT_FIELD {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16()
    }
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let thread_guard = thread.read().unwrap();
        let current_frame = thread_guard.current_frame();
        let mut frame = current_frame.borrow_mut();
        let cp = frame.method.constant_pool();

        //let current_class = cp.read().unwrap().get_class();

        //得到字段符号引用
        let mut cp_name = cp.write().unwrap();
        let field_ref = cp_name.get_constant_mut(self.index as usize).get_field_ref_mut();

        let field = field_ref.resolve_field();

        //let ref_class = field.read().unwrap().class.clone();

        let field_guard = field.read().unwrap();
        if field_guard.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        // if field.read().unwrap().is_final() {
        //     //如果是final 字段，则只能自己改或 init方法中改
        //     if current_class.read().unwrap().name.as_str() != field.read().unwrap().name.as_str()
        //         || frame.method.name.as_str() != "<init>" {
        //         panic!("java.lang.IllegalAccessError");
        //     }
        // }

        let descriptor = field_guard.descriptor.clone();
        let slot_id = field_guard.slot_id;

        let stack = frame.operand_stack();


        match descriptor.chars().next() {
            Some(c) => match c {
                'Z' | 'B' | 'C' | 'S' | 'I' => {
                    let val = stack.pop_int();
                    let slot = stack.pop_slot();
                    //这里可能弹出的是空
                    match slot {
                        Slot::Nil() => panic!(NULL_POINTER_EXCEPTION),
                        Slot::Ref(obj_ref) => {
                            obj_ref.write().unwrap().fields.set_int(slot_id, val);
                        }
                        _ => panic!("put field error")
                    }
                }
                'F' => {
                    let val = stack.pop_float();
                    let slot = stack.pop_slot();
                    //这里可能弹出的是空
                    match slot {
                        Slot::Nil() => panic!(NULL_POINTER_EXCEPTION),
                        Slot::Ref(obj_ref) => {
                            obj_ref.write().unwrap().fields.set_float(slot_id, val);
                        }
                        _ => panic!("put field error")
                    }
                }
                'J' => {
                    let val = stack.pop_long();
                    let slot = stack.pop_slot();
                    //这里可能弹出的是空
                    match slot {
                        Slot::Nil() => panic!(NULL_POINTER_EXCEPTION),
                        Slot::Ref(obj_ref) => {
                            obj_ref.write().unwrap().fields.set_long(slot_id, val);
                        }
                        _ => panic!("put field error")
                    }
                }
                'D' => {
                    let val = stack.pop_double();
                    let slot = stack.pop_slot();
                    //这里可能弹出的是空
                    match slot {
                        Slot::Nil() => panic!(NULL_POINTER_EXCEPTION),
                        Slot::Ref(obj_ref) => {
                            obj_ref.write().unwrap().fields.set_double(slot_id, val);
                        }
                        _ => panic!("put field error")
                    }
                }
                'L' | '[' => {
                    let val = stack.pop_slot();
                    let slot = stack.pop_slot();
                    //这里可能弹出的是空
                    match slot {
                        Slot::Nil() => panic!(NULL_POINTER_EXCEPTION),
                        Slot::Ref(obj_ref) => {
                            obj_ref.write().unwrap().fields.set_slot(slot_id, val);
                        }
                        _ => panic!("put field error")
                    }
                }
                _ => panic!("impossible")
            },
            None => panic!("impossible")
        }
    }
}

impl Debug for PUT_FIELD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(PUT_FIELD)")
    }
}