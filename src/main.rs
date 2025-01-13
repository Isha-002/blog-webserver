mod error;
mod routes;
mod store;
mod types;

use std::sync::Arc;

use axum::{routing::get, Router};
use chrono::Local;
use routes::{
    blogs::{blogs, delete_blog, post_blog, put_blog, single_blog},
    home::home,
};
use store::Store as State;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ops::Deref;

/// This is so unsafe (Quark says this is safe! I trust Quark)
struct TrustMeBro<T> {
    inner: UnsafeCell<MaybeUninit<T>>,
}
impl<T> TrustMeBro<T> {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
    /// You must only call this once, and it must be the first thing you call
    pub unsafe fn write(&self, val: T) {
        unsafe {
            (*self.inner.get()).write(val);
        }
    }
}
/// Must have had write() called first
impl<T> Deref for TrustMeBro<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { (*self.inner.get()).assume_init_ref() }
    }
}
/// Just to quiet rustc's fears about "thread safety" and "comprehensible program structure"
unsafe impl<T: Sync> Sync for TrustMeBro<T> {}

static STATE: TrustMeBro<State> = TrustMeBro::new();


fn main() {
    // This NEEDS to be the first thing that happens
    // never call write again and this is a global static reference
    unsafe {
        STATE.write(State::init())
    }; 
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let store = Arc::new(STATE.clone());
            let app = Router::new()
            .route("/", get(home))
            .route("/blogs", get(blogs).post(post_blog))
            .route(
                "/blogs/{id}",
                get(single_blog).put(put_blog).delete(delete_blog),
            )
            .with_state(store);
    
        let time = Local::now().format("%Y-%m-%d %H:%M:%S");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:4445").await.unwrap();
        println!("{time} start the server on http://localhost:4445/");
        axum::serve(listener, app).await.unwrap();
        });

    // let store = Arc::new(Store::init());

}

// problems:
// - no idea how to implement error handling
// - need a better solution to handle ids
