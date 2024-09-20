 


/// Trait definable on enun to define error codes and their messages.
pub trait ErrorCode {
    fn code(&self) -> u16;
    fn message(&self) -> &'static str;
 }