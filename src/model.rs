multiversx_sc::imports!();
multiversx_sc::derive_imports!();

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