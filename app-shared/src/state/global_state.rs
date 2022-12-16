use std::cell::RefCell;
use std::fmt::Debug;

use parking_lot::ReentrantMutex;

pub trait GlobalState
where
    Self: Sized + 'static + Debug,
{
    fn get_static() -> &'static ReentrantMutex<RefCell<Option<Self>>>;
}

pub trait GlobalStateClone: GlobalState
where
    Self: Clone,
{
    fn clone_state() -> Self {
        let var = Self::get_static();
        let lock = var.lock();

        lock.clone().take().unwrap()
    }
}

pub trait GlobalStateSet: GlobalState {
    fn set_state(value: Self) {
        let var = Self::get_static();
        let lock = var.lock();

        *lock.borrow_mut() = Some(value);
    }
}

pub trait GlobalStateLock: GlobalState {
    fn lock<F, O>(f: F) -> O
    where
        F: Send + FnOnce(&mut Self) -> O,
        Self: Sync + Send,
    {
        let var = Self::get_static();
        let lock = var.lock();
        let mut state = lock.borrow_mut();

        f(state.as_mut().unwrap())
    }
}
