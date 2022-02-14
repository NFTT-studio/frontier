extern crate alloc;
use pallet_evm::{Context, Precompile, PrecompileResult};
use sp_core::H160;
use sp_std::marker::PhantomData;
use alloc::collections::BTreeMap;

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};

pub struct MyFrontierPrecompiles<R>(PhantomData<R>);

pub type PrecompileFn = fn(&[u8], Option<u64>, &Context, bool) -> PrecompileResult;

impl<R> MyFrontierPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> BTreeMap<H160, PrecompileFn> {
		let mut pset = BTreeMap::<H160, PrecompileFn>::new();
		pset.insert(hash(0x0000000000000000000000000000000000000001), ECRecover::execute);
		pset.insert(hash(0x0000000000000000000000000000000000000002), Sha256::execute);
		pset.insert(hash(0x0000000000000000000000000000000000000003), Ripemd160::execute);
		pset.insert(hash(0x0000000000000000000000000000000000000004), Identity::execute);
		pset.insert(hash(0x0000000000000000000000000000000000000005), Modexp::execute);
		// Non-Frontier specific nor Ethereum precompiles :
		pset.insert(hash(0x0000000000000000000000000000000000000400), Sha3FIPS256::execute);
		pset.insert(hash(0x0000000000000000000000000000000000000401), ECRecoverPublicKey::execute);
		pset
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
