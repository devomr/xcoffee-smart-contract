#![no_std]

mod donation;
mod donation_factory;
mod storage;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait Xcoffee: donation_factory::DonationFactory + storage::StorageModule {
    #[init]
    fn init(&self) {}
    /// Nothing to do yet when the smart contract is deployed

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
        require!(*donation_amount > 0u32, "Donation must be more than 0");

        let sender = self.blockchain().get_caller();

        self.create_donation(
            to.clone(),
            sender,
            name,
            message,
            donation_amount.clone_value(),
        );
        self.send().direct_egld(&to, &donation_amount)
    }
}
