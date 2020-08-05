// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Get or set the receive flow hash indirection table (RETA) and (or) the hash key.
///
/// For `ETHTOOL_GRSSH`, an `indir_size` of zero and a `key_size` of zero means that only the size should be returned.
///
/// For `ETHTOOL_SRSSH`, an `indir_size` of `ETH_RXFH_INDIR_NO_CHANGE` means that the indirection table setting is not requested.
/// For `ETHTOOL_SRSSH`, an `indir_size` of `0` and `rss_context` of `0` means that the indirection table should be reset to its default values
/// For `ETHTOOL_SRSSH`, an `indir_size` of `0` and `rss_context != 0` means that the specified RSS context should be deleted.
/// An `hfunc` of `0` means that hash function setting is not requested.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ethtool_rxfh
{
	/// Either `ETHTOOL_GRSSH` or `ETHTOOL_SRSSH`.
	pub(crate) cmd: u32,
	
	/// RSS context identifier.
	///
	/// Context 0 is the default for normal traffic.
	/// Other contexts can be referenced as the destination for receive flow classification rules.
	///
	/// The special value `ETH_RXFH_CONTEXT_ALLOC` can be used to create a new context; on return, this will contain the value of the new context's identifier.
	pub(crate) rss_context: Option<ContextIdentifierOrCreate>,
	
	/// On entry, the array size of the user buffer for the indirection table, which may be zero.
	///
	/// For the command `ETHTOOL_SRSSH`, may also be the special value `ETH_RXFH_INDIR_NO_CHANGE` on entry.
	///
	/// For the command `ETHTOOL_GRSSH`, on return, the array size of hardware indirection table.
	pub(crate) indir_size: u32,
	
	/// On entry, the array size of the user buffer for the receive hardware hash key, which may be zero.
	///
	/// For the command `ETHTOOL_GRSSH`, on return, the array size of the receive hardware hash key.
	pub(crate) key_size: u32,
	
	/// Defines the current RSS hash function used by HW (or to be set to).
	///
	/// Bit set of (?more than one)? of `ETH_RSS_HASH`.
	///
	/// If zero then no hash functions are supported.
	pub(crate) hfunc: u8,
	
	rsvd8: [u8; 3],
	
	rsvd32: u32,
	
	/// Receive ring queue index for each hash value ie, indirection table of `indir_size` u32 elements, followed by hash key of `key_size` bytes.
	pub(crate) rss_config: __IncompleteArrayField<u8>,
}

impl EthtoolCommand for ethtool_rxfh
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl VariablySizedEthtoolCommand for ethtool_rxfh
{
	type ArrayElement = u8;
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		let size_in_bytes = self.indirection_table_size_in_bytes() + self.key_size();
		size_in_bytes as u32
	}
}

impl VariablySizedEthtoolCommandWrapper<ethtool_rxfh>
{
	#[inline(always)]
	pub(crate) fn configured_hash_settings(&self) -> Result<ConfiguredHashSettings, UnsupportedHashFunctionError>
	{
		Ok
		(
			ConfiguredHashSettings
			{
				function: self.hash_function()?,
				indirection_table: self.hash_indirection_table().map(|slice| IndirectionTable(slice.to_vec())),
				key: self.hash_key_bytes().map(|slice| HashFunctionKey(slice.to_vec())),
			}
		)
	}
	
	#[inline(always)]
	fn hash_function(&self) -> Result<Option<ETH_RSS_HASH>, UnsupportedHashFunctionError>
	{
		let hash_function_bit_mask = self.hfunc;
		if hash_function_bit_mask == 0
		{
			Ok(None)
		}
		else
		{
			let bit = self.hfunc.trailing_zeros();
			
			if (bit as usize) > ETH_RSS_HASH::ETH_RSS_HASH_COUNT
			{
				Err(UnsupportedHashFunctionError)
			}
			else
			{
				Ok(Some(unsafe { transmute(bit) }))
			}
		}
	}
	
	/// Slices are never empty.
	#[inline(always)]
	fn hash_indirection_table(&self) -> Option<&[QueueIdentifier]>
	{
		let length = self.indir_size();
		if length == 0
		{
			None
		}
		else
		{
			Some(unsafe { from_raw_parts(self.array_start() as *const QueueIdentifier, self.indir_size() ) })
		}
	}
	
	/// Slices are never empty.
	#[inline(always)]
	fn hash_key_bytes(&self) -> Option<&[u8]>
	{
		let length = self.key_size();
		if length == 0
		{
			None
		}
		else
		{
			Some(unsafe { from_raw_parts(self.array_start().add(self.indirection_table_size_in_bytes()), length) })
		}
	}
}

impl ethtool_rxfh
{
	pub(crate) const fn get_indirection_table_size_and_key_size(context_identifier: Option<ContextIdentifier>) -> Self
	{
		Self::get_indirection_table_and_key_data(context_identifier, 0, 0)
	}
	
	pub(crate) const fn reset(context_identifier: Option<ContextIdentifier>) -> Self
	{
		Self::set_indirection_table_and_key_data(context_identifier.map(ContextIdentifierOrCreate::identifier), None, 0, 0)
	}
	
	#[inline(always)]
	pub(crate) fn get_indirection_table_and_key(context_identifier: Option<ContextIdentifier>, indirection_size: usize, key_size: usize) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		Self::new_with_initialized_header_but_uninitialized_array(ethtool_rxfh::get_indirection_table_and_key_data(context_identifier, indirection_size, key_size))
	}
	
	pub(crate) fn set(context_identifier_or_create: Option<ContextIdentifierOrCreate>, configured_hash_settings: &ConfiguredHashSettings) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		let indirection_table = configured_hash_settings.indirection_table.as_ref();
		let indirection_size = indirection_table.map(|vec| vec.len()).unwrap_or(0);
		
		let key = configured_hash_settings.key.as_ref();
		let key_size = key.map(|vec| vec.len()).unwrap_or(0);
		
		let mut this = Self::new_with_initialized_header_but_uninitialized_array(Self::set_indirection_table_and_key_data(context_identifier_or_create, configured_hash_settings.function, indirection_size, key_size));
		
		let array_start = this.array_start_mut();
		let array_start = array_start as *mut QueueIdentifier;
		let indirection_table_end = if let Some(indirection_table) = indirection_table
		{
			unsafe
			{
				array_start.copy_from_nonoverlapping(indirection_table.as_ptr(), indirection_size);
				array_start.add(indirection_size)
			}
		}
		else
		{
			array_start
		};
		
		if let Some(key) = key
		{
			let key_start = indirection_table_end as *mut u8;
			unsafe { key_start.copy_from_nonoverlapping(key.as_ptr(), key_size) }
		}
		
		this
	}
	
	#[inline(always)]
	pub(crate) fn indirection_size_and_key_size(&self) -> (usize, usize)
	{
		(self.indir_size as usize, self.key_size as usize)
	}
	
	const fn get_indirection_table_and_key_data(context_identifier: Option<ContextIdentifier>, indirection_size: usize, key_size: usize) -> Self
	{
		Self
		{
			cmd: ETHTOOL_GRSSH,
			rss_context: context_identifier.map(ContextIdentifierOrCreate::identifier),
			indir_size: indirection_size as u32,
			key_size: key_size as u32,
			hfunc: 0,
			rsvd8: [0; 3],
			rsvd32: 0,
			rss_config: Default::default()
		}
	}
	
	const fn set_indirection_table_and_key_data(context_identifier_or_create: Option<ContextIdentifierOrCreate>, hash_function: Option<ETH_RSS_HASH>, indirection_size: usize, key_size: usize) -> Self
	{
		Self
		{
			cmd: ETHTOOL_SRSSH,
			rss_context: context_identifier_or_create,
			indir_size: indirection_size as u32,
			key_size: key_size as u32,
			hfunc: match hash_function
			{
				None => 0,
				Some(hash_function) => (1 << (hash_function as u8)),
			},
			rsvd8: [0; 3],
			rsvd32: 0,
			rss_config: Default::default()
		}
	}
	
	fn indirection_table_size_in_bytes(&self) -> usize
	{
		const ElementSize: usize = size_of::<Self::ArrayElement>();
		
		self.indir_size() * ElementSize
	}
	
	#[inline(always)]
	fn indir_size(&self) -> usize
	{
		self.indir_size as usize
	}
	
	#[inline(always)]
	fn key_size(&self) -> usize
	{
		self.key_size as usize
	}
}
