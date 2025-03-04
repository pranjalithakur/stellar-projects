use ethers_core::{
    abi::AbiDecode,
    types::{Signature, H256},
};
use ethers_signers::LocalWallet;
use soroban_sdk::{Bytes, Env};

pub fn sign_message(env: &Env, message_bytes: &Vec<u8>, wallet: &LocalWallet) -> Signature {
    let message = env.crypto().keccak256(&Bytes::from_array(
        &env,
        arrayref::array_ref![message_bytes, 0, 32],
    ));
    let hash = H256::decode(message.to_array()).unwrap();

    wallet.sign_hash(hash).unwrap()
}

#[cfg(test)]
mod tests {
    use ethers_signers::LocalWallet;
    use soroban_sdk::Env;

    use super::sign_message;

    #[test]
    fn sign_message_test() {
        let env = Env::default();
        let expected_signature = "af7fdba729faa976d99ba71fa719a5c5b9a27e4cd4c1bde1cfa9c155663b702e0c0db1de0b3ed49e259f04d9cd1f61f24bff8becf63fc5f0bec8d3d1219390411c";

        let message_hash = "02071fcbf5613e2d8b01227c36a42ebfbe7bef39289119da3d281fcfb6e91ad3";

        // 0473f258c5df2ccecd2e4155ff6accba58d72361656e807da42506eae72fbfc0c78cf80438504f8e8301c85129f21c987ca656f70335a5843d3eae37ff5aeee71e
        let private_key = "b07d0e5f33e159bd7b471d0e79e4211205f4e89949247ec01ba7559b71acee77";
        let wallet = private_key.parse::<LocalWallet>().unwrap();

        let signature = sign_message(&env, &hex::decode(message_hash).unwrap(), &wallet);

        assert_eq!(&signature.to_string(), expected_signature);
    }
}
