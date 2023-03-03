multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct NFTStaked<M: ManagedTypeApi> {
    pub identifier: TokenIdentifier<M>,
    pub nonce: u64
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct NFTStakedDetailed<M: ManagedTypeApi> {
    pub identifier: TokenIdentifier<M>,
    pub nonce: u64,
    pub staked_at: u64
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct NFTUnbondDetailed<M: ManagedTypeApi> {
    pub identifier: TokenIdentifier<M>,
    pub nonce: u64,
    pub deadline: u64
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq)]
pub enum Rarity {
  None,
  Common,
  CommonGold,
  Rare,
  UltraRare,
  Epic,
  Legendary
}