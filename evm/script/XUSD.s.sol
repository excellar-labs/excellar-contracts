pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {XUSD} from "../src/XUSD.sol";

contract XUSDScript is Script {
    XUSD public xusd;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        xusd = new XUSD();

        vm.stopBroadcast();
    }
}
