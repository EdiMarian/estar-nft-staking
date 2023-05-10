#![no_std]

multiversx_sc::imports!();
mod storage;
mod views;

pub const ONE_DAY_IN_SECONDS: u64 = 86400;

#[multiversx_sc::contract]
pub trait StakeContract: storage::StorageModule + views::ViewsModule {

    #[init]
    fn init(&self, collection_id: TokenIdentifier, token: TokenIdentifier) {
        if self.collection().is_empty() {
            self.collection().set_token_id(collection_id);
        }
        self.reward_token().set(token);
    }

    #[only_owner]
    #[endpoint(setRewardPerNft)]
    fn set_reward_per_nft(&self, reward: BigUint) {
        self.reward_per_nft().update(|amount| *amount = reward);
    }

    #[only_owner]
    #[endpoint(togglePause)]
    fn toggle_pause(&self) {
        self.pause().update(|pause| *pause = !*pause);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(fundSystem)]
    fn fund_system(&self) {
        let payment = self.call_value().single_esdt();

        require!(payment.amount > BigUint::zero(), "Amount must be greater than zero!");

        self.reward_token_amount().update(|amount| *amount += payment.amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(withdrawFunds)]
    fn withdraw_funds(&self, withdraw_amount: BigUint) {
        let caller = self.blockchain().get_caller();
        require!(withdraw_amount > BigUint::zero(), "Amount must be greater than zero!");
        self.send().direct_esdt(&caller, &self.reward_token().get(), 0, &withdraw_amount);
        self.reward_token_amount().update(|amount| *amount -= &withdraw_amount);
    }

    #[only_owner]
    #[endpoint(setAllowList)]
    fn set_allow_list(&self, items: MultiValueEncoded<u64>) {
        for item in items.into_iter() {
            self.allow_list().insert(item);
        }
    }

    #[only_owner]
    #[endpoint(removeFromAllowList)]
    fn remove_from_allow_list(&self, items: MultiValueEncoded<u64>) {
        for item in items.into_iter() {
            self.allow_list().remove(&item);
        }
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self, #[payment_multi] nfts: ManagedVec<EsdtTokenPayment<Self::Api>>) {
        require!(!self.pause().get(), "The stake is stopped!");
        self.collection().require_all_same_token(&nfts);

        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();

        if self.user_last_claim(&caller).is_empty() {
            self.user_last_claim(&caller).set(current_time);
        } else {
            self.calculate_user_rewards_and_save(&caller, current_time);
            self.user_last_claim(&caller).set(current_time);
        }

        for payment in nfts.into_iter() {
            require!(self.allow_list().contains(&payment.token_nonce), "This NFT is not allowed to stake!");
            self.nfts_staked(&caller).insert(payment.token_nonce.clone());
            self.nft_staked_at(&payment.token_nonce).set(current_time);
        }

        if !self.users_staked().contains(&caller) {
            self.users_staked().insert(caller);
        }
    }

    #[endpoint(unStake)]
    fn un_stake(&self, nfts_to_unstake: MultiValueEncoded<u64>) {
        require!(!self.pause().get(), "The stake is stopped!");

        let collection_id = self.collection().get_token_id();
        let caller = self.blockchain().get_caller();

        self.calculate_user_rewards_and_save(&caller, self.blockchain().get_block_timestamp());

        for nft in nfts_to_unstake.into_iter() {
            require!(self.nfts_staked(&caller).contains(&nft), "One or more nfts do not belong to you!");
            self.nfts_staked(&caller).remove(&nft);
            self.nft_staked_at(&nft).clear();

            self.send().direct_esdt(&caller, &collection_id, nft, &BigUint::from(1u64));
        };

        if self.nfts_staked(&caller).len() == 0usize {
            self.users_staked().remove(&caller);
            self.user_last_claim(&caller).clear();
        }
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        require!(!self.pause().get(), "The stake is stopped!");
        
        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();
        self.calculate_user_rewards_and_save(&caller, current_time);
        self.user_last_claim(&caller).set(current_time);

        let reward = self.user_rewards(&caller).get();

        require!(reward > BigUint::zero(), "Amount of ESTAR must be greater than zero!");
        require!(reward <= self.reward_token_amount().get(), "It is not enough ESTAR in SC!");
        
        self.reward_token_amount().update(|amount| *amount -= &reward);
        self.send().direct_esdt(&caller, &self.reward_token().get(), 0, &reward);
        self.user_rewards(&caller).clear();
    }

    // Private functions
    fn calculate_user_rewards_and_save(&self, caller: &ManagedAddress, current_time: u64) {
        let last_claim = self.user_last_claim(caller).get();
        let reward_per_nft = self.reward_per_nft().get();
        let nfts_at_stake = self.nfts_staked(caller).len();
        let time_at_stake = current_time - last_claim;
        let reward = (reward_per_nft * BigUint::from(nfts_at_stake) * BigUint::from(time_at_stake)) / BigUint::from(ONE_DAY_IN_SECONDS);

        if reward > 0 {
            self.user_rewards(caller).update(|amount| *amount += reward);
        }
    }
}