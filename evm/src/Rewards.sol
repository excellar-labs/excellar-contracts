pragma solidity ^0.8.13;

import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Permit.sol";

contract Rewards {
    address public admin;
    uint256 public rewardRate;
    uint256 public rewardTick;
    mapping(address => uint256) public rewards;
    mapping(address => bool) public kycPassed;
    mapping(address => bool) public isAmm;
    mapping(address => uint256) public balances;
    mapping(address => uint256) public lastCheckpoint;
    mapping(address => uint256) public totalDeposits;
    mapping(address => mapping(address => uint256)) public depositorBalances;

    event RewardCheckpoint(address indexed addr, uint256 reward);
    event RewardClaimed(address indexed addr, uint256 reward);
    event RewardRateSet(uint256 rate);
    event RewardTickSet(uint256 tick);
    event AmmAddressAdded(address indexed addr);
    event AmmAddressRemoved(address indexed addr);

    modifier onlyAdmin() {
        require(msg.sender == admin, "Not admin");
        _;
    }

    modifier onlyKycPassed(address addr) {
        require(kycPassed[addr], "KYC not passed");
        _;
    }

    constructor() {
        admin = msg.sender;
        rewardRate = 10000; // 0.01%
        rewardTick = 28800; // roughly the number of ledger advancements in a day
    }

    function setRewardRate(uint256 rate) public onlyAdmin {
        rewardRate = rate;
        emit RewardRateSet(rate);
    }

    function setRewardTick(uint256 tick) public onlyAdmin {
        rewardTick = tick;
        emit RewardTickSet(tick);
    }

    function addAmmAddress(address addr) public onlyAdmin {
        isAmm[addr] = true;
        emit AmmAddressAdded(addr);
    }

    function removeAmmAddress(address addr) public onlyAdmin {
        isAmm[addr] = false;
        resetReward(addr);
        emit AmmAddressRemoved(addr);
    }

    function checkpointReward(address addr) public {
        if (!kycPassed[addr] && !isAmm[addr]) {
            return;
        }

        uint256 totalReward = calculateReward(addr);
        rewards[addr] += totalReward;
        lastCheckpoint[addr] = block.number;

        if (isAmm[addr]) {
            uint256 totalBalance = totalDeposits[addr];
            if (totalBalance > 0) {
                for (address depositor : getDepositors(addr)) {
                    uint256 reward = calculateAmmRewardShare(totalReward, depositorBalances[addr][depositor], totalBalance);
                    rewards[depositor] += reward;
                }
            }
        }

        emit RewardCheckpoint(addr, totalReward);
    }

    function claimReward(address addr) public onlyKycPassed(addr) {
        require(!isAmm[addr], "AMM addresses cannot claim directly");

        checkpointReward(addr);
        uint256 reward = rewards[addr];
        require(reward > 0, "No reward to claim");

        rewards[addr] = 0;
        balances[addr] += reward;

        emit RewardClaimed(addr, reward);
    }

    function calculateReward(address addr) public view returns (uint256) {
        uint256 blocksHeld = block.number - lastCheckpoint[addr];
        uint256 balance = balances[addr];
        return _calculateReward(blocksHeld, balance, rewardRate, rewardTick);
    }

    function _calculateReward(uint256 blocksHeld, uint256 balance, uint256 rate, uint256 tick) internal pure returns (uint256) {
        uint256 scaleFactor = 1e9;
        uint256 rewardRateFp = (rate * scaleFactor) / 1e8;
        uint256 holdingPeriodFp = (blocksHeld * scaleFactor) / tick;
        uint256 rewardNumerator = balance * rewardRateFp * holdingPeriodFp;
        uint256 roundedNumerator = rewardNumerator + (scaleFactor * scaleFactor / 2);
        return roundedNumerator / (scaleFactor * scaleFactor);
    }

    function calculateAmmRewardShare(uint256 totalReward, uint256 depositorBalance, uint256 totalBalance) internal pure returns (uint256) {
        if (totalBalance == 0) {
            return 0;
        }
        uint256 scaleFactor = 1e6;
        uint256 scaledDepositorBalance = depositorBalance * scaleFactor;
        uint256 participation = scaledDepositorBalance / totalBalance;
        return (totalReward * participation) / scaleFactor;
    }

    function resetReward(address addr) internal {
        rewards[addr] = 0;
        lastCheckpoint[addr] = block.number;
    }

    function getDepositors(address ammAddress) internal view returns (address[] memory) {
        // This function should return the list of depositors for the given AMM address.
        // Implement this function based on your storage structure.
    }
}
