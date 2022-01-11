# Soluna Bridge Forwarder

## What does the bridge forwarder do?
This contract can claim the interest it is owed from Pylon protocol and then send the interest over wormhole to the specified address. 

## How to build
`sh reset.sh` to reset, install dependencies, and build

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.4
```

`cargo schema` to update schemas


## Deployed
***Bombay***
contract address: terra1hf37ztxxne8tlv6dmzl6370ndyjg8f7sxm6mkr

## TODO
- [ ] combine this repo with pylon interest redirection because they directly interact