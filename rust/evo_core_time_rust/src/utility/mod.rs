#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
pub(crate) mod u_time;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub(crate) mod u_time_wasm;
pub (crate) mod u_time_elapsed;
pub (crate) mod u_time_ext;