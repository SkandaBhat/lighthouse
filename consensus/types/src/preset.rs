use crate::{ChainSpec, Epoch, EthSpec, Unsigned};
use serde::{Deserialize, Serialize};

/// Value-level representation of an Ethereum consensus "preset".
///
/// This should only be used to check consistency of the compile-time constants
/// with a preset YAML file, or to make preset values available to the API. Prefer
/// the constants on `EthSpec` or the fields on `ChainSpec` to constructing and using
/// one of these structs.
///
/// https://github.com/ethereum/eth2.0-specs/blob/dev/presets/mainnet/phase0.yaml
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct BasePreset {
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_committees_per_slot: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub target_committee_size: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_validators_per_committee: u64,
    #[serde(with = "serde_utils::quoted_u8")]
    pub shuffle_round_count: u8,
    #[serde(with = "serde_utils::quoted_u64")]
    pub hysteresis_quotient: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub hysteresis_downward_multiplier: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub hysteresis_upward_multiplier: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub safe_slots_to_update_justified: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_deposit_amount: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_effective_balance: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub effective_balance_increment: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_attestation_inclusion_delay: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub slots_per_epoch: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_seed_lookahead: Epoch,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_seed_lookahead: Epoch,
    #[serde(with = "serde_utils::quoted_u64")]
    pub epochs_per_eth1_voting_period: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub slots_per_historical_root: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_epochs_to_inactivity_penalty: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub epochs_per_historical_vector: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub epochs_per_slashings_vector: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub historical_roots_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub validator_registry_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub base_reward_factor: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub whistleblower_reward_quotient: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub proposer_reward_quotient: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub inactivity_penalty_quotient: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_slashing_penalty_quotient: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub proportional_slashing_multiplier: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_proposer_slashings: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_attester_slashings: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_attestations: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_deposits: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_voluntary_exits: u64,
}

impl BasePreset {
    pub fn from_chain_spec<E: EthSpec>(spec: &ChainSpec) -> Self {
        Self {
            max_committees_per_slot: spec.max_committees_per_slot as u64,
            target_committee_size: spec.target_committee_size as u64,
            max_validators_per_committee: E::MaxValidatorsPerCommittee::to_u64(),
            shuffle_round_count: spec.shuffle_round_count,
            hysteresis_quotient: spec.hysteresis_quotient,
            hysteresis_downward_multiplier: spec.hysteresis_downward_multiplier,
            hysteresis_upward_multiplier: spec.hysteresis_upward_multiplier,
            safe_slots_to_update_justified: spec.safe_slots_to_update_justified,
            min_deposit_amount: spec.min_deposit_amount,
            max_effective_balance: spec.max_effective_balance,
            effective_balance_increment: spec.effective_balance_increment,
            min_attestation_inclusion_delay: spec.min_attestation_inclusion_delay,
            slots_per_epoch: E::SlotsPerEpoch::to_u64(),
            min_seed_lookahead: spec.min_seed_lookahead,
            max_seed_lookahead: spec.max_seed_lookahead,
            epochs_per_eth1_voting_period: E::EpochsPerEth1VotingPeriod::to_u64(),
            slots_per_historical_root: E::SlotsPerHistoricalRoot::to_u64(),
            min_epochs_to_inactivity_penalty: spec.min_epochs_to_inactivity_penalty,
            epochs_per_historical_vector: E::EpochsPerHistoricalVector::to_u64(),
            epochs_per_slashings_vector: E::EpochsPerSlashingsVector::to_u64(),
            historical_roots_limit: E::HistoricalRootsLimit::to_u64(),
            validator_registry_limit: E::ValidatorRegistryLimit::to_u64(),
            base_reward_factor: spec.base_reward_factor,
            whistleblower_reward_quotient: spec.whistleblower_reward_quotient,
            proposer_reward_quotient: spec.proposer_reward_quotient,
            inactivity_penalty_quotient: spec.inactivity_penalty_quotient,
            min_slashing_penalty_quotient: spec.min_slashing_penalty_quotient,
            proportional_slashing_multiplier: spec.proportional_slashing_multiplier,
            max_proposer_slashings: E::MaxProposerSlashings::to_u64(),
            max_attester_slashings: E::MaxAttesterSlashings::to_u64(),
            max_attestations: E::MaxAttestations::to_u64(),
            max_deposits: E::MaxDeposits::to_u64(),
            max_voluntary_exits: E::MaxVoluntaryExits::to_u64(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct AltairPreset {
    #[serde(with = "serde_utils::quoted_u64")]
    pub inactivity_penalty_quotient_altair: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_slashing_penalty_quotient_altair: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub proportional_slashing_multiplier_altair: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub sync_committee_size: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub epochs_per_sync_committee_period: Epoch,
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_sync_committee_participants: u64,
}

impl AltairPreset {
    pub fn from_chain_spec<E: EthSpec>(spec: &ChainSpec) -> Self {
        Self {
            inactivity_penalty_quotient_altair: spec.inactivity_penalty_quotient_altair,
            min_slashing_penalty_quotient_altair: spec.min_slashing_penalty_quotient_altair,
            proportional_slashing_multiplier_altair: spec.proportional_slashing_multiplier_altair,
            sync_committee_size: E::SyncCommitteeSize::to_u64(),
            epochs_per_sync_committee_period: spec.epochs_per_sync_committee_period,
            min_sync_committee_participants: spec.min_sync_committee_participants,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct BellatrixPreset {
    #[serde(with = "serde_utils::quoted_u64")]
    pub inactivity_penalty_quotient_bellatrix: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_slashing_penalty_quotient_bellatrix: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub proportional_slashing_multiplier_bellatrix: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_bytes_per_transaction: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_transactions_per_payload: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub bytes_per_logs_bloom: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_extra_data_bytes: u64,
}

impl BellatrixPreset {
    pub fn from_chain_spec<E: EthSpec>(spec: &ChainSpec) -> Self {
        Self {
            inactivity_penalty_quotient_bellatrix: spec.inactivity_penalty_quotient_bellatrix,
            min_slashing_penalty_quotient_bellatrix: spec.min_slashing_penalty_quotient_bellatrix,
            proportional_slashing_multiplier_bellatrix: spec
                .proportional_slashing_multiplier_bellatrix,
            max_bytes_per_transaction: E::max_bytes_per_transaction() as u64,
            max_transactions_per_payload: E::max_transactions_per_payload() as u64,
            bytes_per_logs_bloom: E::bytes_per_logs_bloom() as u64,
            max_extra_data_bytes: E::max_extra_data_bytes() as u64,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct CapellaPreset {
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_bls_to_execution_changes: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_withdrawals_per_payload: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_validators_per_withdrawals_sweep: u64,
}

impl CapellaPreset {
    pub fn from_chain_spec<E: EthSpec>(spec: &ChainSpec) -> Self {
        Self {
            max_bls_to_execution_changes: E::max_bls_to_execution_changes() as u64,
            max_withdrawals_per_payload: E::max_withdrawals_per_payload() as u64,
            max_validators_per_withdrawals_sweep: spec.max_validators_per_withdrawals_sweep,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct DenebPreset {
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_blobs_per_block: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_blob_commitments_per_block: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub field_elements_per_blob: u64,
}

impl DenebPreset {
    pub fn from_chain_spec<E: EthSpec>(_spec: &ChainSpec) -> Self {
        Self {
            max_blobs_per_block: E::max_blobs_per_block() as u64,
            max_blob_commitments_per_block: E::max_blob_commitments_per_block() as u64,
            field_elements_per_blob: E::field_elements_per_blob() as u64,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ElectraPreset {
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_activation_balance: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_effective_balance_electra: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub min_slashing_penalty_quotient_electra: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub whistleblower_reward_quotient_electra: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_pending_partials_per_withdrawals_sweep: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub pending_balance_deposits_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub pending_partial_withdrawals_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub pending_consolidations_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_consolidations: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_deposit_requests_per_payload: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_attester_slashings_electra: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_attestations_electra: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub max_withdrawal_requests_per_payload: u64,
}

impl ElectraPreset {
    pub fn from_chain_spec<E: EthSpec>(spec: &ChainSpec) -> Self {
        Self {
            min_activation_balance: spec.min_activation_balance,
            max_effective_balance_electra: spec.max_effective_balance_electra,
            min_slashing_penalty_quotient_electra: spec.min_slashing_penalty_quotient_electra,
            whistleblower_reward_quotient_electra: spec.whistleblower_reward_quotient_electra,
            max_pending_partials_per_withdrawals_sweep: spec
                .max_pending_partials_per_withdrawals_sweep,
            pending_balance_deposits_limit: E::pending_balance_deposits_limit() as u64,
            pending_partial_withdrawals_limit: E::pending_partial_withdrawals_limit() as u64,
            pending_consolidations_limit: E::pending_consolidations_limit() as u64,
            max_consolidations: E::max_consolidations() as u64,
            max_deposit_requests_per_payload: E::max_deposit_requests_per_payload() as u64,
            max_attester_slashings_electra: E::max_attester_slashings_electra() as u64,
            max_attestations_electra: E::max_attestations_electra() as u64,
            max_withdrawal_requests_per_payload: E::max_withdrawal_requests_per_payload() as u64,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Eip7594Preset {
    #[serde(with = "serde_utils::quoted_u64")]
    pub field_elements_per_cell: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub field_elements_per_ext_blob: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub kzg_commitments_inclusion_proof_depth: u64,
}

impl Eip7594Preset {
    pub fn from_chain_spec<E: EthSpec>(_spec: &ChainSpec) -> Self {
        Self {
            field_elements_per_cell: E::field_elements_per_cell() as u64,
            field_elements_per_ext_blob: E::field_elements_per_ext_blob() as u64,
            kzg_commitments_inclusion_proof_depth: E::kzg_commitments_inclusion_proof_depth()
                as u64,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{GnosisEthSpec, MainnetEthSpec, MinimalEthSpec};
    use serde::de::DeserializeOwned;
    use std::env;
    use std::fs::File;
    use std::path::PathBuf;

    fn presets_base_path() -> PathBuf {
        env::var("CARGO_MANIFEST_DIR")
            .expect("should know manifest dir")
            .parse::<PathBuf>()
            .expect("should parse manifest dir as path")
            .join("presets")
    }

    fn preset_from_file<T: DeserializeOwned>(preset_name: &str, filename: &str) -> T {
        let f = File::open(presets_base_path().join(preset_name).join(filename))
            .expect("preset file exists");
        serde_yaml::from_reader(f).unwrap()
    }

    fn preset_test<E: EthSpec>() {
        let preset_name = E::spec_name().to_string();
        let spec = E::default_spec();

        let phase0: BasePreset = preset_from_file(&preset_name, "phase0.yaml");
        assert_eq!(phase0, BasePreset::from_chain_spec::<E>(&spec));

        let altair: AltairPreset = preset_from_file(&preset_name, "altair.yaml");
        assert_eq!(altair, AltairPreset::from_chain_spec::<E>(&spec));

        let bellatrix: BellatrixPreset = preset_from_file(&preset_name, "bellatrix.yaml");
        assert_eq!(bellatrix, BellatrixPreset::from_chain_spec::<E>(&spec));

        let capella: CapellaPreset = preset_from_file(&preset_name, "capella.yaml");
        assert_eq!(capella, CapellaPreset::from_chain_spec::<E>(&spec));

        let deneb: DenebPreset = preset_from_file(&preset_name, "deneb.yaml");
        assert_eq!(deneb, DenebPreset::from_chain_spec::<E>(&spec));

        let electra: ElectraPreset = preset_from_file(&preset_name, "electra.yaml");
        assert_eq!(electra, ElectraPreset::from_chain_spec::<E>(&spec));

        let eip7594: Eip7594Preset = preset_from_file(&preset_name, "eip7594.yaml");
        assert_eq!(eip7594, Eip7594Preset::from_chain_spec::<E>(&spec));
    }

    #[test]
    fn mainnet_presets_consistent() {
        preset_test::<MainnetEthSpec>();
    }

    #[test]
    fn gnosis_presets_consistent() {
        preset_test::<GnosisEthSpec>();
    }

    #[test]
    fn minimal_presets_consistent() {
        preset_test::<MinimalEthSpec>();
    }
}
