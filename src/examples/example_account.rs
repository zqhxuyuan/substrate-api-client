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
use sp_core::crypto::Pair;
use sp_runtime::{MultiAddress, MultiSigner, AccountId32};

use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{Api, XtStatus, GenericAddress, AccountInfo};
use sp_runtime::traits::IdentifyAccount;

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();
    let client = WsRpcClient::new(&url);

    // let A0: AccountId32 = [0; 32].into();
    // let A1: AccountId32 = [1; 32].into();

    let mut api = Api::new(client).unwrap();
    let signer = AccountKeyring::Alice;
    api.signer = Some(signer.pair());
    println!("[+] Alice's Account Nonce is {}", api.get_nonce().unwrap());

    let origin = signer.to_account_id();
    println!("account:{:?}", origin);

    // account name: One: --: 5Fxune7f71ZbpP2FoY3mhYcmM596Erhv1gRue4nsPwkxMR4n 🔥
    //   -> alice public key: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 🔥 5GrwvaEF...
    // account name: Two: --: 5CUjxa4wVKMj3FqKdqAUf7zcEMr4MYAjXeWmUf44B41neLmJ 🔥
    //   -> bob   public key: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 🔥
    let alice: MultiSigner = AccountKeyring::Alice.pair().public().into();
    let alice_u8: [u8; 32] = alice.into_account().into();

    let bob: MultiSigner = AccountKeyring::Bob.pair().public().into();
    let bob_u8: [u8; 32] = bob.into_account().into();

    let one: MultiSigner = AccountKeyring::One.pair().public().into();
    let one_u8: [u8; 32] = one.into_account().into();

    let two: MultiSigner = AccountKeyring::Two.pair().public().into();
    let two_u8: [u8; 32] = two.into_account().into();

    let xt = api.create_account(one_u8, alice_u8, Some(alice_u8));
    println!("[+] Composed extrinsic: {:?}\n", xt);
    let tx_hash = api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock).unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);

    let xt = api.create_account(two_u8, bob_u8, Some(bob_u8));
    println!("[+] Composed extrinsic: {:?}\n", xt);
    let tx_hash = api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock).unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../../src/examples/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("19944");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
