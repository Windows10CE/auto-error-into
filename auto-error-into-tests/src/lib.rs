#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(test)]
mod tests {
    use auto_error_into::auto_error_into;

    struct S(i32);

    #[auto_error_into(force_inline)]
    fn test1(S(i): S) -> Result<i32, i8> {
        println!("test");
        Err(3)
    }
    fn test1_use() {
        let _: i32 = test1(S(2));
    }

    #[auto_error_into]
    fn test2() -> Result<u32, u8> {
        println!("test");
        Err(3)
    }
    fn test2_use() {
        let _: u32 = test2();
    }
}
