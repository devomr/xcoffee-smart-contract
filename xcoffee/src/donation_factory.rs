use crate::{donation::Donation, storage};

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait DonationFactory: storage::StorageModule {
    /// Function that creates a donation and insert it in the storage mapper
    ///
    /// Arguments:
    ///
    /// * receiver - Wallet address who is receiving the donation
    /// * sender - Wallet address who is sending the donation
    /// * name - A string that represents the name of the sender
    /// * message - A string that represents a message from sender
    /// * amount - EGLD amount that will be donated
    ///
    fn create_donation(
        &self,
        receiver: ManagedAddress,
        sender: ManagedAddress,
        name: ManagedBuffer,
        message: ManagedBuffer,
        amount: BigUint,
    ) {
        let new_donation = Donation {
            sender,
            name,
            message,
            amount,
        };

        self.donations(&receiver).insert(new_donation);
    }
}
