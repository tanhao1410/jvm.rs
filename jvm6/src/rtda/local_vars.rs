use crate::rtda::slot::Slot;
use crate::rtda::heap::object::Object;
use std::sync::{RwLock, Arc};
use std::fmt::{Display, Formatter};

/// 栈帧中的局部变量表
pub struct LocalVars(Vec<Slot>);


impl LocalVars{

    pub fn print_vars(&self){
        print!("局部变量表：[");
        for var in self.0.iter(){

            match var{
                Slot::Nil()=>print!("nil "),
                Slot::Num(num)=>print!("{},",num),
                Slot::Ref(obj)=>{}
            }

        }
        println!("]");
    }

    pub fn new(size:usize)->Self{
        LocalVars(vec![Slot::Nil();size])
    }

    pub fn set_i32(&mut self,index:usize,val:i32){
        let slots = &mut self.0;
        slots[index] = Slot::Num(val as u32);
    }

    pub fn set_int(&mut self,index:usize,val:i32){
        self.set_i32(index,val);
    }

    pub fn get_i32(&self,index:usize)->i32{
        let slots = &self.0;
        let slot = &slots[index];
        match slot {
            Slot::Num(num)=>*num as i32,
            _=>panic!("error")
        }
    }

    pub fn get_int(&self,index:usize)->i32{
        self.get_i32(index)
    }

    pub fn set_f32(&mut self,index:usize,val:f32){
        let slots = &mut self.0;
        slots[index] = Slot::Num(val.to_bits())
    }

    pub fn set_float(&mut self,index:usize,val:f32){
        self.set_f32(index,val)
    }

    pub fn get_f32(&self,index:usize)->f32{
        let slots = &self.0;
        let slot = &slots[index];
        match slot {
            Slot::Num(num)=>f32::from_bits(*num),
            _=>panic!("error")
        }
    }

    pub fn get_float(&self,index:usize)->f32{
        self.get_f32(index)
    }

    pub fn set_i64(&mut self,index:usize,val:i64){
        let slots = &mut self.0;
        slots[index] = Slot::Num((val >> 32) as u32);
        slots[index +1] = Slot::Num(val as u32);
    }

    pub fn set_long(&mut self,index:usize,val:i64){
        self.set_i64(index,val)
    }

    pub fn get_i64(&self,index:usize)->i64{
        let slots = &self.0;
        let slot = (&slots[index],&slots[index+1]);
        match slot {
            (Slot::Num(big),Slot::Num(low))=> ((*big as i64) << 32) +(*low as i64),
            _=>panic!("error")
        }
    }

    pub fn get_long(&self,index:usize)->i64{
        self.get_i64(index)
    }

    pub fn set_f64(&mut self,index:usize,val:f64){
        let slots = &mut self.0;
        let bytes = val.to_be_bytes();
        let (mut big,mut low) = ([0u8;4],[0u8;4]);
        for i in 0..4{
            big[i] = bytes[i];
            low[i] = bytes[i + 4];
        }
        slots[index] = Slot::Num(u32::from_be_bytes(big));
        slots[index+1] = Slot::Num(u32::from_be_bytes(low));
    }

    pub fn set_double(&mut self,index:usize,val:f64){
        self.set_f64(index,val)
    }

    pub fn get_f64(&self,index:usize)->f64{
        let slots = &self.0;
        let slot = (&slots[index],&slots[index+1]);
        match slot {
            (Slot::Num(big),Slot::Num(low))=>{
                let mut bytes = [0u8;8];
                let big = big.to_be_bytes();
                let low = low.to_be_bytes();
                for i in 0..4{
                    bytes[i] = big[i];
                    bytes[i+4] = low[i];
                }
                f64::from_be_bytes(bytes)
            },
            _=>panic!("error")
        }
    }
    pub fn get_double(&self,index:usize)->f64{
        self.get_f64(index)
    }


    pub fn set_slot(&mut self,index:usize,slot:Slot){
        let slots = &mut self.0;
        slots[index] = slot;
    }

    pub fn get_slot(&self,index:usize)->&Slot{
        let slots = &self.0;
         &slots[index]
    }

}