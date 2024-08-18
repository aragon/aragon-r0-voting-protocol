//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ControlID, RiscZeroGroth16Verifier} from "risc0/groth16/RiscZeroGroth16Verifier.sol";

import {Counter} from "../contracts/Counter.sol";
import {ERC20} from "../contracts/ERC20.sol";

/// @notice Deployment script for the Counter contract.
/// @dev Use the following environment variable to control the deployment:
///     * ETH_WALLET_PRIVATE_KEY private key of the wallet to be used for deployment.
///
/// See the Foundry documentation for more information about Solidity scripts.
/// https://book.getfoundry.sh/tutorials/solidity-scripting
contract CounterDeploy is Script {
    function run() external {
        uint256 deployerKey = uint256(vm.envBytes32("ETH_WALLET_PRIVATE_KEY"));

        vm.startBroadcast(deployerKey);

        IRiscZeroVerifier verifier = IRiscZeroVerifier(
            0x925d8331ddc0a1F0d96E68CF073DFE1d92b69187
        );
        string memory config = vm.readFile(
            "./script/RiscVotingProtocolConfig.txt"
        );

        console2.logBytes(abi.encode(config));
        Counter counter = new Counter(
            verifier,
            address(0x185Bb1cca668C474214e934028A3e4BB7A5E6525),
            config
        );
        console2.log("Deployed Counter to", address(counter));

        vm.stopBroadcast();
    }
}
