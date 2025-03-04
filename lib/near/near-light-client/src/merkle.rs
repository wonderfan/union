use borsh::BorshSerialize;
use near_primitives_core::{hash::CryptoHash, types::MerkleHash};
use near_sdk::env;
use unionlabs::near::types::{Direction, MerklePath};

pub fn hash_borsh<T: BorshSerialize>(value: T) -> CryptoHash {
    CryptoHash(
        env::sha256(&borsh::to_vec(&value).expect("serialize will work"))
            .try_into()
            .expect("sha256 output size is always 32 bytes"),
    )
}

pub fn combine_hash(hash1: &MerkleHash, hash2: &MerkleHash) -> MerkleHash {
    hash_borsh((hash1, hash2))
}

/// Verify merkle path for given item and corresponding path.
pub fn verify_path<T: BorshSerialize>(root: MerkleHash, path: &MerklePath, item: T) -> bool {
    verify_hash(root, path, CryptoHash::hash_borsh(item))
}

pub fn verify_hash(root: MerkleHash, path: &MerklePath, item_hash: MerkleHash) -> bool {
    compute_root_from_path(path, item_hash) == root
}

pub fn compute_root_from_path(path: &MerklePath, item_hash: MerkleHash) -> MerkleHash {
    let mut res = item_hash;
    for item in path {
        match item.direction {
            Direction::Left => {
                res = combine_hash(&item.hash, &res);
            }
            Direction::Right => {
                res = combine_hash(&res, &item.hash);
            }
        }
    }
    res
}
