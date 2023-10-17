multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// Donation struct
#[derive(NestedDecode, NestedEncode, TopDecode, TopEncode, TypeAbi)]
pub struct Donation<M: ManagedTypeApi> {
    pub sender: ManagedAddress<M>,
    pub name: ManagedBuffer<M>,
    pub message: ManagedBuffer<M>,
    pub amount: BigUint<M>,
}
