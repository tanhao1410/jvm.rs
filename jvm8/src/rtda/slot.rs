use crate::rtda::heap::object::Object;
use std::sync::{RwLock, Arc};


pub struct Slots{
    pub(crate) slots : Vec<Slot>
}

impl Default for Slots {
    fn default() -> Self {
        Self { slots: vec![] }
    }
}

impl Slots {

    pub fn new(len:usize)->Self{
        Self{
            slots:vec![Slot::Nil();len]
        }
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        self.slots[index] = Slot::Num(val as u32);
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match self.slots[index] {
            Slot::Num(int) => int as i32,
            _ => panic!("error local var operation for get i32."),
        }
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        self.slots[index] = Slot::Num(f32::to_bits(val));
    }

    pub fn get_float(&self, index: usize) -> f32 {
        match self.slots[index] {
            Slot::Num(int) => f32::from_bits(int),
            _ => panic!("error local var operation for get f32."),
        }
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
        let low = val as u32;
        let high = (val as u64 >> 32) as u32;

        self.slots[index] = Slot::Num(low);
        self.slots[index + 1] = Slot::Num(high);
    }


    fn get_u32(&self, index: usize) -> u32 {
        match self.slots[index] {
            Slot::Num(int) => int,
            _ => panic!("error local var operation for get i32."),
        }
    }

    pub fn get_long(&self, index: usize) -> i64 {
        let low = self.get_u32(index) as u64;
        let high = self.get_u32(index + 1) as u64;
        ((high << 32) | low) as i64
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        let n = f64::to_bits(val) as i64;
        self.set_long(index, n);
    }

    pub fn get_double(&self, index: usize) -> f64 {
        let n = self.get_long(index) as u64;
        f64::from_bits(n)
    }

    // todo 需要处理，这里放的不一定都是引用，也可能是空
    pub fn set_ref(&mut self, index: usize, reference: Arc<RwLock<Object>>) {
        self.slots[index] = Slot::Ref(reference)
    }

    pub fn get_ref(&self, index: usize) -> Arc<RwLock<Object>> {
        match &self.slots[index] {
            Slot::Ref(obj) => obj.clone(),
            _ => panic!("error local var operation for get ref."),
        }
    }

    pub fn set_slot(&mut self, index: usize, slot: Slot) {
        self.slots[index] = slot;
    }

    pub fn get_slot(& self, index: usize) -> Slot {
        self.slots[index].clone()
    }

}

/// 代表操作数栈或局部变量表中的 数，要么是一个数，要么是一个引用
pub enum Slot{
    Nil(),
    Num(u32),
    //Ref(Object)
    Ref(Arc<RwLock<Object>>)
}

impl Clone for Slot{
    fn clone(&self) -> Self {
        match self {
            Slot::Nil()=>Slot::Nil(),
            Slot::Num(n) => Slot::Num(*n),
            Slot::Ref(rc_obj) => Slot::Ref(rc_obj.clone()),
        }
    }
}