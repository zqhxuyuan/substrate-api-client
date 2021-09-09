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

    // d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)
    // 88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee (5FA9nQDV...)
    // 01e552298e47454041ea31273b4b630c64c104e4514aa3643490b8aaca9cf8ed (5C7C2Z5s...)
    // 80280ca34c7ad2f3e1642606e04cc55ebee1cbce552f250e85c57b70b2e2625b (5ExjtCWm...)
    let alice1: MultiSigner = AccountKeyring::Alice.pair().public().into();
    let alice2: MultiSigner = Ed25519Keyring::Alice.pair().public().into();
    let alice3: MultiSigner = get_from_seed_pair::<ecdsa::Public>("Alice").public().into();
    let alice4: MultiSigner = get_from_seed_pair::<ecdsa2::Public2>("Alice").public().into();

    // 4afa5b7b13cabc96fe93321322c39f0c6e15cd15145c0decb2968c7bae0b0ae8 (5Dm1mkFh...)
    // let secret_key = hex::decode(
    //     "01e552298e47454041ea31273b4b630c64c104e4514aa3643490b8aaca9cf8ed").unwrap();
    // let alice = ecdsa::Pair::from_seed_slice(&secret_key).unwrap();
    // let alice = alice.public().into();
    // print_info(alice)

    let from = get_from_seed_pair::<ecdsa2::Public2>("Alice");
    let public_key = from.public();
    let private_key = from.seed();
    println!("pubkey:{:?}", public_key);
    println!("prikey:{:?}", hex::encode(private_key));
    let account: MultiSigner = public_key.into();
    let alice = account.into_account();
    println!("account:{:?}", alice);

    let alice = ecdsa2::Pair2::from_seed_slice(&private_key).unwrap();
    let alice = alice.public().into();
    print_info(alice)
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