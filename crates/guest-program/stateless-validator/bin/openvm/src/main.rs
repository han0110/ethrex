//! OpenVM Ethrex stateless validator guest program.

use ere_platform_openvm::OpenVMPlatform;
use ethrex_stateless_validator::platform::entrypoint;

fn main() {
    entrypoint::<OpenVMPlatform>();
}
