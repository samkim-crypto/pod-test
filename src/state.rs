use bytemuck::{Pod, Zeroable};
use crate::pod::*;

/// Account state to test
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TestAccount {
    /// Associated u64 counter as pod
    pub associated_counter: PodU64,
}
impl PodAccountInfo<'_, '_> for TestAccount {}


