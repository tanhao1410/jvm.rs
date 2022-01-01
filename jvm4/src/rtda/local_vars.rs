use crate::rtda::slot::Slot;
use crate::rtda::object::Object;
use std::sync::{RwLock, Arc};

/// 栈帧中的局部变量表
pub struct LocalVars(Vec<Slot>);


impl LocalVars{
    pub fn new(size:usize)->Self{
        LocalVars(vec![Slot::Nil();size])
    }

    pub fn set_i32(&mut self,index:usize,val:i32){
        let slots = &mut self.0;
        slots[index] = Slot::Num(val as u32);
    }

    pub fn get_i32(&self,index:usize)->i32{
        let slots = &self.0;
        let slot = &slots[index];
        match slot {
            Slot::Num(num)=>*num as i32,
            _=>panic!("error")
        }
    }

    pub fn set_f32(&mut self,index:usize,val:f32){
        let slots = &mut self.0;
        slots[index] = Slot::Num(val.to_bits())
    }

    pub fn get_f32(&self,index:usize)->f32{
        let slots = &self.0;
        let slot = &slots[index];
        match slot {
            Slot::Num(num)=>f32::from_bits(*num),
            _=>panic!("error")
        }
    }

    pub fn set_i64(&mut self,index:usize,val:i64){
        let slots = &mut self.0;
        slots[index] = Slot::Num((val >> 32) as u32);
        slots[index +1] = Slot::Num(val as u32);
    }

    pub fn get_i64(&self,index:usize)->i64{
        let slots = &self.0;
        let slot = (&slots[index],&slots[index+1]);
        match slot {
            (Slot::Num(big),Slot::Num(low))=> ((*big as i64) << 32) +(*low as i64),
            _=>panic!("error")
        }
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

    pub fn set_ref(&mut self,index:usize,val:Arc<RwLock<Object>>){
        let slots = &mut self.0;
        slots[index] = Slot::Ref(val.clone());
    }

    pub fn get_ref(&self,index:usize)->Arc<RwLock<Object>>{
        let slots = &self.0;
        let slot = &slots[index];
        match slot {
            Slot::Ref(num)=>num.clone(),
            _=>panic!("error")
        }
    }

}