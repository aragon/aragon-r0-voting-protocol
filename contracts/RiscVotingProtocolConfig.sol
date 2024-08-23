// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.24;

contract RiscVotingProtocolConfig {
    string public config;

    event ConfigSet(string config);

    function getConfig() public view returns (string memory) {
        return config;
    }

    function setRiscVotingProtocolConfig(string memory _config) internal {
        config = _config;

        emit ConfigSet(_config);
    }
}
