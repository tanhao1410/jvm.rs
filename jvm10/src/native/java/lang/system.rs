use crate::native::registry::register;
use crate::rtda::local_vars::LocalVars;
use crate::rtda::slot::{Slots, Slot};
use crate::rtda::heap::array_datas::ArrayDatas;
use crate::constants::java_exception::NULL_POINTER_EXCEPTION;
use std::sync::RwLockWriteGuard;
use crate::rtda::thread::Thread;

pub fn init() {
    register("java/lang/System",
             "arraycopy",
             "(Ljava/lang/Object;ILjava/lang/Object;II)V"
             , arraycopy);
}

fn arraycopy(local_vars: &LocalVars,_:RwLockWriteGuard<Thread>) -> Option<Slots> {
    let src = local_vars.get_slot(0);
    let src_pos = local_vars.get_int(1) as usize;
    let dest = local_vars.get_slot(2);
    let dest_pos = local_vars.get_int(3) as usize;
    let length = local_vars.get_int(4) as usize;

    //验证
    let src = match src {
        Slot::Ref(obj) => obj,
        _ => panic!(NULL_POINTER_EXCEPTION)
    };
    let dest = match dest {
        Slot::Ref(obj) => obj,
        _ => panic!(NULL_POINTER_EXCEPTION)
    };

    //todo 其它验证,类型一致性，边界大小验证等

    //从src处读取数据，写入到目标中
    let src_guard = src.read().unwrap();

    let mut dest_guard = dest.write().unwrap();
    //判断类型
    match &src_guard.datas {
        ArrayDatas::Bytes(src) => {
            match &mut dest_guard.datas {
                ArrayDatas::Bytes(dest) => {
                    for i in 0..length {
                        dest[i + dest_pos] = src[i + src_pos];
                    }
                }
                _ => panic!("error")
            }
        }
        ArrayDatas::Shorts(src) => {
            match &mut dest_guard.datas {
                ArrayDatas::Shorts(dest) => {
                    for i in 0..length {
                        dest[i + dest_pos] = src[i + src_pos];
                    }
                }
                _ => panic!("error")
            }
        }
        ArrayDatas::Ints(src) => {
            match &mut dest_guard.datas {
                ArrayDatas::Ints(dest) => {
                    for i in 0..length {
                        dest[i + dest_pos] = src[i + src_pos];
                    }
                }
                _ => panic!("error")
            }
        }
        ArrayDatas::Longs(src) => {
            match &mut dest_guard.datas {
                ArrayDatas::Longs(dest) => {
                    for i in 0..length {
                        dest[i + dest_pos] = src[i + src_pos];
                    }
                }
                _ => panic!("error")
            }
        }
        ArrayDatas::Chars(src) => {
            match &mut dest_guard.datas {
                ArrayDatas::Chars(dest) => {
                    for i in 0..length {
                        dest[i + dest_pos] = src[i + src_pos];
                    }
                }
                _ => panic!("error")
            }
        }
        ArrayDatas::Floats(src) => {
            match &mut dest_guard.datas {
                ArrayDatas::Floats(dest) => {
                    for i in 0..length {
                        dest[i + dest_pos] = src[i + src_pos];
                    }
                }
                _ => panic!("error")
            }
        }
        ArrayDatas::Doubles(src) => {
            match &mut dest_guard.datas {
                ArrayDatas::Doubles(dest) => {
                    for i in 0..length {
                        dest[i + dest_pos] = src[i + src_pos];
                    }
                }
                _ => panic!("error")
            }
        }
        ArrayDatas::Refs(Slots{slots}) => {
            match &mut dest_guard.datas {
                ArrayDatas::Refs(dest) => {
                    let dest = &mut dest.slots;
                    for i in 0..length {
                        dest[i + dest_pos] = slots[i + src_pos].clone();
                    }
                }
                _ => panic!("error")
            }
        }
        _ => panic!("error")
    }
    None
}