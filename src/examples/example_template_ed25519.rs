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
use sp_runtime::MultiAddress;

use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{Api, XtStatus};

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();
    let client = WsRpcClient::new(&url);

    let mut api = Api::new(client).unwrap();
    let signer = Ed25519Keyring::Alice;
    api.signer = Some(signer.pair());
    println!("[+] Alice's Account Nonce is {}", api.get_nonce().unwrap());

    let origin = signer.to_account_id();
    println!("account:{:?}", origin);

    // let xt = api.do_something(1);
    let xt = api.do_something1(Some(1));
    // let xt = api.do_something0();
    println!("[+] Composed extrinsic: {:?}\n", xt);

    // send and watch extrinsic until finalized
    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}\n", tx_hash);

    // get StorageValue
    let result: u32 = api
        .get_storage_value("TemplateModule", "Something", None)
        .unwrap()
        .or(Some(99))
        .unwrap();
    println!("[+] some value is {:?}", result);
}

pub fn get_node_url_from_cli() -> String {
    let yml = load_yaml!("../../src/examples/cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let node_ip = matches.value_of("node-server").unwrap_or("ws://127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9990");
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
