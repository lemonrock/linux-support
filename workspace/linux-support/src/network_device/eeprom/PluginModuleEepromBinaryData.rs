// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// EEPROM binary data for SFP, SFP+, QFSP, QFSP+, QFSP28 and similar plugin modules.
///
/// `SFF` specifications are at <https://www.snia.org/technology-communities/sff/specifications>.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum PluginModuleEepromBinaryData
{
	/// `SFF-8079` EEPROM layout of page at address `A0h`.
	///
	/// See <https://members.snia.org/document/dl/26164>.
	#[serde(rename = "SFF-8079")] SFF_8079
	{
		/// Page at address `A0h`.
		#[serde(flatten)] page_A0: BinaryData256,
	},
	
	/// `SFF-8472` EEPROM layout, consisting of a page at address `A0h` with the same layout as `SFF-8079` as above and then a page at address `A2h`.
	///
	/// An extended form of `SFF-8079`.
	///
	/// The second page is for diagnostic monitoring and might not be available, eg for Mellanox `mlxsw` SFP.
	///
	/// See <https://members.snia.org/document/dl/X> at
	#[serde(rename = "SFF-8472")] SFF_8472
	{
		/// Page at address `A0h`.
		#[serde(flatten)] page_A0: BinaryData256,
		
		/// Page at address `A2h`.
		#[serde(flatten)] page_A2: Option<BinaryData256>,
	},
	
	/// `SFF-8436` EEPROM layout of page at address `A0h`.
	///
	/// See <https://members.snia.org/document/dl/X>.
	#[serde(rename = "SFF-8436")] SFF_8436
	{
		/// Page at address `A0h`.
		#[serde(flatten)] page_A0: BinaryData256,
	},
	
	/// `SFF-8436` EEPROM layout 640 byte variant used for:-
	///
	/// * Intel `i40e` QFSP+.
	/// * Intel `ice` QFSP+.
	/// * Intel `ice` QFSP28.
	/// * Mellanox `mlx5` QFSP.
	/// * Mellanox `mlx5` QFSP+.
	/// * Mellanox `mlx5` QFSP28.
	/// * Mellanox `mlxsw` (?switch) QFSP.
	/// * Mellanox `mlxsw` (?switch) QFSP+.
	/// * Mellanox `mlxsw` (?switch) QFSP28.
	#[serde(rename = "SFF-8436 640")] SFF_8436_640
	{
		/// Binary data.
		#[serde(flatten)] binary_data: BinaryData640,
	},
	
	/// `SFF-8636` EEPROM layout of page at address `A0h`.
	///
	/// Sometimes considered to be revision 0x03 or greater of `SFF-8436`.
	///
	/// See <https://members.snia.org/document/dl/X>.
	#[serde(rename = "SFF-8636")] SFF_8636
	{
		/// Page at address `A0h`.
		page_A0: BinaryData256,
	},
	
	/// `SFF-8636` EEPROM layout 640 byte variant used for:-
	///
	/// * Intel `i40e` QFSP+.
	/// * Intel `i40e` QFSP28.
	/// * Intel `ice` QFSP+.
	/// * Intel `ice` QFSP28.
	/// * Mellanox `mlx5` QFSP+.
	/// * Mellanox `mlx5` QFSP28.
	/// * Mellanox `mlxsw` (?switch) QFSP+.
	/// * Mellanox `mlxsw` (?switch) QFSP28.
	#[serde(rename = "SFF-8636 640")] SFF_8636_640
	{
		/// Binary data.
		#[serde(flatten)] binary_data: BinaryData640,
	},

	/// An unknown or bad implementation by a driver.
	Unknown
	{
		/// Only other known type is `0x0`, used by the `ionic` driver for an unknown EEPROM.
		type_: u32,
		
		/// For the `ionic` variant above, this is 256 bytes.
		binary_data: Box<[u8]>
	}
}
