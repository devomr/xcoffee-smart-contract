use crate::{storage, subscription::Subscription};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait SubscriptionFactory: storage::StorageModule {
    /// Function that creates a subscription and insert in the storage mappers
    ///
    /// Arguments:
    ///
    /// * creator - Wallet address of the creator
    /// * subscriber - Wallet address of the subscriber
    /// * plan - Subscription plan
    /// * deadline - Subscription deadline
    /// * amount - EGLD amount that will be payed for subscription
    ///
    fn create_subscription(
        &self,
        creator: ManagedAddress,
        subscriber: ManagedAddress,
        plan: ManagedBuffer,
        deadline: u64,
        amount: BigUint,
    ) {
        self.subscriptions_count().update(|id| {
            self.user_subscriptions(&subscriber).insert(*id);
            self.creator_subscriptions(&creator).insert(*id);
            self.subscription_user(id).set(&subscriber);

            let new_subscription = Subscription {
                creator,
                subscriber,
                deadline,
                plan,
                amount,
            };
            self.subscriptions(id).set(new_subscription);

            *id += 1;
        });
    }

    /// Function that updates the deadline of an existing subscription
    ///
    /// Arguments:
    ///
    /// * subscription_id - Subscription id
    /// * deadline - Subscription deadline
    ///
    fn update_subscription_deadline(&self, subscription_id: usize, deadline: u64) {
        self.subscriptions(&subscription_id)
            .update(|user_subscription| user_subscription.deadline = deadline);
    }
}
