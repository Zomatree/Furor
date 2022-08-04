// please dont read this code

use futures::Future;
use std::{pin::Pin, cell::RefCell, marker::PhantomData};

// BoxFuture from futures requires Send however we are in a single threaded env so we dont want Send

pub type BoxFuture<O> = Pin<Box<dyn Future<Output=O> + 'static>>;
pub type Take<T> = RefCell<Option<T>>;
pub type TakenAsyncFunc<Args, O> = Take<Box<dyn FnOnce<Args, Output=BoxFuture<O>>>>;

struct Wrapper<Args, O, F, Fut>(F, PhantomData<(Args, O, Fut)>)
where
    Args: 'static,
    O: 'static,
    F: FnOnce<Args, Output=Fut> + 'static,
    Fut: Future<Output=O> + 'static;

impl<Args, O, F, Fut> FnOnce<Args> for Wrapper<Args, O, F, Fut>
where
    Args: 'static,
    O: 'static,
    F: FnOnce<Args, Output=Fut> + 'static,
    Fut: Future<Output=O> + 'static
{
    type Output = BoxFuture<O>;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        Box::pin(self.0.call_once(args))
    }
}

pub fn wrap_async<Args, F, Fut, O>(func: F) -> TakenAsyncFunc<Args, O>
where
    Args: 'static,
    O: 'static,
    F: FnOnce<Args, Output=Fut> + 'static,
    Fut: Future<Output=O> + 'static
{
    RefCell::new(Some(Box::new(Wrapper(func, PhantomData))))
}