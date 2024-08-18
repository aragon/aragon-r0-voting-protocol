// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

contract RiscVotingProtocolConfig {
    string public config;

    constructor(string memory _config) {
        config = _config;
    }

    function getConfig() public view returns (string memory) {
        return config;
    }

    function setConfig(string memory _config) public {
        config = _config;
    }
}
