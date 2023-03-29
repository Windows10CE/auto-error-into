#[doc(inline)]
pub use auto_error_into_macro::auto_error_into;

#[doc(hidden)]
pub mod __ {
    pub trait ResultResolver {
        type Ok;
        type Err;
    }

    impl<T, E> ResultResolver for Result<T, E> {
        type Ok = T;
        type Err = E;
    }
}
