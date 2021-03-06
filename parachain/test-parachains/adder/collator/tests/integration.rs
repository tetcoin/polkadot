// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Tetcoin.

// Tetcoin is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tetcoin is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tetcoin.  If not, see <http://www.gnu.org/licenses/>.

//! Integration test that ensures that we can build and include parachain
//! blocks of the adder parachain.

// If this test is failing, make sure to run all tests with the `real-overseer` feature being enabled.
#[tetcore_test_utils::test]
#[cfg(feature = "real-overseer")]
async fn collating_using_adder_collator(task_executor: tc_service::TaskExecutor) {
	use tp_keyring::AccountKeyring::*;
	use futures::join;
	use tetcoin_primitives::v1::Id as ParaId;

	let mut builder = tc_cli::LoggerBuilder::new("");
	builder.with_colors(false);
	builder.init().expect("Set up logger");

	let para_id = ParaId::from(100);

	// start alice
	let alice = tetcoin_test_service::run_validator_node(task_executor.clone(), Alice, || {}, vec![]);

	// start bob
	let bob = tetcoin_test_service::run_validator_node(
		task_executor.clone(),
		Bob,
		|| {},
		vec![alice.addr.clone()],
	);

	let collator = test_parachain_adder_collator::Collator::new();

	// register parachain
	alice
		.register_parachain(
			para_id,
			collator.validation_code().to_vec(),
			collator.genesis_head(),
		)
		.await
		.unwrap();

	// run the collator node
	let mut charlie = tetcoin_test_service::run_collator_node(
		task_executor.clone(),
		Charlie,
		|| {},
		vec![alice.addr.clone(), bob.addr.clone()],
		collator.collator_id(),
	);

	charlie.register_collator(collator.collator_key(), para_id, collator.create_collation_function()).await;

	// Wait until the parachain has 4 blocks produced.
	collator.wait_for_blocks(4).await;

	join!(
		alice.task_manager.clean_shutdown(),
		bob.task_manager.clean_shutdown(),
		charlie.task_manager.clean_shutdown(),
	);
}
