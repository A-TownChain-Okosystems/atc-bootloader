//! UEFI Bootloader — GPT, Secure Boot, Kernel-Handoff
//!
//! Part of the A-TownChain-Okosystems ecosystem.
//! Copyright (c) Michael Wroblewski. All Rights Reserved.

#![no_std]

pub mod main;
pub mod gpt;
pub mod secure_boot;
pub mod kernel_handoff;
pub mod framebuffer;
