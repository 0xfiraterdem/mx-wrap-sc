#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod storage;
mod owner;

#[multiversx_sc::contract]
pub trait WrapCnetSc:
    owner::OwnerModule
    + storage::StorageModule  {

    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("EGLD")]
    #[endpoint(wrapCnet)]
    fn wrap_cnet(&self) {  
        let payment = self.call_value().egld_value();
        let wrapped_token = self.wrapped_cnet_token().get();
        
        self.send().esdt_local_mint(&wrapped_token.clone().unwrap_esdt(), 0, &payment);
        self.send().direct(&self.blockchain().get_caller(), &wrapped_token, 0, &payment);
    }

    #[payable("*")]
    #[endpoint(unwrapCnet)]
    fn unwrap_cnet(&self) {
        let payment = self.call_value().single_esdt();
        let wrapped_token = self.wrapped_cnet_token().get();

        require!(
            payment.token_identifier == wrapped_token,
            "Invalid token for unwrapping"
        );

        self.send().esdt_local_burn(&wrapped_token.clone().unwrap_esdt(), 0, &payment.amount);

        self.send().direct_egld(&self.blockchain().get_caller(), &payment.amount);
    }
}