multiversx_sc::imports!();
multiversx_sc::derive_imports!();


#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("wrappedCNETToken")]
    fn wrapped_cnet_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;
}