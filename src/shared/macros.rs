#[macro_export]
macro_rules! cstr {
    ($str: expr) => { format!("{}\0", $str).as_ptr() as *const i8 }
}
