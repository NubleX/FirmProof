#![cfg_attr(not(feature = "std"), no_std)]

pub use sp_runtime::RuntimeDebug as Debug;

// Minimal stub runtime – replace with FRAME System/Timestamp/Balances and construct_runtime!

#[cfg(feature = "std")]
pub fn about() -> &'static str {
    "Secure IoT Ledger runtime (stub). TODO: wire standard pallets and our custom pallets."
}
