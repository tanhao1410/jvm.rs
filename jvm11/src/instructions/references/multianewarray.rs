#[allow(non_camel_case_types)]
pub struct MULTI_ANEW_ARRAY {
    index: u16,
    dimensions: u8,
}


impl MULTI_ANEW_ARRAY {
    pub fn new() -> Self {
        Self { index: 0, dimensions: 0 }
    }
}

impl Instruction for MULTI_ANEW_ARRAY {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
        self.dimensions = _reader.read_u8();
    }
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();

        let guard = arc.write().unwrap();
        let rc = guard.current_frame();

        let mut frame = rc.borrow_mut();
        let cp = frame.method.constant_pool();

        let mut cp_guard = cp.write().unwrap();
        let class_ref = cp_guard.get_constant_mut(self.index as usize).get_class_ref_mut();
        let arr_class = class_ref.resolve_class();

        let operand_stack = frame.operand_stack();

        //弹出维度，并检查合法性
        let mut dimensions = vec![0; self.dimensions as usize];
        for i in 0..self.dimensions {
            let dimension = operand_stack.pop_int();
            if dimension < 0 {
                panic!("java.lang.NegativeArraySizeException")
            }
            dimensions[self.dimensions as usize - 1 - i as usize] = dimension as usize;
        }

        //创建多维数组并推入栈中
        //let arr_obj = Class::new_array(arr_class, self.dimensions as usize);
        let arr_obj = new_multi_dimension_array(&dimensions,arr_class);
        operand_stack.push_ref(arr_obj);
    }
}


fn new_multi_dimension_array(counts: &[usize], arr_class: Arc<RwLock<Class>>) -> Arc<RwLock<Object>> {
    //本轮创建数组的大小
    let count = counts[0];
    //Class的new_array，new的是元素时arr_class 的数组！
    let arr_obj = Class::new_array(arr_class.clone(), count);

    let guard = arr_class.read().unwrap();
    let class_name = guard.name.clone();
    let class_loader = guard.loader.clone();
    if counts.len() > 1 {
        //即还有下一个维度
        //数组里的元素
        let mut obj_guard = arr_obj.write().unwrap();
        let slots =&mut  obj_guard.refs_mut().slots;
        for i in 0..count {
            //得到它的元素的类型
            let component_class = ClassLoader::load_class(class_loader.clone(),
                                                          get_component_class_name(class_name.clone()).as_str());
            let ele = new_multi_dimension_array(&counts[1..], component_class);
            slots[i] = Slot::Ref(ele);
        }
    }
    arr_obj
}

fn get_component_class_name(name: Arc<String>) -> Arc<String> {
    let component_type_desc = &name.as_bytes()[1..];

    match name.as_bytes()[0] {
        b'[' => {
            //数组内部还是数组，直接去除掉外面的【即可
            Arc::new(String::from_utf8_lossy(component_type_desc).to_string())
        }
        b'L' => {
            //数组内部是类，去掉，【以及后面;即可
            let len = component_type_desc.len();
            Arc::new(String::from_utf8_lossy(&component_type_desc[1..len - 1]).to_string())
        }
        b'V' => Arc::new("void".to_string()),
        b'Z' => Arc::new("boolean".to_string()),
        b'B' => Arc::new("byte".to_string()),
        b'S' => Arc::new("short".to_string()),
        b'I' => Arc::new("int".to_string()),
        b'J' => Arc::new("long".to_string()),
        b'C' => Arc::new("char".to_string()),
        b'F' => Arc::new("float".to_string()),
        b'D' => Arc::new("double".to_string()),
        _ => unreachable!()
    }
}

#[test]
fn test_get_component_class_name(){
    assert_eq!("[[Z",get_component_class_name(Arc::new("[[[Z".to_string())).as_str());
    assert_eq!("Z",get_component_class_name(Arc::new("[Z".to_string())).as_str());
}

impl Debug for MULTI_ANEW_ARRAY {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.index)
    }
}