pub mod root;
pub mod modeq;
pub mod hash_to_prime;
pub mod membership;

use std::cell::{BorrowError, BorrowMutError};

quick_error! {
    #[derive(Debug)]
    pub enum ChannelError {
        CouldNotSend {}
        CouldNotBorrow(e: BorrowError) {
            from()
        }
        CouldNotBorrowMut(e: BorrowMutError) {
            from()
        }
    }
}