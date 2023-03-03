multiversx_sc::imports!();

use crate::storage;
use crate::model::*;

use crate::ONE_DAY_IN_SECONDS;

use multiversx_sc::types::heap::Vec;

#[multiversx_sc::module]
pub trait ViewsModule: storage::StorageModule {

    #[view(getNftsStaked)]
    fn get_nfts_staked(&self, address: &ManagedAddress) -> Vec<NFTStakedDetailed<Self::Api>> {
        let mut nfts = Vec::new();
        let token_id = self.token().get_token_id();
        for nft_staked in self.nfts_staked(address).iter() {
            let staked_at = self.nft_staked_at(&nft_staked).get();

            let nft = NFTStakedDetailed {
                identifier: token_id.clone(),
                nonce: nft_staked,
                staked_at: staked_at
            };
            nfts.push(nft);
        }
        nfts
    }

    #[view(getRewards)]
    fn get_rewards(&self, address: &ManagedAddress) -> BigUint {
        let mut total = BigUint::zero();
        let current_time = self.blockchain().get_block_timestamp();
        let user_rewards = self.user_rewards(address).get();

        for nft in self.nfts_staked(&address).iter() {
            let nft_staked_at = self.nft_staked_at(&nft).get();
            let nft_rarity = self.nft_rarity(&nft).get();
            let days_staked: u64 = (current_time - nft_staked_at) / ONE_DAY_IN_SECONDS;

            if days_staked >= 1 {
                let reward = self.get_reward_for_rarity(&nft_rarity);

                total += reward * BigUint::from(days_staked);
            };
        }
        if user_rewards > BigUint::zero() {
            total += user_rewards;
        }
        total
    }

    #[view(getRewardForRarity)]
    fn get_reward_for_rarity(&self, rarity: &Rarity) -> BigUint {
        return match rarity {
            Rarity::Common => BigUint::from(3u64),
            Rarity::CommonGold => BigUint::from(3u64),
            Rarity::Rare => BigUint::from(7u64),
            Rarity::UltraRare => BigUint::from(30u64),
            Rarity::Epic => BigUint::from(100u64),
            Rarity::Legendary => BigUint::from(200u64),
            _ => BigUint::zero()
        };
    }
}