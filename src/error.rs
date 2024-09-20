 



 pub trait ErrorCode {
    fn code(&self) -> u16;
    fn message(&self) -> &'static str;
 }