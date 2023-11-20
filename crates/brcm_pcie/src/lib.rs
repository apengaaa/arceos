#![no_std]
#![feature(const_ptr_as_ref)]
#![feature(const_option)]
#![feature(const_nonnull_new)]
mod bcm2711_pcie;

pub use bcm2711_pcie::*;
/// reset controller

/// sets bit 1 of [pcie->base+0x9210] to val
pub trait BCM2711Hal {
    fn sleep(ms:core::time::Duration);
}