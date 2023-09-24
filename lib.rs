//! Macros for determining if a type is `Send` and/or `Sync`. The macro outputs a const expression.

#[doc(hidden)]
pub trait ItIsnt {
    const IS_SEND: bool = false;
    const IS_SYNC: bool = false;
}
impl<X> ItIsnt for X {}

/// Equivalent to `(is_send!(ty), is_sync!(ty))`.
#[macro_export]
macro_rules! is_send_sync {
    ($ty:ty) => {{
        #[allow(unused_imports)]
        use $crate::ItIsnt;
        struct Shallow<T>(T);
        #[allow(dead_code)] impl<X: Send> Shallow<X> { const IS_SEND: bool = true; }
        #[allow(dead_code)] impl<X: Sync> Shallow<X> { const IS_SYNC: bool = true; }
        (Shallow::<$ty>::IS_SEND, Shallow::<$ty>::IS_SYNC)
    }};
}

/// Returns a const `true` if the type is `Send`.
#[macro_export]
macro_rules! is_send {
    ($ty:ty) => {{
        #[allow(unused_imports)]
        use $crate::ItIsnt;
        struct Shallow<T>(T);
        #[allow(dead_code)] impl<X: Send> Shallow<X> { const IS_SEND: bool = true; }
        Shallow::<$ty>::IS_SEND
    }};
}

/// Returns a const `true` if the type is `Sync`.
#[macro_export]
macro_rules! is_sync {
    ($ty:ty) => {{
        #[allow(unused_imports)]
        use $crate::ItIsnt;
        struct Shallow<T>(T);
        #[allow(dead_code)] impl<X: Sync> Shallow<X> { const IS_SYNC: bool = true; }
        Shallow::<$ty>::IS_SYNC
    }};
}
