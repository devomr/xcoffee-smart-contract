use crate::donation::Donation;

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait StorageModule {
    /// Storage mapper used to store the donations received for an address
    ///
    /// Arguments:
    ///
    /// * creator - Wallet address who is receiving the donation
    ///
    #[storage_mapper("donations")]
    fn donations(&self, creator: &ManagedAddress) -> SetMapper<Donation<Self::Api>>;
}
