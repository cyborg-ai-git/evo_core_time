//==================================================================================================
//  CyborgAI
//  CC BY-NC-ND 4.0 Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International
//  https://github.com/cyborg-ai-git
//==================================================================================================
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
//--------------------------------------------------------------------------------------------------
use evo_framework::TypeID;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use log::debug;
//--------------------------------------------------------------------------------------------------
pub struct UTime;
//--------------------------------------------------------------------------------------------------
impl UTime {
    /// Returns the current system time in nanoseconds since UNIX epoch
    #[inline(always)]
    pub fn time_ns() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX EPOCH")
            .as_nanos() as u64
    }

    /// Returns the current system time in milliseconds since UNIX epoch
    #[inline(always)]
    pub fn time_ms() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX EPOCH")
            .as_millis() as u64
    }

    /// Returns the current system time in seconds since UNIX epoch
    #[inline(always)]
    pub fn time_s() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX EPOCH")
            .as_secs() as u64
    }
}
//==================================================================================================
