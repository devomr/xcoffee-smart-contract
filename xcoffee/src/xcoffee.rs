#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedDecode, NestedEncode, TopDecode, TopEncode, TypeAbi)]
pub struct Donation<M: ManagedTypeApi> {
    sender: ManagedAddress<M>,
    name: ManagedBuffer<M>,
    message: ManagedBuffer<M>,
    amount: BigUint<M>,
}

#[multiversx_sc::contract]
pub trait Xcoffee {
    #[init]
    fn init(&self) {}

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

    #[view(getDonations)]
    #[storage_mapper("donations")]
    fn donations(&self, creator: &ManagedAddress) -> SetMapper<Donation<Self::Api>>;
}
