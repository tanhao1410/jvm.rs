#[allow(non_camel_case_types)]
pub struct NEW_ARRAY {
    index: u8
}

// 根据第一个操作数的不同，创建不同的基本类型数组
const AT_BOOLEAN: u8 = 4;
const AT_CHAR: u8 = 5;
const AT_FLOAT: u8 = 6;
const AT_DOUBLE: u8 = 7;
const AT_BYTE: u8 = 8;
const AT_SHORT: u8 = 9;
const AT_INT: u8 = 10;
const AT_LONG: u8 = 11;

impl NEW_ARRAY {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for NEW_ARRAY {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u8()
    }
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.write().unwrap();
        let rc = guard.current_frame();
        let mut frame = rc.borrow_mut();

        let operand_stack = frame.operand_stack();
        let count = operand_stack.pop_int();
        if count < 0 {
            panic!("java.lang.NegativeArraySizeException");
        }

        let class_loader = frame.method.class.read().unwrap().class_loader();
        //let arr_class = ClassLoader::get_primitive_array_class(class_loader,name:&str);

        let arr_class = match self.index {
            AT_BOOLEAN =>
                ClassLoader::load_class(class_loader, "[Z"),
            AT_CHAR =>
                ClassLoader::load_class(class_loader, "[C"),
            AT_FLOAT =>
                ClassLoader::load_class(class_loader, "[F"),
            AT_DOUBLE =>
                ClassLoader::load_class(class_loader, "[D"),
            AT_BYTE =>
                ClassLoader::load_class(class_loader, "[B"),
            AT_SHORT =>
                ClassLoader::load_class(class_loader, "[S"),
            AT_INT =>
                ClassLoader::load_class(class_loader, "[I"),
            AT_LONG =>
                ClassLoader::load_class(class_loader, "[J"),
            _ => unreachable!()
        };

        let arr_obj = Class::new_array(arr_class, count as usize);
        frame.operand_stack().push_ref(arr_obj);
    }
}

impl Debug for NEW_ARRAY {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.index)
    }
}