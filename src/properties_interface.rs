pub trait Prop {
    fn get_name() -> &'static str;
    fn get_description(&self) -> &str;
}
