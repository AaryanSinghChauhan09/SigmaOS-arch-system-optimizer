// Integration test for SigmaOS-arch-system-optimizer
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use arch_system_optimizer as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
