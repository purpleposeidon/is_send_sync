extern crate is_send_sync;

use is_send_sync::{is_send_sync, is_send, is_sync};

#[cfg(test)]
#[test]
fn check() {
    const T: (bool, bool) = is_send_sync!(String);
    assert_eq!(T, (true, true));

    assert_eq!((true, true), is_send_sync!(String));
    assert_eq!((false, false), is_send_sync!(*const u8));
    assert_eq!((false, true), is_send_sync!(std::sync::RwLockReadGuard::<'static, i8>));

    assert_eq!(true, is_send!(String));
    assert_eq!(false, is_send!(*const u8));
    assert_eq!(false, is_send!(std::sync::RwLockReadGuard::<'static, i8>));

    assert_eq!(true, is_sync!(String));
    assert_eq!(false, is_sync!(*const u8));
    assert_eq!(true, is_sync!(std::sync::RwLockReadGuard::<'static, i8>));
}

