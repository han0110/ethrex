//! Crypto provider selection for the guest: every zkVM routes through the
//! zkvm-standards `zkvm-interface` syscalls. ZisK and SP1 export those symbols
//! from their runtimes, OpenVM from `ere-platform-openvm`'s zkvm-accelerator.
//! This keeps guest crypto decoupled from per-SDK patched-crate stacks, because ere
//! pins sp1 v6.4.0 and openvm v2.1.0-preview, which the ethrex first-party providers
//! in `ethrex_guest_program::crypto` do not target.

#[cfg(feature = "zkvm-interface")]
mod zkvm_interface;

use std::sync::Arc;

use ethrex_crypto::Crypto;

/// Returns the [`Crypto`] implementation for the active zkVM feature.
pub fn crypto() -> Arc<dyn Crypto> {
    #[cfg(feature = "zkvm-interface")]
    return zkvm_interface::crypto();
    #[cfg(not(feature = "zkvm-interface"))]
    return Arc::new(ethrex_crypto::NativeCrypto);
}
