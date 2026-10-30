//! RFC-0077 S0: the host's clock, in place of `performance.now()` and `Date.now()`.

#[link(wasm_import_module = "nuthatch")]
extern "C" {
	#[link_name = "now_ms"]
	fn host_now_ms() -> f64;
}

/// Milliseconds since the Unix epoch, as the host reads its wall clock.
#[allow(unsafe_code)]
pub(super) fn now_ms() -> f64 {
	// SAFETY: takes nothing and returns a number.
	unsafe { host_now_ms() }
}
