// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../contracts/AhoyToken.sol";

contract AhoyTokenTest is Test {
    AhoyToken public token;

    uint premint = 100_000_000 * 10 ** 18;

    function setUp() public {
        token = new AhoyToken();
        assertEq(token.balanceOf(address(this)), premint);
    }

    function testMint() public {
        token.mint(address(this), 100);
        assertEq(token.balanceOf(address(this)), premint + 100);
    }
}
