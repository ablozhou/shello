#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! a helloworld onchain program
/// error module
pub mod error;

/// instruction module
pub mod instruction;

/// process module
pub mod process;

/// state to store final state
pub mod state;

/// the program entrypoint
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;