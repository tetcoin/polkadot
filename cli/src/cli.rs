// Copyright 2017-2020 Parity Technologies (UK) Ltd.
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

//! Tetcoin CLI library.

use structopt::StructOpt;

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Build a chain specification.
	BuildSpec(tc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(tc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(tc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(tc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(tc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(tc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(tc_cli::RevertCmd),

	#[allow(missing_docs)]
	#[structopt(name = "validation-worker", setting = structopt::clap::AppSettings::Hidden)]
	ValidationWorker(ValidationWorkerCommand),

	/// The custom benchmark subcommmand benchmarking runtime nobles.
	#[structopt(
		name = "benchmark",
		about = "Benchmark runtime nobles."
	)]
	Benchmark(fabric_benchmarking_cli::BenchmarkCmd),

	/// Key management cli utilities
	Key(tc_cli::KeySubcommand),
}

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct ValidationWorkerCommand {
	#[allow(missing_docs)]
	pub mem_id: String,
}

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct RunCmd {
	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub base: tc_cli::RunCmd,

	/// Force using Metrocoin native runtime.
	#[structopt(long = "force-metrocoin")]
	pub force_metrocoin: bool,

	/// Force using Westend native runtime.
	#[structopt(long = "force-westend")]
	pub force_westend: bool,

	/// Force using Rococo native runtime.
	#[structopt(long = "force-rococo")]
	pub force_rococo: bool,

	/// Setup a GRANDPA scheduled voting pause.
	///
	/// This parameter takes two values, namely a block number and a delay (in
	/// blocks). After the given block number is finalized the GRANDPA voter
	/// will temporarily stop voting for new blocks until the given delay has
	/// elapsed (i.e. until a block at height `pause_block + delay` is imported).
	#[structopt(long = "grandpa-pause", number_of_values(2))]
	pub grandpa_pause: Vec<u32>,

	/// Add the destination address to the jaeger agent.
	///
	/// Must be valid socket address, of format `IP:Port`
	/// commonly `127.0.0.1:6831`.
	#[structopt(long)]
	pub jaeger_agent: Option<std::net::SocketAddr>,
}

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,
	#[structopt(flatten)]
	pub run: RunCmd,
}
