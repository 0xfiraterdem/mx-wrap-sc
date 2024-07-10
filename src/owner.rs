multiversx_sc::imports!();
multiversx_sc::derive_imports!();


#[multiversx_sc::module]
pub trait OwnerModule: crate::storage::StorageModule {

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self, token_id:EgldOrEsdtTokenIdentifier, amount: BigUint){
        let owner = self.blockchain().get_owner_address();
        let _contract_balance = self.blockchain().get_sc_balance(&token_id, 0);
        self.send().direct(&owner, &token_id, 0, &amount);
    }

    #[only_owner]
    #[endpoint(setWrappedCNETToken)]
    fn set_wrapped_cnet_token(&self, token_id: EgldOrEsdtTokenIdentifier) {
        self.wrapped_cnet_token().set(&token_id);
    }
}

