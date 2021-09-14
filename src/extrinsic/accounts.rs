use super::xt_primitives::*;
use crate::extrinsic::CallIndex;
#[cfg(feature = "std")]
use crate::{
    compose_extrinsic, compose_extrinsic_account,
    std::{Api, RpcClient},
};
use sp_core::crypto::Pair;
use sp_runtime::{MultiSignature, MultiSigner, AccountId32};

pub const Accounts: &str = "Accounts";
pub const CreateAccount: &str = "create_account";

pub type CreateAccountFn = (CallIndex, [u8; 32], [u8; 32], Option<[u8; 32]>);
pub type CreateAccountXt = UncheckedExtrinsicV4<CreateAccountFn>;

#[cfg(feature = "std")]
impl<P, Client> Api<P, Client>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
    Client: RpcClient,
{
    pub fn create_account(&self, 
        account_id: [u8; 32],
        pubkey_account_id: [u8; 32],
        owner_pubkey_account_id: Option<[u8; 32]>,    
    ) -> CreateAccountXt {
        compose_extrinsic!(
            self,
            Accounts,
            CreateAccount,
            account_id,
            pubkey_account_id,
            owner_pubkey_account_id
        )
    }
}
