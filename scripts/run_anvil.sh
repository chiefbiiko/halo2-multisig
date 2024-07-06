#!/bin/bash

anvil --version

anvil \
  --code-size-limit 99999 \
  --disable-block-gas-limit \
  --block-time 15 \
  --fork-chain-id 100 \
  --fork-url https://rpc.gnosis.gateway.fm \
  --fork-block-number 34829042 