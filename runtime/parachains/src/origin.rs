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

//! Declaration of the parachain specific origin and a noble that hosts it.

use tetcore_std::result;
use tp_runtime::traits::BadOrigin;
use primitives::v1::Id as ParaId;
use tetsy_scale_codec::{Decode, Encode};

/// Origin for the parachains.
#[derive(PartialEq, Eq, Clone, Encode, Decode, tet_core::RuntimeDebug)]
pub enum Origin {
	/// It comes from a parachain.
	Parachain(ParaId),
}

/// Ensure that the origin `o` represents a parachain.
/// Returns `Ok` with the parachain ID that effected the extrinsic or an `Err` otherwise.
pub fn ensure_parachain<OuterOrigin>(o: OuterOrigin) -> result::Result<ParaId, BadOrigin>
	where OuterOrigin: Into<result::Result<Origin, OuterOrigin>>
{
	match o.into() {
		Ok(Origin::Parachain(id)) => Ok(id),
		_ => Err(BadOrigin),
	}
}

/// The origin module.
pub trait Config: fabric_system::Config {}

fabric_support::decl_module! {
	/// There is no way to register an origin type in `construct_runtime` without a noble the origin
	/// belongs to.
	///
	/// This module fulfills only the single purpose of housing the `Origin` in `construct_runtime`.
	///
	// ideally, though, the `construct_runtime` should support a free-standing origin.
	pub struct Module<T: Config> for enum Call where origin: <T as fabric_system::Config>::Origin {}
}

impl From<u32> for Origin {
	fn from(id: u32) -> Origin {
		Origin::Parachain(id.into())
	}
}
