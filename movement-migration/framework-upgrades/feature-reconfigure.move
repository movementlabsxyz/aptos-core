// Script hash: 
// Modifying on-chain feature flags:
// Enabled Features: [ConcurrentFungibleBalance, RejectUnstableBytecode]
// Disabled Features: [
//     RemoveDetailedError,
//     PeriodicalRewardRateReduction,
//     VMBinaryFormatV7,
//     KeylessAccounts,
//     KeylessButZklessAccounts,
//     KeylessAccountsWithPasskeys,
// ]

script {
    use aptos_framework::aptos_governance;
    use std::features;
    use std::vector;

    fun main(proposal_id: u64) {
        let core_signer = aptos_governance::get_signer_testnet_only(core_resources, @0000000000000000000000000000000000000000000000000000000000000001);
        let core_address: address = signer::address_of(core_resources);

        let enabled_blob: vector<u64> = vector[58,67];

        let disabled_blob: vector<u64> = vector[
            18,
        ];

        features::change_feature_flags(&framework_signer, enabled_blob, disabled_blob);
    }
}
