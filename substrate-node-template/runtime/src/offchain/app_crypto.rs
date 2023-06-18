use sp_core::{crypto::KeyTypeId, sr25519::Signature as Sr25519Signature};
use sp_runtime::{
	app_crypto::{app_crypto, sr25519},
	traits::Verify,
	MultiSignature, MultiSigner,
};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");
app_crypto!(sr25519, KEY_TYPE);

pub struct AppCryptoSr25519;

impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AppCryptoSr25519 {
	type RuntimeAppPublic = Public;
	type GenericSignature = sp_core::sr25519::Signature;
	type GenericPublic = sp_core::sr25519::Public;
}

impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
	for AppCryptoSr25519
{
	type RuntimeAppPublic = Public;
	type GenericSignature = sp_core::sr25519::Signature;
	type GenericPublic = sp_core::sr25519::Public;
}
