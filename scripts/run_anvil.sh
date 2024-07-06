#!/bin/bash

anvil --version

anvil \
  --code-size-limit 99999 \
  --disable-block-gas-limit \
  --block-time 5 \
  --fork-chain-id 100 \
  --fork-url https://rpc.gnosis.gateway.fm \
  --fork-block-number 32806304 #34829042 