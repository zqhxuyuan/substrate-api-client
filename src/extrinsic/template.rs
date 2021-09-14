use super::xt_primitives::*;
use crate::extrinsic::CallIndex;
#[cfg(feature = "std")]
use crate::{
    compose_extrinsic, compose_extrinsic_account,
    std::{Api, RpcClient},
};
use sp_core::crypto::Pair;
use sp_runtime::{MultiSignature, MultiSigner, AccountId32};

pub const Template: &str = "TemplateModule";
pub const DoSomething: &str = "do_something";
pub const DoSomething0: &str = "do_something0";
pub const DoSomething1: &str = "do_something1";

pub type TemplateFn = (CallIndex, u32);
pub type TemplateFn0 = (CallIndex);
pub type TemplateFn1 = (CallIndex, u32);

pub type TemplateXt = UncheckedExtrinsicV4<TemplateFn>;
pub type TemplateXt0 = UncheckedExtrinsicV4<TemplateFn0>;
pub type TemplateXt1 = UncheckedExtrinsicV4<TemplateFn1>;

#[cfg(feature = "std")]
impl<P, Client> Api<P, Client>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
    Client: RpcClient,
{
    pub fn do_something(&self, amount: u32) -> TemplateXt {
        compose_extrinsic!(
            self,
            Template,
            DoSomething,
            amount
        )
    }

    pub fn do_something0(&self) -> TemplateXt0 {
        compose_extrinsic!(
            self,
            Template,
            DoSomething0
        )
    }

    // pub fn do_something1(&self, accountId: [u8; 32], amount: u32) -> TemplateXt1 {
    //     compose_extrinsic_account!(
    //         self,
    //         Template,
    //         DoSomething1,
    //         accountId,
    //         amount
    //     )
    // }

    pub fn do_something1(&self, operator: P, amount: u32) -> TemplateXt1 {
        compose_extrinsic_account!(
            self,
            Template,
            DoSomething1,
            operator,
            amount
        )
    }
}
