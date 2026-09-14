#![cfg_attr(
    all(doc, feature = "doc-uml"),
    doc = include_str!("../../documentation/data/e10879987690756349231.svg")
)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use js_sys::Date;

pub struct UTime;

impl UTime {
    #[inline(always)]
    pub fn time_ns() -> u64 {
        (Date::now() * 1_000_000.0) as u64 // Milliseconds to nanoseconds
    }

    #[inline(always)]
    pub fn time_ms() -> u64 {
        Date::now() as u64
    }

    #[inline(always)]
    pub fn time_s() -> u64 {
        (Date::now() / 1000.0) as u64 // Milliseconds to seconds
    }

}
