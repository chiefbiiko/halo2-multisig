
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import { Script } from "forge-std/Script.sol";
import { console } from "forge-std/console.sol";
import { Safe } from "@safe-global/safe-contracts/contracts/Safe.sol";
import { SafeProxy } from "@safe-global/safe-contracts/contracts/proxies/SafeProxy.sol";
import { SafeProxyFactory } from
    "@safe-global/safe-contracts/contracts/proxies/SafeProxyFactory.sol";

contract DeploySafeScript is  Script {
    function run() public {
        uint256 privKey = vm.envUint("PRIVATE_KEY");
        require(privKey != uint256(0), "PRIVATE_KEY is required");
        vm.startBroadcast(privKey);

        address owner = address(0); // TODO: add owner
        address moduleAddress = address(0); // TODO: add module address

        // could also grab safeSingleton & safeProxyFactory from fork
        Safe safeSingleton = new Safe();
        SafeProxyFactory safeProxyFactory = new SafeProxyFactory();

        address[] memory owners = new address[](1);
        owners[0] = owner;

        bytes memory initializer = abi.encodeCall(Safe.setup, (            owners,
            1,
            address(0),
            bytes("0"),
            address(0),
            address(0),
            0,
            payable(address(0))));
        SafeProxy safeProxy = safeProxyFactory.createProxyWithNonce(address(safeSingleton), initializer, 1);

        Safe safe = Safe(payable(address(safeProxy)));
        address safeAddress = address(safeProxy);

        // FIXME: This doesn't work
        // vm.startPrank(safeAddress);
        // safe.enableModule(moduleAddress);
        // vm.stopPrank();

        vm.stopBroadcast();
    }
}