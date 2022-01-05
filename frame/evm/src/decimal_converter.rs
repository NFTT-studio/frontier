use sp_runtime::{
	traits::{CheckedDiv, Saturating, Zero},
};
use sp_std::prelude::*;

/// Convert decimal between native(12) and EVM(18) and therefore the 1_000_000 conversion.
const DECIMALS_VALUE: u32 = 1_000_000u32;

/// Convert decimal from native(KAR/ACA 12) to EVM(18).
pub fn convert_decimals_to_evm<B: Zero + Saturating + From<u32>>(b: B) -> B {
	if b.is_zero() {
		return b;
	}
	b.saturating_mul(DECIMALS_VALUE.into())
}

/// Convert decimal from EVM(18) to native(KAR/ACA 12).
pub fn convert_decimals_from_evm<B: Zero + Saturating + CheckedDiv + PartialEq + Copy + From<u32>>(b: B) -> Option<B> {
	if b.is_zero() {
		return Some(b);
	}
	let res = b
		.checked_div(&Into::<B>::into(DECIMALS_VALUE))
		.expect("divisor is non-zero; qed");

	if res.saturating_mul(DECIMALS_VALUE.into()) == b {
		Some(res)
	} else {
		None
	}
}

