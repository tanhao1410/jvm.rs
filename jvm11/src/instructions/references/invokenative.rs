#[allow(non_camel_case_types)]
pub struct INVOKE_NATIVE {}


impl Instruction for INVOKE_NATIVE {
    fn execute(&mut self, thread: Arc<RwLock<Thread>>) {
        let arc = thread.clone();
        let guard = arc.write().unwrap();
        let frame = guard.current_frame();

        let frame2 = frame.clone();
        let mut frame2_ref = frame2.borrow_mut();
        let local_vars = frame2_ref.local_vars();
        //根据类名、方法名和描述符从本地方法注册表中查找本地方法实现，
        let method = frame2_ref.method.clone();
        let method_class = method.class.clone();
        let method_class_guard = method_class.read().unwrap();
        let class_name = method_class_guard.name.clone();
        let method_name = method.name.clone();
        let method_desc = method.descriptor.clone();

        //先明白调用本地方法后的结果是什么，把结果推到当前方法栈的栈顶。
        //在现在，本地方法栈与虚拟机栈用的是同一个。
        // 思路1:不传入frame，而是传入参数列表与操作数栈。因为操作数栈需要可变借用不管操作数栈。
        //思路3：重写本地方法的 局部变量表与操作数栈等。传入到本地方法中，最后从本地方法操作数栈弹出结果，
        //思路4：完全重写本地方法帧，不需要专门增加return 指令。在执行完本指令后，直接由本指令完成所有后续操作。

        //思路2：注册的本地方法采用返回值的方式，将结果传递过来，在本条指令之后，将结果推到栈顶。 先采用这种方式吧
        //返回值用slot的话，对于dubble类型与long类型无法做到，返回slots。


        if let Some(func) = find_native_method(class_name.as_str(),
                                               method_name.as_str(),
                                               method_desc.as_str()) {
            let res = func(local_vars,guard);
            if let Some(res) = res {
                let len = res.slots.len();
                for index in 0..len {
                    frame2_ref.operand_stack().push_slot(res.get_slot(index))
                }
            }
        } else if "initialize" == method_name.as_str() && JAVA_CLASS_NAME_VM == class_name.as_str() {
            //为了实现自动拆箱装箱的逻辑
            let vm_class = ClassLoader::load_class(ClassLoader::get_system_class_loader(),
                                                   JAVA_CLASS_NAME_VM);
            let vm_class_guard = vm_class.read().unwrap();

            let saved_props = vm_class_guard.get_field("savedProps", "Ljava/util/Properties;");

            let saved_props_guard = saved_props.read().unwrap();

            let saved_props=  match & vm_class_guard.static_vars.slots[saved_props_guard.slot_id]{
                Slot::Ref(val) => {
                    val.clone()
                }
                _=>unreachable!()
            };


            let key = get_java_string("foo");
            let val = get_java_string("bar");

            let stack = frame2_ref.operand_stack();
            stack.push_ref(saved_props);
            stack.push_ref(key);
            stack.push_ref(val);

            let props_class = ClassLoader::load_class(ClassLoader::get_system_class_loader(),
                                                      JAVA_CLASS_NAME_PROPERTIES);

            let props_class = props_class.read().unwrap();
            let set_prop_method = props_class.get_instance_method("setProperty",
                                                                  "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;").unwrap();

            invoke_method(guard, set_prop_method, thread, frame2_ref);
        } else {
            panic!("java.lang.UnstatifiedLinkError")
        }
    }
}

impl Debug for INVOKE_NATIVE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(INVOKE_NATIVE)")
    }
}