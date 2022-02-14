use pallet_evm::{Context, Precompile, PrecompileResult, PrecompileSet};
use sp_core::H160;
use sp_std::marker::PhantomData;

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_evm_precompile_balances_erc20::{Erc20BalancesPrecompile, Erc20Metadata};
use frame_system_precompiles::FrameSystemWrapper;

pub struct FrontierPrecompiles<R>(PhantomData<R>);

impl<R> FrontierPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> sp_std::vec::Vec<H160> {
		sp_std::vec![1, 2, 3, 4, 5, 1024, 1025, 2057]
			.into_iter()
			.map(|x| hash(x))
			.collect()
	}
}
impl<R> PrecompileSet for FrontierPrecompiles<R>
where
	R: pallet_evm::Config,
	FrameSystemWrapper<R>: Precompile,
	Erc20BalancesPrecompile<R, NativeErc20Metadata>: Precompile,
{
	fn execute(
		&self,
		address: H160,
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
		is_static: bool,
	) -> Option<PrecompileResult> {
		match address {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(input, target_gas, context, is_static)),
			a if a == hash(2) => Some(Sha256::execute(input, target_gas, context, is_static)),
			a if a == hash(3) => Some(Ripemd160::execute(input, target_gas, context, is_static)),
			a if a == hash(4) => Some(Identity::execute(input, target_gas, context, is_static)),
			a if a == hash(5) => Some(Modexp::execute(input, target_gas, context, is_static)),
			// Non-Frontier specific nor Ethereum precompiles :
			a if a == hash(1024) => {
				Some(Sha3FIPS256::execute(input, target_gas, context, is_static))
			}
			a if a == hash(1025) => Some(ECRecoverPublicKey::execute(
				input, target_gas, context, is_static,
			)),
            a if a == hash(2050) => Some(Erc20BalancesPrecompile::<R, NativeErc20Metadata>::execute(
				input, target_gas, context, is_static,
			)),
            a if a == hash(2057) => Some(FrameSystemWrapper::<R>::execute(
				input, target_gas, context, is_static,
			)),
            a if a == hash(2058) => Some(FrameSystemWrapper::<R>::execute(input, target_gas, context, is_static)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160) -> bool {
		Self::used_addresses().contains(&address)
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

/// ERC20 metadata for the native token.
pub struct NativeErc20Metadata;

impl Erc20Metadata for NativeErc20Metadata {
        /// Returns the name of the token.
        fn name() -> &'static str {
                "DEV token"
        }

        /// Returns the symbol of the token.
        fn symbol() -> &'static str {
                "DEV"
        }

        /// Returns the decimals places of the token.
        fn decimals() -> u8 {
                18
        }
}
