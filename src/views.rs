multiversx_sc::imports!();

use crate::storage;

use crate::ONE_DAY_IN_SECONDS;

#[multiversx_sc::module]
pub trait ViewsModule: storage::StorageModule {

    #[view(getRewards)]
    fn get_rewards(&self, address: &ManagedAddress) -> BigUint {
        let mut total = BigUint::zero();
        let user_rewards = self.user_rewards(address).get();

        let current_time = self.blockchain().get_block_timestamp();
        let last_claim = self.user_last_claim(address).get();
        let reward_per_nft = self.reward_per_nft().get();
        let nfts_at_stake = self.nfts_staked(address).len();
        let time_at_stake = current_time - last_claim;
        let reward = (reward_per_nft * BigUint::from(nfts_at_stake) * BigUint::from(time_at_stake)) / BigUint::from(ONE_DAY_IN_SECONDS);

        if reward > 0 {
            total += reward;
        }
        if user_rewards > BigUint::zero() {
            total += user_rewards;
        }
        total
    }
}