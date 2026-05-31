//#========================================================================================================================================
//#   CyborgAI CC BY-NC-ND 4.0 Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International  https://github.com/cyborg-ai-git
//#========================================================================================================================================

//--------------------------------------------------------------------------------------------------
#![allow(missing_docs)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/129898917?v=4")]
#![doc(html_no_source)]
//! ## evo_core_id
//! ### 202512321
//!
//! {_DOC_}
//!
//--------------------------------------------------------------------------------------------------
/// Version of the package
pub const EVO_VERSION: u64 = 202512321;
//--------------------------------------------------------------------------------------------------
#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(unused_assignments)]
mod utility;
#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
pub use utility::u_time::UTime;
#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
pub use utility::u_time_ext::UTimeExt;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub use utility::u_time_wasm::UTime;

pub use utility::u_time_elapsed::UTimeElapsed;
//---------------------------------------------------------------------------------------------------------------------------------------
