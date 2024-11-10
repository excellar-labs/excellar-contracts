// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Initializable} from "openzeppelin-contracts-upgradeable/proxy/utils/Initializable.sol";
import {ERC20Upgradeable} from "openzeppelin-contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import {OwnableUpgradeable} from "openzeppelin-contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ERC20PermitUpgradeable} from "openzeppelin-contracts-upgradeable/token/ERC20/extensions/ERC20PermitUpgradeable.sol";
import {ERC20BurnableUpgradeable} from "openzeppelin-contracts-upgradeable/token/ERC20/extensions/ERC20BurnableUpgradeable.sol";
import {UUPSUpgradeable} from "openzeppelin-contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

contract XUSD is
    Initializable,
    ERC20Upgradeable,
    OwnableUpgradeable,
    ERC20PermitUpgradeable,
    ERC20BurnableUpgradeable,
    UUPSUpgradeable
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address initialOwner) public initializer {
        __Ownable_init(initialOwner);
        __ERC20_init("XUSD Token", "XUSD");
        __ERC20Permit_init("XUSD Token");
        __ERC20Burnable_init();
        __UUPSUpgradeable_init();
    }

    function decimals() public pure override returns (uint8) {
        return 6;
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
