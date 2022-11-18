# xcm-cli

## Teleport Asset

E.g. teleporting to Canvas parachain on Rococo:

```sh
xcm-cli teleport \
      --url wss://rococo-rpc.polkadot.io:443 \
      --parachain-id 1002 \
      --dest-account <dest public account> \
      --amount 10000000000000 \
      --suri <your private key hex>
```

Example using Alice's account as the sender and destination:
```sh
./target/debug/xcm-cli teleport --url "wss://rococo-rpc.polkadot.io:443" --parachain-id 1002 --dest-account "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d" --amount 10000000000000 --suri "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a"
```
