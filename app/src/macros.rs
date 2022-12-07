#[macro_export]
macro_rules! async_closure {
    (|$arg:ident| $code:block ) => {
        |$arg| async move { $code }.boxed()
    };
    (move |$arg:ident| $code:block ) => {
        move |$arg| async move { $code }.boxed()
    };
}
