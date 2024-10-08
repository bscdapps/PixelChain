// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Primitives for session keys
#![cfg_attr(not(feature = "std"), no_std)]

pub mod vrf;
pub use vrf::*;

/// A Trait to lookup keys from AuthorIds
pub trait KeysLookup<AuthorId, Keys> {
	fn lookup_keys(author: &AuthorId) -> Option<Keys>;
}

// A dummy impl used in simple tests
impl<AuthorId, Keys> KeysLookup<AuthorId, Keys> for () {
	fn lookup_keys(_: &AuthorId) -> Option<Keys> {
		None
	}
}
