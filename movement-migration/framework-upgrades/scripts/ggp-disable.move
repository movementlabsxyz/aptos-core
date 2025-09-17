script {
    use aptos_framework::aptos_governance;
    use aptos_framework::signer;
    use std::features;
    use std::vector;

    fun enable_enum(core_resources: &signer) {
        let core_signer = aptos_governance::get_signer_testnet_only(
            core_resources,
            @0000000000000000000000000000000000000000000000000000000000000001
        );
        //let core_address: address = signer::address_of(core_resources);

        let enabled_blob: vector<u64> = vector[74];
        let disabled_blob: vector<u64> = vector[0];
        features::change_feature_flags(&core_signer, enabled_blob, disabled_blob);
    }
}
