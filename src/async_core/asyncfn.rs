use std::sync::Arc;
use std::task::{Context, Poll};
use bytes::Bytes;
use futures_util::FutureExt;
use http::Response;

use std::future::Future;
use std::pin::Pin;




// Define a trait for creating the callback future
pub trait CreateCallback: Send + Sync + 'static {
   fn create_callback(&self) -> Pin<Box<dyn Future<Output = http::Response<Bytes>> + Send>>;
}

// Implement the trait for a closure that returns a pinned boxed future
impl<F, Fut> CreateCallback for F
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = http::Response<Bytes>> + Send + 'static,
{
    fn create_callback(&self) -> Pin<Box<dyn Future<Output = http::Response<Bytes>> + Send + 'static>> {
        Box::pin((self)())
    }
}

// A struct to hold the callback creator
pub struct CallbackHolder {
    pub callback: Arc<dyn CreateCallback + Send + Sync>,
}

impl CallbackHolder {
    pub fn new<F, Fut>(callback: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = http::Response<Bytes>> + Send + 'static,
    {
        CallbackHolder {
            callback: Arc::new(callback),
        }
    }
}

