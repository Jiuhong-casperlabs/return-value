# return-value

## Building contract:
_cd contract_

_cargo build --release_

## Deploying Contract:
_casper-client put-deploy --chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jiuhong/my-project/keys/test11/test11_secret_key.pem \
--session-path /home/jiuhong/rust/erc20/my-project/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--payment-amount 30000000000_

## Check the value of result:
_casper-client query-state --node-address http://16.162.124.124:7777 \
-k uref-b5fab412c890d3512bc8bf2763aeb85831fd7da5f27fb67e1912c3e08c6d90c1-007 \
-s ad18ed71f00d3d81771b2e363cff9638d56d392ed365b4332ddd46f49118c2c7_

*-k: the key of "result" under your account:
{
"name":"result"
"key":"uref-b5fab412c890d3512bc8bf2763aeb85831fd7da5f27fb67e1912c3e08c6d90c1-007"
}\*


