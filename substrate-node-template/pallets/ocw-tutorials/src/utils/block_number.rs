use sp_runtime::traits::{AtLeast32BitUnsigned, Zero};

pub(crate) fn is_odd_block_number<BN>(block_number: BN) -> bool
where
	BN: AtLeast32BitUnsigned,
{
	return block_number % 2u32.into() != Zero::zero()
}
