use super::xt_primitives::*;
use crate::extrinsic::CallIndex;
#[cfg(feature = "std")]
use crate::{
    compose_extrinsic, compose_extrinsic_account,
    std::{Api, RpcClient},
};
use sp_core::crypto::Pair;
use sp_runtime::{MultiSignature, MultiSigner, AccountId32};

pub const System: &str = "System";
pub const Remark: &str = "remark";

pub type RemarkFn = (CallIndex, Vec<u8>);

pub type RemarkXt = UncheckedExtrinsicV4<RemarkFn>;

#[cfg(feature = "std")]
impl<P, Client> Api<P, Client>
    where
        P: Pair,
        MultiSignature: From<P::Signature>,
        MultiSigner: From<P::Public>,
        Client: RpcClient,
{
    pub fn remark(&self, vec: Vec<u8>) -> RemarkXt {
        compose_extrinsic!(
            self,
            System,
            Remark,
            vec
        )
    }
}
