use super::xt_primitives::*;
use crate::extrinsic::CallIndex;
#[cfg(feature = "std")]
use crate::{
    compose_extrinsic,
    std::{Api, RpcClient},
};
use sp_core::crypto::Pair;
use sp_runtime::{MultiSignature, MultiSigner};

pub const Template: &str = "TemplateModule";
pub const DoSomething: &str = "do_something";
pub const DoSomething0: &str = "do_something0";

pub type TemplateFn = (CallIndex, u32);
pub type TemplateFn0 = (CallIndex);

pub type TemplateXt = UncheckedExtrinsicV4<TemplateFn>;
pub type TemplateXt0 = UncheckedExtrinsicV4<TemplateFn0>;

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
}
