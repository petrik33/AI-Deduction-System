pub trait Property {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    // fn get_value(&self) -> &T;
}