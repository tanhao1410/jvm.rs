
pub const JAVA_PRIMITIVE_CLASS_INT: &str = "int";
pub const JAVA_PRIMITIVE_CLASS_FLOAT: &str = "float";
pub const JAVA_PRIMITIVE_CLASS_DOUBLE: &str = "double";
pub const JAVA_PRIMITIVE_CLASS_VOID: &str = "void";
pub const JAVA_PRIMITIVE_CLASS_BOOLEAN: &str = "boolean";
pub const JAVA_PRIMITIVE_CLASS_SHORT: &str = "short";
pub const JAVA_PRIMITIVE_CLASS_BYTE: &str = "byte";
pub const JAVA_PRIMITIVE_CLASS_LONG: &str = "long";
pub const JAVA_PRIMITIVE_CLASS_CHAR: &str = "char";


pub fn get_java_primitive_class_names() -> Vec<&'static str> {
    vec![
         JAVA_PRIMITIVE_CLASS_FLOAT,
         JAVA_PRIMITIVE_CLASS_DOUBLE,
         JAVA_PRIMITIVE_CLASS_VOID,
         JAVA_PRIMITIVE_CLASS_BOOLEAN,
         JAVA_PRIMITIVE_CLASS_SHORT,
         JAVA_PRIMITIVE_CLASS_BYTE,
         JAVA_PRIMITIVE_CLASS_LONG,
         JAVA_PRIMITIVE_CLASS_CHAR,
         JAVA_PRIMITIVE_CLASS_INT]
}
