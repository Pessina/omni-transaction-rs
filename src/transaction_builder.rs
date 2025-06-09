//! High level transaction builder that can be used to build transactions for different chains.

/// Transaction trait builder for different chains.
pub trait TxBuilder<T> {
    fn build(&self) -> T;
}

/// High level structure to build transactions for different chains.
pub struct TransactionBuilder;

impl TransactionBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> T
    where
        T: Default,
    {
        T::default()
    }
}

