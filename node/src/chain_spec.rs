// Copyright 2018 Commonwealth Labs, Inc.
// This file is part of Edgeware.

// Edgeware is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Edgeware is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>

use node_primitives::AccountId;
use primitives::{Ed25519AuthorityId as AuthorityId, ed25519};
use edgeware_runtime::{
	Permill, Perbill,
	BalancesConfig, ConsensusConfig, GenesisConfig, ContractConfig, SessionConfig,
	TimestampConfig, TreasuryConfig, StakingConfig, UpgradeKeyConfig, GrandpaConfig,
	IdentityConfig, GovernanceConfig, DelegationConfig, FeesConfig,
	CouncilSeatsConfig, CouncilVotingConfig, DemocracyConfig, IndicesConfig,
};
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;
use substrate_keystore::pad_seed;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialised `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

pub fn edgeware_testnet_config() -> ChainSpec {
	match ChainSpec::from_json_file(std::path::PathBuf::from("testnets/v0.1.4/edgeware.json")) {
		Ok(spec) => spec,
		Err(e) => panic!(e),
	}
}

pub fn edgeware_config_gensis() -> GenesisConfig {
	testnet_genesis(
		vec![(
			hex!["7ef7449d0d0224e0d9cabc66fe29aeff73dc923e10c8e199cb5aab0afb69d0e5"].into(),
			hex!["be3e3264a06a61d9c5c8055807bce41a71e2497257ee72f8745d251429014a2b"].into(),
			hex!["619473a7bd9f608bfdfa93582b53cc8867245e91c9fe5026fee379d47c94dd09"].into(),
		), (
			hex!["ab66295ab4f3015a6108e391181f8ac13e40b437cedfa87983688c7e5065bb70"].into(),
			hex!["01489c5e4c7d0cc8af9fba72c72b95785357e2db50fd8c5ae907ac799a66d9dd"].into(),
			hex!["e48e7a2b1c381a7a0821d61791daaa695bfd070815dd9fe02b51f60f81f0e034"].into(),
		), (
			hex!["8510ba4363ac9a70b34fd586a5a6a1335e3484ec4767617f49db060865e899c4"].into(),
			hex!["83191772bc526b7625ee6ca197a63f984ca10afc2231ad87865d71a6fda0b84d"].into(),
			hex!["d04fa941c18fef1461da631b36766e410ef0017817a06f5c8728e3b23d87f660"].into(),
		), (
			hex!["b30d0b164273c00050d4c2e1eb1cc8be6ade9ac9078abbb692c649c81b4c21b4"].into(),
			hex!["d3739f9e24a0644f34b1301bb490745a1afdfa45bd1c699fbcba0a6723ecc87c"].into(),
			hex!["cf77ff32f8728fd08e9bbd70b0161ba979c55e166de193c17c35a328ecf5cdc2"].into(),
		)],
		hex!["7ef7449d0d0224e0d9cabc66fe29aeff73dc923e10c8e199cb5aab0afb69d0e5"].into(),
		Some(vec![
			hex!["7ef7449d0d0224e0d9cabc66fe29aeff73dc923e10c8e199cb5aab0afb69d0e5"].into(),
			hex!["be3e3264a06a61d9c5c8055807bce41a71e2497257ee72f8745d251429014a2b"].into(),
			hex!["619473a7bd9f608bfdfa93582b53cc8867245e91c9fe5026fee379d47c94dd09"].into(),
			hex!["ab66295ab4f3015a6108e391181f8ac13e40b437cedfa87983688c7e5065bb70"].into(),
			hex!["01489c5e4c7d0cc8af9fba72c72b95785357e2db50fd8c5ae907ac799a66d9dd"].into(),
			hex!["e48e7a2b1c381a7a0821d61791daaa695bfd070815dd9fe02b51f60f81f0e034"].into(),
			hex!["8510ba4363ac9a70b34fd586a5a6a1335e3484ec4767617f49db060865e899c4"].into(),
			hex!["83191772bc526b7625ee6ca197a63f984ca10afc2231ad87865d71a6fda0b84d"].into(),
			hex!["d04fa941c18fef1461da631b36766e410ef0017817a06f5c8728e3b23d87f660"].into(),
			hex!["b30d0b164273c00050d4c2e1eb1cc8be6ade9ac9078abbb692c649c81b4c21b4"].into(),
			hex!["d3739f9e24a0644f34b1301bb490745a1afdfa45bd1c699fbcba0a6723ecc87c"].into(),
			hex!["cf77ff32f8728fd08e9bbd70b0161ba979c55e166de193c17c35a328ecf5cdc2"].into(),
		])
	)
}

/// Edgeware testnet generator
pub fn edgeware_config() -> Result<ChainSpec, String> {
	let boot_nodes = vec![];
	Ok(ChainSpec::from_genesis(
		"Edgeware",
		"edgeware",
		edgeware_config_gensis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
		None,
		None,
		None
	))
}

/// Helper function to generate AuthorityID from seed
pub fn get_authority_id_from_seed(seed: &str) -> AuthorityId {
	let padded_seed = pad_seed(seed);
	// NOTE from ed25519 impl:
	// prefer pkcs#8 unless security doesn't matter -- this is used primarily for tests.
	ed25519::Pair::from_seed(&padded_seed).public().0.into()
}

pub fn get_testnet_pubkeys() -> Vec<AuthorityId> {
	let pubkeys = vec![
		ed25519::Public::from_raw(hex!("df291854c27a22c50322344604076e8b2dc3ffe11dbdcd886adba9e0d6c9f950") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("3bd15363a31eac0e5ecd067731d8a4561185347fc804c50b507025abc29c2ba1") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("65b118b4ae7fe642a59316fc5f0ad9b75cdb9f5ab52733165004f7602755bcfd") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("68128017e34fe40f4ed40f79c24dc7f5a531afc82fc6b71e8092c903627a9133") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("dc746491a214053440d8b9df6774587da105661cc58ed703dc36965359c666a6") as [u8; 32]).into()
	];

	return pubkeys;
}



/// Helper function to generate AuthorityId from seed
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
	let padded_seed = pad_seed(seed);
	// NOTE from ed25519 impl:
	// prefer pkcs#8 unless security doesn't matter -- this is used primarily for tests.
	ed25519::Pair::from_seed(&padded_seed).public().0.into()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuthorityId) {
	let padded_seed = pad_seed(seed);
	// NOTE from ed25519 impl:
	// prefer pkcs#8 unless security doesn't matter -- this is used primarily for tests.
	(
		get_account_id_from_seed(&format!("{}-stash", seed)),
		get_account_id_from_seed(seed),
		ed25519::Pair::from_seed(&padded_seed).public().0.into()
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, AuthorityId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed("Alice"),
			get_account_id_from_seed("Bob"),
			get_account_id_from_seed("Charlie"),
			get_account_id_from_seed("Dave"),
			get_account_id_from_seed("Eve"),
			get_account_id_from_seed("Ferdie"),
		]
	});

	const STASH: u128 = 1 << 20;
	const ENDOWMENT: u128 = 1 << 20;

	GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/edgeware_runtime.compact.wasm").to_vec(),
			authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
		}),
		system: None,
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			existential_deposit: 500,
			transfer_fee: 0,
			creation_fee: 0,
			balances: endowed_accounts.iter().map(|&k| (k.into(), ENDOWMENT)).collect(),
			vesting: vec![],
		}),
		session: Some(SessionConfig {
			validators: initial_authorities.iter().map(|x| x.1.into()).collect(),
			session_length: 10,
			keys: initial_authorities.iter().map(|x| (x.1.clone(), x.2.clone())).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			minimum_validator_count: 1,
			validator_count: 2,
			sessions_per_era: 5,
			bonding_duration: 2 * 60 * 12,
			offline_slash: Perbill::zero(),
			session_reward: Perbill::zero(),
			current_offline_slash: 0,
			current_session_reward: 0,
			offline_slash_grace: 0,
			stakers: initial_authorities.iter().map(|x| (x.0.into(), x.1.into(), STASH)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.1.into()).collect(),
		}),
		democracy: Some(DemocracyConfig {
			launch_period: 9,
			voting_period: 18,
			minimum_deposit: 10,
			public_delay: 0,
			max_lock_periods: 6,
		}),
		council_seats: Some(CouncilSeatsConfig {
			active_council: endowed_accounts.iter()
				.filter(|&endowed| initial_authorities.iter().find(|&(_, controller, _)| controller == endowed).is_none())
				.map(|a| (a.clone().into(), 1000000)).collect(),
			candidacy_bond: 10,
			voter_bond: 2,
			present_slash_per_voter: 1,
			carry_count: 4,
			presentation_duration: 10,
			approval_voting_period: 20,
			term_duration: 1000000,
			desired_seats: (endowed_accounts.len() - initial_authorities.len()) as u32,
			inactive_grace_period: 1,
		}),
		council_voting: Some(CouncilVotingConfig {
			cooloff_period: 75,
			voting_period: 20,
			enact_delay_period: 0,
		}),
		timestamp: Some(TimestampConfig {
			period: 2,                    // 2*2=4 second block time.
		}),
		treasury: Some(TreasuryConfig {
			proposal_bond: Permill::from_percent(5),
			proposal_bond_minimum: 1_000_000,
			spend_period: 12 * 60 * 24,
			burn: Permill::from_percent(50),
		}),
		contract: Some(ContractConfig {
			contract_fee: 21,
			call_base_fee: 135,
			create_base_fee: 175,
			gas_price: 1,
			max_depth: 1024,
			block_gas_limit: 10_000_000,
			current_schedule: Default::default(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
		}),
		fees: Some(FeesConfig {
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
		}),
		upgrade_key: Some(UpgradeKeyConfig {
			key: root_key,
		}),
		identity: Some(IdentityConfig {
			verifiers: get_testnet_pubkeys().iter().map(|x| x.0.into()).collect(),
			expiration_time: 604800, // 7 days
		}),
		governance: Some(GovernanceConfig {
			voting_time: 604800, // 7 days
		}),
		delegation: Some(DelegationConfig {
			delegation_depth: 5,
			_genesis_phantom_data: Default::default(),
		}),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
		],
		get_account_id_from_seed("Alice").into(),
		None,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis("Development", "dev", development_config_genesis, vec![], None, None, None, None)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed("Alice").into(),
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis("Local Testnet", "local_testnet", local_testnet_genesis, vec![], None, None, None, None)
}
