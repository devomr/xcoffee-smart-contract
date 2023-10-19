use crate::{donation::Donation, subscription::Subscription};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    /// Storage mapper used to store the donations received for an address
    ///
    /// Arguments:
    ///
    /// * creator - Wallet address who is receiving the donation
    ///
    #[storage_mapper("donations")]
    fn donations(&self, creator: &ManagedAddress) -> SetMapper<Donation<Self::Api>>;

    /// Storage mapper used to store the subscription's count
    ///
    #[view]
    #[storage_mapper("subscriptions_count")]
    fn subscriptions_count(&self) -> SingleValueMapper<usize>;

    /// Storage mapper used to store all existing subscriptions
    ///
    /// Arguments:
    ///
    /// * id - Subscription id
    ///
    #[view]
    #[storage_mapper("subscriptions")]
    fn subscriptions(&self, id: &usize) -> SingleValueMapper<Subscription<Self::Api>>;

    /// Storage mapper used to store the subscriptions id that an user has
    ///
    /// Arguments:
    ///
    /// * user - Wallet address of the user
    ///
    #[view]
    #[storage_mapper("user_subscriptions")]
    fn user_subscriptions(&self, user: &ManagedAddress) -> UnorderedSetMapper<usize>;

    /// Storage mapper used to store the user address which belongs to a subscription id
    ///
    /// Arguments:
    ///
    /// * id - Subscription id
    ///
    #[view]
    #[storage_mapper("subscription_user")]
    fn subscription_user(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;
}
