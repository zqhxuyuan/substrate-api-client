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
use sp_runtime::{MultiAddress, MultiSigner, AccountId32};
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

    // AccountId32 的打印：
    //  crate::hexdisplay::HexDisplay::from(&self.0)  -- 对 [u8;32] 的 hex 格式
    //  self.to_ss58check()[0..8]                     -- KW39r9CJ...
    // 注意：虽然下面的 ed25519/sr25519 两种情况下 public key 和 account 打印的值一样，
    // 但并不代表 public key 等于 account，前者 public key 是 Public 类型，后者是 AccountId32 类型
    // 从密码学上而言，public key 可以推导出 account, 即 address, 通常用 ss58 格式
    // 由于 public key 和 account 的打印 Debug 实现，都是调用 to_ss58check_with_version
    // 即打印的是 ss58 格式的地址，所以显示上，public key 和 account 的地址值都一样

    // ecdsa2 的 public key 有 66 char = 33 bytes, account 有 64 char =  32 bytes
    //  pubkey:020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1 (KW39r9CJ...)
    //  prikey:"cb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854"
    //  account:80280ca34c7ad2f3e1642606e04cc55ebee1cbce552f250e85c57b70b2e2625b (5ExjtCWm...)

    // ecdsa 的 public key 和 ecdsa2 的 public key 一样，但是 account 不一样
    //  pubkey:020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1 (KW39r9CJ...)
    //  prikey:"cb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854"
    //  account:01e552298e47454041ea31273b4b630c64c104e4514aa3643490b8aaca9cf8ed (5C7C2Z5s...)

    // ed25519 的 public key 和 account 都是 64 char, 32 bytes，并且 public key = account??
    //  pubkey:88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee (5FA9nQDV...)
    //  prikey:"abf8e5bdbe30c65656c0a3cbd181ff8a56294a69dfedd27982aace4a76909115"
    //  account:88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee (5FA9nQDV...)

    // sr25519
    //  pubkey:d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)
    //  account:d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)
    println!("ecdsa2");
    let from = get_from_seed_pair::<ecdsa2::Public2>("Alice");
    let public_key = from.public();
    let private_key = from.seed();
    println!(" pubkey:{:?}", public_key);
    println!(" prikey:{:?}", hex::encode(private_key));
    let account: MultiSigner = public_key.into();
    let alice = account.into_account();
    println!(" account:{:?}", alice);
    // let alice = ecdsa2::Pair2::from_seed_slice(&private_key).unwrap();
    // let alice = alice.public().into();
    // print_info(alice)

    println!("ecdsa");
    let from = get_from_seed_pair::<ecdsa::Public>("Alice");
    let public_key = from.public();
    let private_key = from.seed();
    println!(" pubkey:{:?}", public_key);
    println!(" prikey:{:?}", hex::encode(private_key));
    let account: MultiSigner = public_key.into();
    let alice = account.into_account();
    println!(" account:{:?}", alice);

    println!("ed25519");
    let from = get_from_seed_pair::<ed25519::Public>("Alice");
    let public_key = from.public();
    let private_key = from.seed();
    println!(" pubkey:{:?}", public_key);
    println!(" prikey:{:?}", hex::encode(private_key));
    let account: MultiSigner = public_key.into();
    let alice = account.into_account();
    println!(" account:{:?}", alice);

    println!("sr25519");
    let from = get_from_seed_pair::<sr25519::Public>("Alice");
    let public_key = from.public();
    // let private_key = from.seed();
    println!(" pubkey:{:?}", public_key);
    // println!(" prikey:{:?}", hex::encode(private_key));
    let account: MultiSigner = public_key.into();
    let alice = account.into_account();
    println!(" account:{:?}", alice);

    println!("--------------------2");
    generate_mock_account_name_address();

    println!("--------------------1");
    let one: MultiSigner = AccountKeyring::One.pair().public().into();
    let account: AccountId32 = one.into_account();
    println!("one:{}", account); // 5Fxune7f71ZbpP2FoY3mhYcmM596Erhv1gRue4nsPwkxMR4n

    let two: MultiSigner = AccountKeyring::Two.pair().public().into();
    let account: AccountId32 = two.into_account();
    println!("one:{}", account); // 5CUjxa4wVKMj3FqKdqAUf7zcEMr4MYAjXeWmUf44B41neLmJ
}

// idx:0, 5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM
// idx:1, 5C62Ck4UrFPiBtoCmeSrgF7x9yv9mn38446dhCpsi2mLHiFT
// idx:2, 5C7LYpP2ZH3tpKbvVvwiVe54AapxErdPBbvkYhe6y9ZBkqWt
// idx:3, 5C8etthaGJi5SkQeEDSaK32ABBjkhwDeK9ksQCTLEGM3EH14
// idx:4, 5C9yEy27yLNG5BDMxVwS8RyGBneZB1ouShazFhGZVP8thK5z
pub fn generate_mock_account_name_address() {
    for idx in 0..5u8 {
        let arr = [idx; 32];
        let account: AccountId32 = arr.into();
        println!("idx:{}, {}", idx, account);
    }
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