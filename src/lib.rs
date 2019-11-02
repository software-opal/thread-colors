#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod thread;
pub use crate::thread::{OwnedThread, Thread, ThreadRef};

mod generated;
