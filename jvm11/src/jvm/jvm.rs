use crate::cmd::CMD;
use std::sync::{Arc, RwLock};
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::thread::Thread;
use crate::classpath::classpath::ClassPath;
use crate::{native, interpreter};
use crate::rtda::heap::object::Object;
use crate::rtda::heap::string_pool::get_java_str_obj_by_pool;
use crate::rtda::slot::Slot;
use crate::constants::java_class_name::{JAVA_CLASS_NAME_STRING};
use crate::rtda::frame::Frame;

pub struct JVM {
    cmd: CMD,
    bootstrap_classloader: Arc<RwLock<ClassLoader>>,
    main_thread: Arc<RwLock<Thread>>,
}

impl JVM {
    /// 根据命令行参数创建虚拟机
    pub fn new(cmd: CMD) -> Self {
        let cp = ClassPath::parse(cmd.jre_option.clone(), cmd.cp_option.clone());
        let bootstrap_classloader = ClassLoader::new(cp);
        //创建一个线程
        let main_thread = Arc::new(RwLock::new(Thread::new()));
        Self { cmd, bootstrap_classloader, main_thread }
    }

    pub fn start(&self) {
        //注册本地方法
        native::init();
        //执行main方法
        self.exec_main();
    }

    fn exec_main(&self) {
        let cmd = &self.cmd;
        let bootstrap_classloader = &self.bootstrap_classloader;
        //启动类名
        let class_name = cmd.class.replace(".", "/");
        let main_class = ClassLoader::load_class(bootstrap_classloader.clone(), class_name);
        let main_method = main_class.read().unwrap().get_main_method();
        if let Some(main_method) = main_method {
            let args_array = Self::create_args_array(bootstrap_classloader.clone(), &cmd.args);
            {
                let frame = Frame::new(self.main_thread.clone(), main_method);
                let mut main_thread = self.main_thread.write().unwrap();
                main_thread.push_frame(frame);
                let current_frame = main_thread.current_frame();
                let mut frame_mut = current_frame.borrow_mut();
                //给main方法设置参数
                frame_mut.local_vars_mut().set_slot(0, Slot::Ref(args_array));
            }
            //测试解释器
            interpreter::interpret(self.main_thread.clone(), cmd.inst_log_flag);
        } else {
            println!("Main method not found in class {}", &cmd.class);
        }
    }

    fn create_args_array(loader: Arc<RwLock<ClassLoader>>, args: &Vec<String>) -> Arc<RwLock<Object>> {
        let string_class = ClassLoader::load_class(loader.clone(), JAVA_CLASS_NAME_STRING);
        let args_arr = Object::new_array(string_class, args.len());
        //需要把该数组中的元素替换成java中的字符串
        {
            let mut guard = args_arr.write().unwrap();
            let slots = &mut guard.refs_mut().slots;
            for i in 0..args.len() {
                let java_string = get_java_str_obj_by_pool(Arc::new(args[i].to_string()));
                slots[i] = Slot::Ref(java_string);
            }
        }
        args_arr
    }
}