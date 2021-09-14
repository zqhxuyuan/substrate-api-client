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
pub const CreateOwnerPerm: &str = "create_owner_permission_auth";

pub type CreateAccountFn = (CallIndex, [u8; 32], [u8; 32], Option<[u8; 32]>);
pub type CreateAccountXt = UncheckedExtrinsicV4<CreateAccountFn>;

pub type CreateOwnerPermFn = (CallIndex, [u8; 32]);
pub type CreateOwnerPermXt = UncheckedExtrinsicV4<CreateOwnerPermFn>;

#[cfg(feature = "std")]
impl<P, Client> Api<P, Client>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
    Client: RpcClient,
{
    pub fn create_account(&self, 
        account_name: [u8; 32],
        pubkey_account_id: [u8; 32],
        active_pubkey_account_id: Option<[u8; 32]>,
    ) -> CreateAccountXt {
        compose_extrinsic!(
            self,
            Accounts,
            CreateAccount,
            account_name,
            pubkey_account_id,
            active_pubkey_account_id
        )
    }

    pub fn create_owner_account_permission(&self,
                          account_name: [u8; 32],
    ) -> CreateOwnerPermXt {
        compose_extrinsic!(
            self,
            Accounts,
            CreateOwnerPerm,
            account_name
        )
    }
}
