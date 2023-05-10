multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageModule {

    #[view(getCollection)]
    #[storage_mapper("collection")]
    fn collection(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[view(getRewardToken)]
    #[storage_mapper("reward_token")]
    fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getRewardTokenAmount)]
    #[storage_mapper("reward_token_amount")]
    fn reward_token_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardPerNft)]
    #[storage_mapper("reward_per_nft")]
    fn reward_per_nft(&self) -> SingleValueMapper<BigUint>;

    #[view(getPause)]
    #[storage_mapper("pause")]
    fn pause(&self) -> SingleValueMapper<bool>;

    #[view(allowList)]
    #[storage_mapper("allow_list")]
    fn allow_list(&self) -> SetMapper<u64>;

    #[view(getUsersStaked)]
    #[storage_mapper("users_staked")]
    fn users_staked(&self) -> SetMapper<ManagedAddress>;

    #[view(getNftsStaked)]
    #[storage_mapper("nfts_staked")]
    fn nfts_staked(&self, address: &ManagedAddress) -> SetMapper<u64>;

    #[storage_mapper("user_rewards")]
    fn user_rewards(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getUserLastClaim)]
    #[storage_mapper("user_last_claim")]
    fn user_last_claim(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getNftStakedAt)]
    #[storage_mapper("nft_stakedAt")]
    fn nft_staked_at(&self, nonce: &u64) -> SingleValueMapper<u64>;
}