/*
   Copyright 2019 Supercomputing Systems AG

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

*/

//! Offers macros that build extrinsics for custom runtime modules based on the metadata.
//! Additionally, some predefined extrinsics for common runtime modules are implemented.

#[cfg(feature = "std")]
pub extern crate codec;
#[cfg(feature = "std")]
pub extern crate log;

#[cfg(feature = "std")]
pub mod template;
#[cfg(feature = "std")]
pub mod balances;
#[cfg(feature = "std")]
pub mod system;
#[cfg(feature = "std")]
pub mod accounts;
#[cfg(feature = "std")]
pub mod contract;
#[cfg(feature = "staking-xt")]
pub mod staking;
pub mod xt_primitives;

pub mod mock_data;

pub type CallIndex = [u8; 2];

/// Generates the extrinsic's call field for a given module and call passed as &str
/// # Arguments
///
/// * 'node_metadata' - This crate's parsed node metadata as field of the API.
/// * 'module' - Module name as &str for which the call is composed.
/// * 'call' - Call name as &str
/// * 'args' - Optional sequence of arguments of the call. They are not checked against the metadata.
/// As of now the user needs to check himself that the correct arguments are supplied.
#[macro_export]
macro_rules! compose_call {
($node_metadata: expr, $module: expr, $call_name: expr $(, $args: expr) *) => {
        {
            let module = $node_metadata.module_with_calls($module).unwrap().to_owned();

            let call_index = module.calls.get($call_name).unwrap();

            // CallIndex                         , parameters
            ([module.index, *call_index as u8] $(, ($args)) *)
        }
    };
}

// Generates an Unchecked extrinsic for a given call
// # Arguments
//
// * 'signer' - AccountKey that is used to sign the extrinsic.
// * 'call' - call as returned by the compose_call! macro or via substrate's call enums.
// * 'nonce' - signer's account nonce: u32
// * 'era' - Era for extrinsic to be valid
// * 'genesis_hash' - sp-runtime::Hash256/[u8; 32].
// * 'runtime_spec_version' - RuntimeVersion.spec_version/u32
#[macro_export]
macro_rules! compose_extrinsic_offline {
    ($signer: expr,
    $call: expr,
    $nonce: expr,
    $era: expr,
    $genesis_hash: expr,
    $genesis_or_current_hash: expr,
    $runtime_spec_version: expr,
    $transaction_version: expr) => {{
        use $crate::extrinsic::xt_primitives::*;
        use $crate::sp_runtime::generic::Era;
        use $crate::sp_runtime::traits::IdentifyAccount;
        use $crate::sp_runtime::traits::{BlakeTwo256, Hash};
        use $crate::sp_runtime::MultiSigner;

        let extra = GenericExtra::new($era, $nonce);
        let raw_payload = SignedPayload::from_raw(
            $call.clone(),
            extra.clone(),
            (
                $runtime_spec_version,
                $transaction_version,
                $genesis_hash,
                $genesis_or_current_hash,
                (),
                (),
                (),
                // ()
                // [0u8; 32]
            ),
        );

        let signature = raw_payload.using_encoded(|payload| {
            // println!("payload:{}", hex::encode(payload));
            // println!("payload:{:?}", BlakeTwo256::hash_of(&payload).as_fixed_bytes());
            $signer.sign(payload)
        });

        let multi_signer: MultiSigner = $signer.public().into();

        UncheckedExtrinsicV4::new_signed(
            $call,
            GenericAddress::from(multi_signer.into_account()),
            signature.into(),
            extra,
        )
    }};
}

#[macro_export]
macro_rules! compose_extrinsic_offline_account {
    ($signer: expr,
    $call: expr,
    $nonce: expr,
    $era: expr,
    $genesis_hash: expr,
    $genesis_or_current_hash: expr,
    $runtime_spec_version: expr,
    $transaction_version: expr,
    $account: expr
    ) => {{
        use $crate::extrinsic::xt_primitives::*;
        use $crate::sp_runtime::generic::Era;
        use $crate::sp_runtime::traits::IdentifyAccount;
        use $crate::sp_runtime::traits::{BlakeTwo256, Hash};
        use $crate::sp_runtime::MultiSigner;

        // the account is Pair type, passing Address type to new_signed2()
        let operator_signer: MultiSigner = $account.public().into();
        let account_id = operator_signer.into_account();
        let account_addr = GenericAddress::from(account_id.clone());
        let account_u8: [u8; 32] = account_id.into();

        let extra = GenericExtra::new_account($era, $nonce, account_u8);
        // let extra = GenericExtra::new($era, $nonce);
        let raw_payload = SignedPayload::from_raw(
            $call.clone(),
            extra.clone(),
            (
                $runtime_spec_version,
                $transaction_version,
                $genesis_hash,
                $genesis_or_current_hash,
                (),
                (),
                (),
                // ()
                // $account
            ),
        );

        let signature = raw_payload.using_encoded(|payload| {
            // println!("payload:{}", hex::encode(payload));
            // println!("payload:{:?}", BlakeTwo256::hash_of(&payload).as_fixed_bytes());
            $signer.sign(payload)
        });

        let multi_signer: MultiSigner = $signer.public().into();

        UncheckedExtrinsicV4::new_signed2(
            $call,
            GenericAddress::from(multi_signer.into_account()),
            signature.into(),
            extra,
            account_addr,
        )
    }};
}

// Generates an Unchecked extrinsic for a given module and call passed as a &str.
// # Arguments
//
// * 'api' - This instance of API. If the *signer* field is not set, an unsigned extrinsic will be generated.
// * 'module' - Module name as &str for which the call is composed.
// * 'call' - Call name as &str
// * 'args' - Optional sequence of arguments of the call. They are not checked against the metadata.
// As of now the user needs to check himself that the correct arguments are supplied.
#[macro_export]
#[cfg(feature = "std")]
macro_rules! compose_extrinsic {
	($api: expr,
	$module: expr,
	$call: expr
	$(, $args: expr) *) => {
		{
            #[allow(unused_imports)] // For when extrinsic does not use Compact
            use $crate::extrinsic::codec::Compact;
            use $crate::extrinsic::log::info;
            use $crate::extrinsic::xt_primitives::*;

            info!("Composing generic extrinsic for module {:?} and call {:?}", $module, $call);
            let call = $crate::compose_call!($api.metadata.clone(), $module, $call $(, ($args)) *);

            if let Some(signer) = $api.signer.clone() {
                $crate::compose_extrinsic_offline!(
                    signer,
                    call.clone(),
                    $api.get_nonce().unwrap(),
                    Era::Immortal,
                    $api.genesis_hash,
                    $api.genesis_hash,
                    $api.runtime_version.spec_version,
                    $api.runtime_version.transaction_version
                )
            } else {
                UncheckedExtrinsicV4 {
                    signature: None,
                    function: call.clone(),
                    operator: None,
                }
            }
		}
    };
}

#[macro_export]
#[cfg(feature = "std")]
macro_rules! compose_extrinsic_account {
	($api: expr,
	$module: expr,
	$call: expr,
	$account: expr
	$(, $args: expr) *) => {
		{
            #[allow(unused_imports)] // For when extrinsic does not use Compact
            use $crate::extrinsic::codec::Compact;
            use $crate::extrinsic::log::info;
            use $crate::extrinsic::xt_primitives::*;

            info!("Composing generic extrinsic for module {:?} and call {:?}", $module, $call);
            let call = $crate::compose_call!($api.metadata.clone(), $module, $call $(, ($args)) *);

            if let Some(signer) = $api.signer.clone() {
                $crate::compose_extrinsic_offline_account!(
                    signer,
                    call.clone(),
                    $api.get_nonce().unwrap(),
                    Era::Immortal,
                    $api.genesis_hash,
                    $api.genesis_hash,
                    $api.runtime_version.spec_version,
                    $api.runtime_version.transaction_version,
                    $account
                )
            } else {
                UncheckedExtrinsicV4 {
                    signature: None,
                    function: call.clone(),
                    operator: None,
                }
            }
		}
    };
}
