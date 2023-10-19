#![no_std]

mod donation;
mod donation_factory;
mod storage;
mod subscription;
mod subscription_factory;
mod subscription_helper;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// A contract that allows creators to:
/// * receive donations in EGLD from another users
/// * have membership subscription plans that the users can choose to subscribe by paying a fix amount of EGLD
///
#[multiversx_sc::contract]
pub trait Xcoffee:
    donation_factory::DonationFactory
    + subscription_factory::SubscriptionFactory
    + subscription_helper::SubscriptionHelper
    + storage::StorageModule
{
    #[init]
    fn init(&self) {
        // Start subscription count from 1
        self.subscriptions_count().set(1);
    }

    /// Endpoint that is called to perform a donation.
    /// It will create a new donation and will send the EGLD amount to the receiver
    ///
    /// Arguments:
    ///
    /// * to - Wallet address who is receiving the donation
    /// * name - A string that represents the name of the sender
    /// * message - A string that represents a message from sender
    ///
    #[endpoint]
    #[payable("EGLD")]
    fn donate(&self, to: &ManagedAddress, name: ManagedBuffer, message: ManagedBuffer) {
        let donation_amount = self.call_value().egld_value();
        let sender = self.blockchain().get_caller();

        // Validate donation amount to be more than 0
        require!(*donation_amount > 0u32, "Donation must be more than 0");

        // Create donation and send the EGLD to creator
        self.create_donation(
            to.clone(),
            sender,
            name,
            message,
            donation_amount.clone_value(),
        );
        self.send().direct_egld(&to, &donation_amount)
    }

    /// Endpoint that is called to create an user subscription.
    /// It will calculate the subscription deadline, will create a new subscription
    /// and will send the EGLD amount to the creator
    ///
    /// Arguments:
    ///
    /// * creator - Wallet address of creator
    /// * plan - A string that represents the subscription plan
    /// * duration_in_seconds - Subscription duration in seconds
    ///
    #[endpoint]
    #[payable("EGLD")]
    fn create_user_subscription(
        &self,
        creator: &ManagedAddress,
        plan: ManagedBuffer,
        duration_in_seconds: u64,
    ) {
        let caller = self.blockchain().get_caller();
        let amount = self.call_value().egld_value();
        let block_timestamp = self.blockchain().get_block_timestamp();

        // calculate subscription deadline
        let deadline = block_timestamp + duration_in_seconds;

        // Validate the new deadline to not be in the past
        require!(deadline > block_timestamp, "deadline cannot be in the past");

        // Create subscription and send the EGLD to creator
        self.create_subscription(
            creator.clone(),
            caller,
            plan,
            deadline,
            amount.clone_value(),
        );

        self.send().direct_egld(&creator, &amount);
    }

    /// Endpoint that is called to renew an existing subscription.
    /// It will calculate the new subscription deadline and it will update the existing subscription.
    ///
    /// This operation can be performed only by the user who created the subscription.
    ///
    /// Arguments:
    ///
    /// * subscription_id - Subscription id
    /// * duration_in_seconds - Subscription duration in seconds
    ///
    #[endpoint]
    #[payable("EGLD")]
    fn renew_subscription(&self, subscription_id: usize, duration_in_seconds: u64) {
        let caller = self.blockchain().get_caller();
        let block_timestamp = self.blockchain().get_block_timestamp();
        let amount = self.call_value().egld_value();

        // Validate if the subscription belongs to the caller
        self.check_subscription_belongs_to_caller(subscription_id, &caller);

        // calculate subscription new deadline
        let deadline = block_timestamp + duration_in_seconds;

        // Validate the new deadline to not be in the past
        require!(deadline > block_timestamp, "deadline cannot be in the past");

        // Update the subscription with the new deadline
        self.update_subscription_deadline(subscription_id, block_timestamp);

        // Get the subscription and send the EGLD to the creator
        let user_subscription = self.subscriptions(&subscription_id).get();
        self.send().direct_egld(&user_subscription.creator, &amount);
    }

    /// Endpoint that is called to cancel an existing subscription.
    /// It will update the subscription deadline to the block timestamp.
    ///
    /// This operation can be performed only by the user who created the subscription.
    ///
    /// Arguments:
    ///
    /// * subscription_id - Subscription id
    /// * duration_in_seconds - Subscription duration in seconds
    ///
    #[endpoint]
    fn cancel_subscription(&self, subscription_id: usize) {
        let caller = self.blockchain().get_caller();

        // Validate if the subscription belongs to the caller
        self.check_subscription_belongs_to_caller(subscription_id, &caller);

        // Remove the subscription from the user subscriptions storage mapper TOFIX
        let mut subscriptions = self.user_subscriptions(&caller);
        subscriptions.swap_remove(&subscription_id);

        // Remove the subscription assigned for the user address from storage mapper
        self.subscription_user(&subscription_id).clear();

        // Remove the subscription from the list of available subscriptions from storage mapper
        self.subscriptions(&subscription_id).clear();
    }

    /// View that checkes if an user has an active subscription linked to a creator
    /// wallet address and returns the user subscription deadline and the subscription id if found, otherwise None
    ///
    /// Arguments:
    ///
    /// * user_address - Wallet address of user
    /// * creator_address -  Wallet address of creator
    ///
    #[view(getUserSubscriptionDeadline)]
    fn get_user_subscription_deadline(
        &self,
        user_addres: &ManagedAddress,
        creator_address: &ManagedAddress,
    ) -> (usize, u64) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let user_subscription_ids = self.user_subscriptions(user_addres);

        for subscription_id in &user_subscription_ids {
            let subscription_mapper = self.subscriptions(&subscription_id);
            let subscription = subscription_mapper.get();

            if &subscription.creator == creator_address {
                let time_left = subscription.deadline - current_timestamp;
                return (subscription_id, time_left);
            }
        }
        return (0, 0);
    }
}
