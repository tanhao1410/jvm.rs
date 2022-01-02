use crate::rtda::slot::Slot;
use crate::rtda::object::Object;
use std::sync::{Arc, RwLock};

/// 栈帧中的操作数栈
pub struct OperandStack {
    // 栈顶
    size: usize,

    slots: Vec<Slot>,
}

impl OperandStack {

    pub fn print_stack(&self){
        print!("操作数栈：(");
        for var in &self.slots{
            match  var{
                Slot::Nil()=>print!("nil "),
                Slot::Num(num)=>print!("{},",num),
                _=>{}
            }
        }
        println!(")");
    }

    pub fn new(max_stack: usize) -> Self {
        OperandStack { size: 0, slots: Vec::with_capacity(max_stack) }
    }

    pub fn push_i32(&mut self, val: i32) {
        self.slots.push(Slot::Num(val as u32));
        self.size += 1;
    }

    pub fn push_int(&mut self, val: i32) {
        self.push_i32(val);
    }

    pub fn push_slot(&mut self, slot: Slot) {
        self.slots.push(slot);
        self.size += 1;
    }

    pub fn pop_slot(&mut self) -> Slot {
        self.size -= 1;
        match self.slots.pop() {
            Some(slot) => slot,
            _ => panic! {"error"}
        }
    }

    pub fn pop_i32(&mut self) -> i32 {
        if self.size == 0 {
            panic!("error");
        }
        self.size -= 1;
        match self.slots.pop() {
            Some(Slot::Num(num)) => num as i32,
            _ => panic!("error")
        }
    }

    pub fn pop_int(&mut self) -> i32 {
        self.pop_i32()
    }

    pub fn push_i64(&mut self, val: i64) {
        self.slots.push(Slot::Num((val >> 32) as u32));
        self.slots.push(Slot::Num(val as u32));
        self.size += 2;
    }

    pub fn push_long(&mut self, val: i64) {
        self.push_i64(val);
    }

    pub fn pop_i64(&mut self) -> i64 {
        self.size -= 2;
        match self.slots.pop() {
            Some(Slot::Num(big)) => {
                match self.slots.pop() {
                    Some(Slot::Num(low)) => {
                        ((low as i64) << 32) + big as i64
                    }
                    _ => panic!("error")
                }
            }
            _ => panic!("error")
        }
    }

    pub fn pop_long(&mut self) -> i64 {
        self.pop_i64()
    }

    pub fn push_f32(&mut self, val: f32) {
        self.slots.push(Slot::Num(val.to_bits()));
        self.size += 1;
    }

    pub fn push_float(&mut self,val:f32){
        self.push_f32(val);
    }

    pub fn pop_f32(&mut self) -> f32 {
        self.size -= 1;
        match self.slots.pop() {
            Some(Slot::Num(num)) => f32::from_bits(num),
            _ => panic!("error")
        }
    }

    pub fn pop_float(&mut self)->f32{
        self.pop_f32()
    }

    pub fn push_f64(&mut self, val: f64) {
        let bytes = val.to_be_bytes();
        let (mut big, mut low) = ([0u8; 4], [0u8; 4]);
        for i in 0..4 {
            big[i] = bytes[i];
            low[i] = bytes[i + 4];
        }
        self.slots.push(Slot::Num(u32::from_be_bytes(low)));
        self.slots.push(Slot::Num(u32::from_be_bytes(big)));
        self.size += 2;
    }

    pub fn push_double(&mut self,val:f64){
        self.push_f64(val)
    }

    pub fn pop_f64(&mut self) -> f64 {
        self.size -= 2;
        match self.slots.pop() {
            Some(Slot::Num(big)) => {
                match self.slots.pop() {
                    Some(Slot::Num(low)) => {
                        let mut bytes = [0u8; 8];
                        let big = big.to_be_bytes();
                        let low = low.to_be_bytes();
                        for i in 0..4 {
                            bytes[i] = big[i];
                            bytes[i + 4] = low[i];
                        }
                        f64::from_be_bytes(bytes)
                    }
                    _ => panic!("error")
                }
            }
            _ => panic!("error")
        }
    }

    pub fn pop_double(&mut self)->f64{
        self.pop_f64()
    }

    pub fn push_ref(&mut self, val: Arc<RwLock<Object>>) {
        self.slots.push(Slot::Ref(val.clone()));
        self.size += 1;
    }

    pub fn push_nil(&mut self) {
        self.slots.push(Slot::Nil());
        self.size += 1;
    }

    pub fn pop_ref(&mut self) -> Arc<RwLock<Object>> {
        self.size -= 1;
        match self.slots.pop() {
            Some(Slot::Ref(obj)) => obj.clone(),
            _ => panic!("error")
        }
    }
}