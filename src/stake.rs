#![no_std]

use model::Rarity;

multiversx_sc::imports!();
mod storage;
mod views;
pub mod model;

pub const ONE_DAY_IN_SECONDS: u64 = 60;
const ESTAR_DECIMALS: u64 = 1000000000000000000;

#[multiversx_sc::contract]
pub trait StakeContract: storage::StorageModule + views::ViewsModule {

    #[init]
    fn init(&self, token: TokenIdentifier) {
        if self.token().is_empty() {
            self.token().set_token_id(token);
        }
    }

    #[only_owner]
    #[endpoint(togglePause)]
    fn toggle_pause(&self) {
        self.pause().update(|pause| *pause = !*pause);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(fundSystem)]
    fn fund(&self) {
        let payment = self.call_value().single_esdt();

        require!(payment.amount > BigUint::zero(), "Amount must be greater than zero!");

        self.token_amount().update(|amount| *amount += payment.amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(withdrawFunds)]
    fn withdraw(&self, withdraw_amount: BigUint) {
        let caller = self.blockchain().get_caller();
        require!(withdraw_amount > BigUint::zero(), "Amount must be greater than zero!");
        self.send().direct_esdt(&caller, &TokenIdentifier::from("ESTAR-461bab".as_bytes()), 0, &withdraw_amount);
        self.token_amount().update(|amount| *amount -= &withdraw_amount);
    }

    #[only_owner]
    #[endpoint(setNftRarity)]
    fn set_nft_rarity(&self, nfts: MultiValueEncoded<MultiValue2<u64, u64>>) {
        for nft in nfts.into_iter() {
            let tuple = nft.into_tuple();
            let rarity = match tuple.1 {
                1 => Rarity::Common,
                2 => Rarity::CommonGold,
                3 => Rarity::Rare,
                4 => Rarity::UltraRare,
                5 => Rarity::Epic,
                6 => Rarity::Legendary,
                _ => Rarity::None,
            };
            require!(rarity != Rarity::None, "Invalid rarity!");
            self.nft_rarity(&tuple.0).set(rarity);
        }
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self, #[payment_multi] payments: ManagedVec<EsdtTokenPayment<Self::Api>>) {
        require!(!self.pause().get(), "The stake is stopped!");
        self.token().require_all_same_token(&payments);

        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();
        for payment in payments.into_iter() {
            self.nfts_staked(&caller).insert(payment.token_nonce.clone());
            self.nft_staked_at(&payment.token_nonce).set(current_time);
        }

        if !self.users_staked().contains(&caller) {
            self.users_staked().insert(caller);
        }
    }

    #[endpoint(unStake)]
    fn un_stake(&self, nfts_to_unstake: MultiValueEncoded<MultiValue2<TokenIdentifier, u64>>) {
        require!(!self.pause().get(), "The stake is stopped!");

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();
        let token_id = self.token().get_token_id();

        let caller = self.blockchain().get_caller();

        for nft in nfts_to_unstake.into_iter() {
            let tuple = nft.into_tuple();
            require!(self.nfts_staked(&caller).contains(&tuple.1), "One or more nfts do not belong to you!");
            require!(token_id == tuple.0, "Invalid token!");

            let payment = EsdtTokenPayment::new(tuple.0, tuple.1.clone(), BigUint::from(1u64));
            payments.push(payment);

            self.set_rewards_for_nft_to_user(&tuple.1);
            self.nfts_staked(&caller).remove(&tuple.1);
            self.nft_staked_at(&tuple.1).clear();
        };
        self.send().direct_multi(&caller, &payments);

        if self.nfts_staked(&caller).len() == 0usize {
            self.users_staked().remove(&caller);
        }
    }

    #[endpoint(claimRewards)]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();
        let reward = self.get_rewards(&caller);

        require!(reward > BigUint::zero(), "Amount of estar must be greater than zero!");
        require!(reward <= self.token_amount().get(), "It is not enough estar in SC!");

        self.reset_nfts_staked_at_for_user(&caller);
        self.user_rewards(&caller).clear();

        self.token_amount().update(|amount| *amount -= &reward);
        let reward_to_payment = reward * BigUint::from(ESTAR_DECIMALS);
        self.send().direct_esdt(&caller, &TokenIdentifier::from("ESTAR-461bab".as_bytes()), 0, &reward_to_payment);
    }

    // Private functions
    fn set_rewards_for_nft_to_user(&self, nft_nonce: &u64) {
        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();

        let nft_staked_at = self.nft_staked_at(nft_nonce).get();
        let nft_rarity = self.nft_rarity(nft_nonce).get();
        let days_staked: u64 = (current_time - nft_staked_at) / ONE_DAY_IN_SECONDS;

        if days_staked >= 1 {
            let reward = self.get_reward_for_rarity(&nft_rarity);
            self.user_rewards(&caller).update(|amount| *amount += reward * BigUint::from(days_staked))
        };
    }

    fn reset_nfts_staked_at_for_user(&self, address: &ManagedAddress) {
        let current_time = self.blockchain().get_block_timestamp();
        for nft_nonce in self.nfts_staked(address).iter() {
            self.nft_staked_at(&nft_nonce).set(current_time);
        };
    }
}