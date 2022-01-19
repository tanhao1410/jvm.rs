mod object;
mod class;
mod system;
mod float;
mod double;
mod string;
mod throwable;

pub fn init(){
    object::init();
    class::init();
    system::init();
    float::init();
    double::init();
    string::init();
    throwable::init();
}
