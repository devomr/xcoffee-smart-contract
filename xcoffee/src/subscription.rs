multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// Subscription struct
#[derive(NestedDecode, NestedEncode, TopDecode, TopEncode, TypeAbi)]
pub struct Subscription<M: ManagedTypeApi> {
    pub creator: ManagedAddress<M>,
    pub subscriber: ManagedAddress<M>,
    pub deadline: u64,
    pub plan: ManagedBuffer<M>,
    pub amount: BigUint<M>,
}
