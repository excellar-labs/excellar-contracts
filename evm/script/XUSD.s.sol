// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {XUSD} from "../src/XUSD.sol";
import {ERC1967Proxy} from "openzeppelin-contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract XUSDScript is Script {
    function setUp() public {}

    function run() public {
        // Get deployment private key from environment
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(deployerPrivateKey);
        
        // Get network-specific parameters
        address initialOwner = vm.envAddress("INITIAL_OWNER");
        
        console.log("Deploying XUSD with parameters:");
        console.log("Deployer:", deployerAddress);
        console.log("Initial Owner:", initialOwner);

        vm.startBroadcast(deployerPrivateKey);

        // Deploy implementation
        XUSD implementation = new XUSD();
        console.log("Implementation deployed at:", address(implementation));

        // Encode initialization data
        bytes memory initData = abi.encodeWithSelector(
            XUSD.initialize.selector,
            initialOwner
        );

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(implementation),
            initData
        );
        console.log("Proxy deployed at:", address(proxy));

        vm.stopBroadcast();
    }
}
