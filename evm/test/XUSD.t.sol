pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {XUSD} from "../src/XUSD.sol";

contract XUSDTest is Test {
    XUSD public xusd;

    function setUp() public {
        xusd = new XUSD();
        xusd.setNumber(0);
    }

    function test_Increment() public {
        xusd.increment();
        assertEq(xusd.number(), 1);
    }

    function testFuzz_SetNumber(uint256 x) public {
        xusd.setNumber(x);
        assertEq(xusd.number(), x);
    }
}
