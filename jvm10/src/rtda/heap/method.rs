use std::sync::{Arc, RwLock};

use crate::classfile::member_info::MemberInfo;
use crate::rtda::heap::access_flags::AccessFlags;
use crate::rtda::heap::class::Class;
use crate::rtda::heap::constant_pool::constant_pool::ConstantPool;
use crate::rtda::heap::exception_table::ExceptionTable;
use crate::rtda::heap::object::Object;
use crate::classfile::attribute_info::line_num_table_attribute::{LineNumTableEntry};

pub struct Method {
    access_flags: u16,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    pub class: Arc<RwLock<Class>>,
    pub max_stack: usize,
    pub max_locals: usize,
    pub code: Arc<Vec<u8>>,

    //方法参数的槽数
    pub arg_slot_count: usize,

    //添加异常处理表
    pub exception_table: ExceptionTable,

    pub line_number_table: Arc<Vec<LineNumTableEntry>>,
}

#[allow(dead_code)]
impl Method {
    fn new(class: Arc<RwLock<Class>>, meminfo: &MemberInfo, pool: Arc<RwLock<ConstantPool>>) -> Arc<Method> {
        let code_attr = meminfo.code_attribute();
        if let Some(code_attr) = code_attr {
            Arc::new(Self {
                access_flags: meminfo.access_flgs(),
                name: meminfo.name(),
                descriptor: meminfo.descriptor(),
                class,
                max_stack: code_attr.max_stack() as usize,
                max_locals: code_attr.max_locals() as usize,
                code: code_attr.code().clone(),
                arg_slot_count: Method::calc_arg_slot_count(meminfo.descriptor().as_str(), meminfo.access_flgs()),
                exception_table: ExceptionTable::new(code_attr.exception_table(), pool),
                line_number_table: code_attr.line_number_table_attribute(),
            })
        } else {
            //codeattr不存在，即native方法

            //根据本地方法的返回值类型来判断在最后面加上什么样的返回

            let code = match meminfo.descriptor().as_bytes().last().unwrap() {
                b'V' => vec![0xfeu8, 0xb1],//return
                b'D' => vec![0xfeu8, 0xaf],//dreturn
                b'F' => vec![0xfeu8, 0xae],//freturn
                b'J' => vec![0xfeu8, 0xad],//lreturn
                b';' => vec![0xfeu8, 0xb0],//areturn
                _ => vec![0xfeu8, 0xac],//ireturn
            };

            //本地方法需要填充一些东西
            let arg_slot_count = Method::calc_arg_slot_count(meminfo.descriptor().as_str(), meminfo.access_flgs());

            Arc::new(Self {
                access_flags: meminfo.access_flgs(),
                name: meminfo.name(),
                descriptor: meminfo.descriptor(),
                class,
                max_stack: 4,
                max_locals: arg_slot_count,
                code: Arc::new(code),
                arg_slot_count,
                exception_table: Default::default(),
                line_number_table: Arc::new(vec![]),
            })
        }
    }

    pub fn new_methods(class: Arc<RwLock<Class>>, meminfos: &Vec<MemberInfo>, pool: Arc<RwLock<ConstantPool>>) -> Vec<Arc<Method>> {
        meminfos.iter().map(|mem| {
            Self::new(class.clone(), mem, pool.clone())
        }).collect()
    }

    /// 在方法中找异常处理类的 catch块 起始位置
    pub fn find_exception_handler(&self, ex_obj: Arc<RwLock<Object>>, pc: i32) -> Option<i32> {
        let ex_class = ex_obj.read().unwrap().class.clone();
        match self.exception_table.find_exception_handler(ex_class, pc) {
            None => None,
            Some(handler) => Some(handler.handler_pc as i32)
        }
    }

    pub fn get_line_number(&self, pc: i32) -> i32 {
        if self.is_native() {
            -2
        } else if self.line_number_table.len() == 0 {
            -1
        } else {
            //从后往前遍历，遇到pc 大于它的start的说名找到了
            for line_entry in self.line_number_table.iter().rev() {
                if pc >= line_entry.start_pc as i32 {
                    return line_entry.line_num as i32;
                }
            }
            -1
        }
    }

    pub fn is_static(&self) -> bool {
        AccessFlags::is_static(self.access_flags)
    }
    pub fn is_private(&self) -> bool {
        AccessFlags::is_private(self.access_flags)
    }
    pub fn is_protected(&self) -> bool {
        AccessFlags::is_protected(self.access_flags)
    }
    pub fn is_public(&self) -> bool { AccessFlags::is_public(self.access_flags) }
    pub fn is_synthetic(&self) -> bool {
        AccessFlags::is_synthetic(self.access_flags)
    }
    pub fn is_final(&self) -> bool { AccessFlags::is_final(self.access_flags) }
    pub fn is_abstract(&self) -> bool {
        AccessFlags::is_abstract(self.access_flags)
    }
    pub fn is_native(&self) -> bool { AccessFlags::is_native(self.access_flags) }

    pub fn constant_pool(&self) -> Arc<RwLock<crate::rtda::heap::constant_pool::constant_pool::ConstantPool>> {
        self.class.read().unwrap().constant_pool.as_ref().unwrap().clone()
    }

    /// 对某类是否可访问
    pub fn is_accessible_to(&self, d: Arc<RwLock<Class>>) -> bool {
        if self.is_public() {
            true
        } else {
            let cur_class = self.class.clone();
            if self.is_protected() {
                return d.read().unwrap().name.as_str() == cur_class.read().unwrap().name.as_str() // 同一个类
                    || d.read().unwrap().is_sub_class_of(cur_class.clone()) //当前类是传入类的父类
                    || cur_class.read().unwrap().package_name().eq(d.read().unwrap().package_name());
            } else if !self.is_private() {
                return cur_class.read().unwrap().package_name().eq(d.read().unwrap().package_name());
            }
            self.class.read().unwrap().name.as_str() == d.read().unwrap().name.as_str()
        }
    }

    /// 计算方法参数 需要槽的个数
    fn calc_arg_slot_count(descriptor: &str, access_flag: u16) -> usize {
        let mut arg_slot_count = 0usize;

        let bytes = descriptor.as_bytes();
        let mut end = 1;
        while bytes[end] != b')' {
            end += 1;
        }

        if end > 1 {
            //1.参数 都在两个括号之中
            arg_slot_count += Self::calc_arg_num(&bytes[1..end]);
            //[Ljava.lang.String;)
        }
        //不是静态方法，额外+1，this的存在
        if !AccessFlags::is_static(access_flag) {
            arg_slot_count += 1;
        }
        arg_slot_count
    }


    fn calc_arg_num(desc: &[u8]) -> usize {
        if desc.len() == 0 {
            return 0;
        }
        //先读取一个字母
        match desc[0] as char {
            'B' | 'C' | 'F' | 'I' | 'S' | 'Z' => 1 + Self::calc_arg_num(&desc[1..]),
            '[' => {
                let mut start = 1;
                if desc[1] == b'L' {
                    while desc[start] != b';' {
                        start += 1;
                    }
                }
                if start + 1 > desc.len() {
                    return 1;
                }
                1 + Self::calc_arg_num(&desc[start + 1..])
            }
            'L' => {
                //读到下一个分号
                let mut start = 1;
                while desc[start] != b';' {
                    start += 1;
                }
                if start + 1 > desc.len() {
                    return 1;
                }
                1 + Self::calc_arg_num(&desc[start + 1..])
            }
            'J' | 'D' => {
                2 + Self::calc_arg_num(&desc[1..]) //todo
            }
            _ => return 0
        }
    }
}