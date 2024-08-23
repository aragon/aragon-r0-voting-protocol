// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.8.24;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";

import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {ERC165Checker} from "@openzeppelin/contracts/utils/introspection/ERC165Checker.sol";
import {IERC20Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";
import {IVotesUpgradeable} from "@openzeppelin/contracts-upgradeable/governance/utils/IVotesUpgradeable.sol";

import {IDAO} from "@aragon/osx-commons/dao/IDAO.sol";
import {PermissionLib} from "@aragon/osx-commons/permission/PermissionLib.sol";
import {IPluginSetup} from "@aragon/osx-commons/plugin/setup/IPluginSetup.sol";
import {PluginUpgradeableSetup} from "@aragon/osx-commons/plugin/setup/PluginUpgradeableSetup.sol";

import {MajorityVotingBase} from "./MajorityVotingBase.sol";
import {RiscVotingProtocolPlugin} from "./RiscVotingProtocolPlugin.sol";

import {ProxyLib} from "@aragon/osx-commons/utils/deployment/ProxyLib.sol";

/// @title RiscVotingProtocolPluginSetup
/// @author Aragon X - 2024
/// @notice The setup contract of the `RiscVotingProtocolPlugin` plugin.
/// @dev v1.0 (Release 1, Build 0)
/// @custom:security-contact sirt@aragon.org
contract RiscVotingProtocolPluginSetup is PluginUpgradeableSetup {
    using Address for address;
    using Clones for address;
    using ERC165Checker for address;
    using ProxyLib for address;

    /// @notice The identifier of the `EXECUTE_PERMISSION` permission.
    /// @dev TODO: Migrate this constant to a common library that can be shared across plugins.
    bytes32 public constant EXECUTE_PERMISSION_ID =
        keccak256("EXECUTE_PERMISSION");

    /// @notice The address of the `plugin` base contract.
    // solhint-disable-next-line immutable-vars-naming
    RiscVotingProtocolPlugin private immutable votingProtocolBase;

    /// @notice The token settings struct.
    /// @param addr The token address. If this is `address(0)`, a new `GovernanceERC20` token is deployed.
    /// If not, the existing token is wrapped as an `GovernanceWrappedERC20`.
    /// @param name The token name. This parameter is only relevant if the token address is `address(0)`.
    /// @param symbol The token symbol. This parameter is only relevant if the token address is `address(0)`.
    struct TokenSettings {
        address addr;
        string name;
        string symbol;
    }

    /// @notice Thrown if token address is passed which is not a token.
    /// @param token The token address
    error TokenNotContract(address token);

    /// @notice Thrown if token address is not ERC20.
    /// @param token The token address
    error TokenNotERC20(address token);

    /// @notice Thrown if passed helpers array is of wrong length.
    /// @param length The array length of passed helpers.
    error WrongHelpersArrayLength(uint256 length);

    /// @notice The contract constructor deploying the plugin implementation contract
    /// and receiving the governance token base contracts to clone from.
    constructor()
        PluginUpgradeableSetup(address(new RiscVotingProtocolPlugin()))
    {
        votingProtocolBase = RiscVotingProtocolPlugin(IMPLEMENTATION);
    }

    /// @inheritdoc IPluginSetup
    function prepareInstallation(
        address _dao,
        bytes calldata _data
    )
        external
        returns (address plugin, PreparedSetupData memory preparedSetupData)
    {
        // Decode `_data` to extract the params needed for deploying and initializing `TokenVoting` plugin,
        // and the required helpers
        (
            MajorityVotingBase.VotingSettings memory votingSettings,
            TokenSettings memory tokenSettings,
            IRiscZeroVerifier verifier
        ) = abi.decode(
                _data,
                (
                    MajorityVotingBase.VotingSettings,
                    TokenSettings,
                    IRiscZeroVerifier
                )
            );

        address token = tokenSettings.addr;
        bool tokenAddressNotZero = token != address(0);

        // Prepare helpers.
        address[] memory helpers = new address[](1);

        if (tokenAddressNotZero) {
            if (!_isContract(token)) {
                revert TokenNotContract(token);
            }

            if (!_isERC20(token)) {
                revert TokenNotERC20(token);
            }
        }

        helpers[0] = token;

        // Prepare and deploy plugin proxy.
        plugin = address(votingProtocolBase).deployUUPSProxy(
            abi.encodeCall(
                RiscVotingProtocolPlugin.initialize,
                (IDAO(_dao), votingSettings, verifier, IERC20Upgradeable(token))
            )
        );

        // Prepare permissions
        PermissionLib.MultiTargetPermission[]
            memory permissions = new PermissionLib.MultiTargetPermission[](2);

        // Set plugin permissions to be granted.
        // Grant the list of permissions of the plugin to the DAO.
        permissions[0] = PermissionLib.MultiTargetPermission({
            operation: PermissionLib.Operation.Grant,
            where: plugin,
            who: _dao,
            condition: PermissionLib.NO_CONDITION,
            permissionId: votingProtocolBase
                .UPDATE_VOTING_SETTINGS_PERMISSION_ID()
        });

        // Grant `EXECUTE_PERMISSION` of the DAO to the plugin.
        permissions[1] = PermissionLib.MultiTargetPermission({
            operation: PermissionLib.Operation.Grant,
            where: _dao,
            who: plugin,
            condition: PermissionLib.NO_CONDITION,
            permissionId: EXECUTE_PERMISSION_ID
        });

        preparedSetupData.helpers = helpers;
        preparedSetupData.permissions = permissions;
    }

    /// @inheritdoc IPluginSetup
    /// @dev Revoke the upgrade plugin permission to the DAO for all builds prior the current one (3).
    function prepareUpdate(
        address _dao,
        uint16 _fromBuild,
        SetupPayload calldata _payload
    )
        external
        view
        override
        returns (
            bytes memory initData,
            PreparedSetupData memory preparedSetupData
        )
    {
        (initData);
        if (_fromBuild < 3) {
            PermissionLib.MultiTargetPermission[]
                memory permissions = new PermissionLib.MultiTargetPermission[](
                    1
                );

            permissions[0] = PermissionLib.MultiTargetPermission({
                operation: PermissionLib.Operation.Revoke,
                where: _payload.plugin,
                who: _dao,
                condition: PermissionLib.NO_CONDITION,
                permissionId: votingProtocolBase.UPGRADE_PLUGIN_PERMISSION_ID()
            });

            preparedSetupData.permissions = permissions;
        }
    }

    /// @inheritdoc IPluginSetup
    function prepareUninstallation(
        address _dao,
        SetupPayload calldata _payload
    )
        external
        view
        returns (PermissionLib.MultiTargetPermission[] memory permissions)
    {
        // Prepare permissions.
        uint256 helperLength = _payload.currentHelpers.length;
        if (helperLength != 1) {
            revert WrongHelpersArrayLength({length: helperLength});
        }

        permissions = new PermissionLib.MultiTargetPermission[](2);

        // Set permissions to be Revoked.
        permissions[0] = PermissionLib.MultiTargetPermission({
            operation: PermissionLib.Operation.Revoke,
            where: _payload.plugin,
            who: _dao,
            condition: PermissionLib.NO_CONDITION,
            permissionId: votingProtocolBase
                .UPDATE_VOTING_SETTINGS_PERMISSION_ID()
        });

        permissions[1] = PermissionLib.MultiTargetPermission({
            operation: PermissionLib.Operation.Revoke,
            where: _dao,
            who: _payload.plugin,
            condition: PermissionLib.NO_CONDITION,
            permissionId: EXECUTE_PERMISSION_ID
        });
    }

    /// @notice Unsatisfiably determines if the contract is an ERC20 token.
    /// @dev It's important to first check whether token is a contract prior to this call.
    /// @param token The token address
    function _isERC20(address token) private view returns (bool) {
        (bool success, bytes memory data) = token.staticcall(
            abi.encodeCall(IERC20Upgradeable.balanceOf, (address(this)))
        );
        return success && data.length == 0x20;
    }

    function _isContract(address account) private view returns (bool) {
        // This method relies on extcodesize/address.code.length, which returns 0
        // for contracts in construction, since the code is only stored at the end
        // of the constructor execution.

        return account.code.length > 0;
    }
}
