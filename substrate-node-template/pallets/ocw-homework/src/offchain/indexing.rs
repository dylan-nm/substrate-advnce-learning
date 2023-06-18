use codec::{Decode, Encode};
use sp_io::offchain_index;
use sp_std::vec::Vec;

pub(crate) fn derived_key<BN>(block_number: BN, key: &[u8]) -> Vec<u8>
where
	BN: Encode,
{
	block_number.using_encoded(|encoded_bn| {
		key.iter().chain(b"@".iter()).chain(encoded_bn).copied().collect::<Vec<u8>>()
	})
}

#[derive(Debug, Default, Encode, Decode)]
struct IndexingData(Vec<u8>, u64);

pub(crate) fn offchain_index_set<BN>(block_number: BN, number: u64)
where
	BN: Encode,
{
	let key = derived_key(block_number, b"indexing_1");
	let data = IndexingData(b"submit_number_unsigned".to_vec(), number).encode();
	log::info!("offchain_index::set, key: {:?}, data: {:?}", key, data);
	offchain_index::set(&key, &data);
}
