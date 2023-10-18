multiversx_sc::imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait SubscriptionHelper: storage::StorageModule {
    /// Function that checks if a subscription belongs to an wallet address
    ///
    /// Arguments:
    ///
    /// * subscription_id - Subscription id
    /// * caller - Wallet address of the caller
    ///
    fn check_subscription_belongs_to_caller(
        &self,
        subscription_id: usize,
        caller: &ManagedAddress,
    ) {
        require!(
            caller == &self.subscription_user(&subscription_id).get(),
            "Only the owner of the subscription can perform this operation"
        );
    }
}
