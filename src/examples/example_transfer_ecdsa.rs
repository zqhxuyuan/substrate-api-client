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
use keyring::Ed25519Keyring;
use sp_core::crypto::Pair;
use sp_runtime::{MultiAddress, MultiSigner};

use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{Api, XtStatus};
use sp_core::{ecdsa, Public};
use sp_runtime::traits::IdentifyAccount;

pub fn get_from_seed_pair<TPublic: Public>(seed: &str) -> TPublic::Pair {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
}

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();

    // initialize api and set the signer (sender) that is used to sign the extrinsics
    // let from = Ed25519Keyring::Alice.pair();
    // let to = Ed25519Keyring::Bob.to_account_id();

    // from: alice
    //  acc_ecdsa:01e552298e47454041ea31273b4b630c64c104e4514aa3643490b8aaca9cf8ed (5C7C2Z5s...)
    let from = get_from_seed_pair::<ecdsa::Public>("Alice");
    let public_key = from.public();
    let account: MultiSigner = public_key.into();
    let alice = account.into_account();
    println!("alice:{:?}", alice);

    // to: bob
    //  acc_ecdsa:3f6eaf1be5add88d84ca8b02d350074935dbf04f53f4287cb6abfd6b33413f8f (5DVskgSC...)
    let bob_pair = get_from_seed_pair::<ecdsa::Public>("Bob");
    let public_key = bob_pair.public();
    let account: MultiSigner = public_key.into();
    let to = account.into_account();
    println!("bob:{:?}", to);

    let client = WsRpcClient::new(&url);
    let api = Api::new(client)
        .map(|api| api.set_signer(from.clone()))
        .unwrap();

    // 查询Alice/Bob的账户余额
    match api.get_account_data(&alice).unwrap() {
        Some(alice) => println!("[+] Alice's Free Balance is is {}", alice.free),
        None => println!("[+] Alice's Free Balance is is 0"),
    }
    match api.get_account_data(&to).unwrap() {
        Some(bob) => println!("[+] Bob's Free Balance is is {}", bob.free),
        None => println!("[+] Bob's Free Balance is is 0"),
    }

    // 生成转账的交易
    let xt = api.balance_transfer(MultiAddress::Id(to.clone()), 1000);

    println!(
        "Sending an extrinsic from Alice (Key = {}),\n\nto Bob (Key = {})\n",
        from.public(),
        to
    );
    println!("[+] Composed extrinsic: {:?}\n", xt);

    // send and watch extrinsic until finalized
    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);

    // verify that Bob's free Balance increased
    let alice = api.get_account_data(&alice).unwrap().unwrap();
    println!("[+] Alice's Free Balance is now {}", alice.free);
    let bob = api.get_account_data(&to).unwrap().unwrap();
    println!("[+] Bob's Free Balance is now {}", bob.free);
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
