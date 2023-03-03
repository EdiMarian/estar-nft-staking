multiversx_sc::imports!();

use crate::model::Rarity;

#[multiversx_sc::module]
pub trait StorageModule {

    #[view(getToken)]
    #[storage_mapper("token")]
    fn token(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[view(getPause)]
    #[storage_mapper("pause")]
    fn pause(&self) -> SingleValueMapper<bool>;

    #[view(getUsersStaked)]
    #[storage_mapper("users_staked")]
    fn users_staked(&self) -> SetMapper<ManagedAddress>;

    #[storage_mapper("user_rewards")]
    fn user_rewards(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("nfts_staked")]
    fn nfts_staked(&self, address: &ManagedAddress) -> SetMapper<u64>;

    #[view(getNftStakedAt)]
    #[storage_mapper("nft_stakedAt")]
    fn nft_staked_at(&self, nonce: &u64) -> SingleValueMapper<u64>;

    #[view(getNftRarity)]
    #[storage_mapper("nft_rarity")]
    fn nft_rarity(&self, nonce: &u64) -> SingleValueMapper<Rarity>;

    #[view(getTokenAmount)]
    #[storage_mapper("token_amount")]
    fn token_amount(&self) -> SingleValueMapper<BigUint>;
}