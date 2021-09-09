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

///! Very simple example that shows how to use a predefined extrinsic from the extrinsic module
use clap::{load_yaml, App};
use keyring::AccountKeyring;
use keyring::Ed25519Keyring;
use sp_core::crypto::Pair;
use sp_runtime::{MultiAddress, MultiSigner};
use sp_core::{ecdsa, Public, sr25519, ed25519, ecdsa2};

use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{Api, XtStatus};
use sp_runtime::traits::IdentifyAccount;

fn main() {
    env_logger::init();

    let t1 = (1, 2, true, 1.0);
    let (k1, k2, ..) = t1;
    println!("k1:{}", k1);
}

pub fn generate_eth_account_32() {
    let secret_key = hex::decode(
        "502f97299c472b88754accd412b7c9a6062ef3186fba0c0388365e1edec24875").unwrap();
    let alice = ecdsa2::Pair2::from_seed_slice(&secret_key).unwrap();
    let alice = alice.public().into();
    print_info(alice);
}

pub fn get_from_seed_pair<TPublic: Public>(seed: &str) -> TPublic::Pair {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
}

pub fn print_info(account: MultiSigner) {
    let acc = account.into_account();
    println!("account:{:?}", acc);
}