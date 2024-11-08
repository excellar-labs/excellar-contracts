// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {XUSD} from "../src/XUSD.sol";
import {ERC1967Proxy} from "openzeppelin-contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {ERC1967Utils} from "openzeppelin-contracts/proxy/ERC1967/ERC1967Utils.sol";

contract XUSDTest is Test {
    XUSD public implementation;
    XUSD public xusd;
    ERC1967Proxy public proxy;
    address public admin = address(1);
    address public user = address(2);
    address public minter = address(3);
    address public newOwner = address(4);

    function setUp() public {
        implementation = new XUSD();

        proxy = new ERC1967Proxy(address(implementation), abi.encodeCall(implementation.initialize, admin));

        xusd = XUSD(address(proxy));
    }

    function testInitialState() public view {
        assertEq(xusd.name(), "XUSD Token");
        assertEq(xusd.symbol(), "XUSD");
        assertEq(xusd.decimals(), 6);
        assertEq(xusd.owner(), admin);
        assertTrue(xusd.hasRole(xusd.MINTER_ROLE(), admin), "Admin should be a minter by default");
    }

    function testMinterRole() public {
        // Admin can mint by default
        vm.startPrank(admin);
        xusd.mint(user, 500);
        assertEq(xusd.balanceOf(user), 500);
        vm.stopPrank();

        // Initially minter cannot mint
        vm.startPrank(minter);
        vm.expectRevert();
        xusd.mint(user, 1000);
        vm.stopPrank();

        // Admin adds minter
        vm.startPrank(admin);
        xusd.addMinter(minter);
        vm.stopPrank();

        // Now minter can mint
        vm.startPrank(minter);
        xusd.mint(user, 1000);
        assertEq(xusd.balanceOf(user), 1500);
        vm.stopPrank();

        // Admin removes minter
        vm.startPrank(admin);
        xusd.removeMinter(minter);
        vm.stopPrank();

        // Minter can no longer mint
        vm.startPrank(minter);
        vm.expectRevert();
        xusd.mint(user, 1000);
        vm.stopPrank();
    }

    function testTransfer() public {
        vm.startPrank(admin);
        xusd.mint(admin, 1000);

        xusd.transfer(user, 500);
        assertEq(xusd.balanceOf(admin), 500);
        assertEq(xusd.balanceOf(user), 500);
        vm.stopPrank();
    }

    function testPermit() public {
        uint256 ownerPrivateKey = 0x1234;
        address owner = vm.addr(ownerPrivateKey);
        address spender = address(0x789);
        uint256 value = 100;
        uint256 deadline = block.timestamp + 1 hours;

        uint256 nonce = xusd.nonces(owner);

        bytes32 structHash = keccak256(
            abi.encode(
                keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"),
                owner,
                spender,
                value,
                nonce,
                deadline
            )
        );

        bytes32 hash = xusd.DOMAIN_SEPARATOR();
        hash = keccak256(abi.encodePacked("\x19\x01", hash, structHash));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPrivateKey, hash);

        xusd.permit(owner, spender, value, deadline, v, r, s);

        assertEq(xusd.allowance(owner, spender), value);
    }

    function testOwnerChangeAndUpgrade() public {
        // Transfer ownership to new owner
        vm.startPrank(admin);
        xusd.transferOwnership(newOwner);
        vm.stopPrank();

        assertEq(xusd.owner(), newOwner);

        // Deploy new implementation
        XUSD newImplementation = new XUSD();

        // Old owner can't upgrade
        vm.startPrank(admin);
        vm.expectRevert();
        xusd.upgradeToAndCall(address(newImplementation), "");
        vm.stopPrank();

        // New owner can upgrade
        vm.startPrank(newOwner);
        xusd.upgradeToAndCall(address(newImplementation), "");
        vm.stopPrank();

        // Verify state after upgrade
        assertEq(xusd.owner(), newOwner);
        assertEq(xusd.name(), "XUSD Token");
        assertEq(xusd.symbol(), "XUSD");
        assertEq(xusd.decimals(), 6);
    }

    function testUpgradePreservesBalances() public {
        address user1 = address(0x123);
        address user2 = address(0x456);

        vm.startPrank(admin);
        xusd.mint(user1, 1000);
        xusd.mint(user2, 2000);
        vm.stopPrank();

        uint256 user1BalanceBefore = xusd.balanceOf(user1);
        uint256 user2BalanceBefore = xusd.balanceOf(user2);
        uint256 totalSupplyBefore = xusd.totalSupply();

        XUSD newImplementation = new XUSD();
        vm.startPrank(admin);
        xusd.upgradeToAndCall(address(newImplementation), "");
        vm.stopPrank();

        assertEq(xusd.balanceOf(user1), user1BalanceBefore, "User1 balance should be preserved after upgrade");
        assertEq(xusd.balanceOf(user2), user2BalanceBefore, "User2 balance should be preserved after upgrade");
        assertEq(xusd.totalSupply(), totalSupplyBefore, "Total supply should be preserved after upgrade");

        vm.startPrank(user1);
        xusd.transfer(user2, 500);
        vm.stopPrank();

        assertEq(xusd.balanceOf(user1), user1BalanceBefore - 500, "User1 balance should be updated after transfer");
        assertEq(xusd.balanceOf(user2), user2BalanceBefore + 500, "User2 balance should be updated after transfer");
    }

    function testCannotInitializeTwice() public {
        vm.startPrank(admin);
        vm.expectRevert(abi.encodeWithSignature("InvalidInitialization()"));
        xusd.initialize(admin);
        vm.stopPrank();
    }

    function testCannotInitializeImplementation() public {
        vm.startPrank(admin);
        vm.expectRevert(abi.encodeWithSignature("InvalidInitialization()"));
        implementation.initialize(admin);
        vm.stopPrank();
    }

    function testBurn() public {
        vm.startPrank(admin);
        xusd.mint(user, 1000);
        vm.stopPrank();

        vm.startPrank(user);
        xusd.burn(500);
        assertEq(xusd.balanceOf(user), 500);
        vm.stopPrank();
    }

    function testCannotBurnMoreThanBalance() public {
        vm.startPrank(admin);
        xusd.mint(user, 1000);
        vm.stopPrank();

        vm.startPrank(user);
        vm.expectRevert(abi.encodeWithSignature("ERC20InsufficientBalance(address,uint256,uint256)", user, 1000, 1001));
        xusd.burn(1001);
        vm.stopPrank();
    }

    function testBurnFrom() public {
        address spender = address(5);

        vm.startPrank(admin);
        xusd.mint(user, 1000);
        vm.stopPrank();

        vm.startPrank(user);
        xusd.approve(spender, 500);
        vm.stopPrank();

        vm.startPrank(spender);
        xusd.burnFrom(user, 500);
        assertEq(xusd.balanceOf(user), 500);
        assertEq(xusd.allowance(user, spender), 0);
        vm.stopPrank();
    }

    function testCannotBurnFromWithoutAllowance() public {
        address spender = address(5);

        vm.startPrank(admin);
        xusd.mint(user, 1000);
        vm.stopPrank();

        vm.startPrank(spender);
        vm.expectRevert(abi.encodeWithSignature("ERC20InsufficientAllowance(address,uint256,uint256)", spender, 0, 500));
        xusd.burnFrom(user, 500);
        vm.stopPrank();
    }

    function testCannotBurnFromMoreThanAllowance() public {
        address spender = address(5);

        vm.startPrank(admin);
        xusd.mint(user, 1000);
        vm.stopPrank();

        vm.startPrank(user);
        xusd.approve(spender, 500);
        vm.stopPrank();

        vm.startPrank(spender);
        vm.expectRevert(
            abi.encodeWithSignature("ERC20InsufficientAllowance(address,uint256,uint256)", spender, 500, 501)
        );
        xusd.burnFrom(user, 501);
        vm.stopPrank();
    }
}
