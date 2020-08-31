// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileSystemMetadataDiagnostics
{
	pub filesystem_preferred_block_size: u64,
	
	pub filesystem_fragment_block_size: u64,
	
	pub number_of_blocks: u64,
	
	pub number_of_free_blocks: u64,
	
	pub number_of_free_blocks_for_unprivileged_users: u64,
	
	pub number_of_inodes: NumberOfInodes,
	
	pub number_of_free_inodes: NumberOfInodes,
	
	pub number_of_free_inodes_for_unprivileged_users: NumberOfInodes,
	
	pub file_system_identifier: u64,
	
	pub maximum_file_name_length: u64,
	
	pub mount_flags: Option<FileSystemMountFlags>,
}

impl FileSystemMetadataDiagnostics
{
	fn gather(mount_point_file_descriptor: &DirectoryFileDescriptor) -> DiagnosticUnobtainableResult<Self>
	{
		let metadata = mount_point_file_descriptor.underlying_file_system_metadata().map_err(DiagnosticUnobtainable::from)?;
		Ok
		(
			Self
			{
				filesystem_preferred_block_size: metadata.filesystem_preferred_block_size(),
				
				filesystem_fragment_block_size: metadata.filesystem_fragment_block_size(),
				
				number_of_blocks: metadata.number_of_blocks(),
				
				number_of_free_blocks: metadata.number_of_free_blocks(),
				
				number_of_free_blocks_for_unprivileged_users: metadata.number_of_free_blocks_for_unprivileged_users(),
				
				number_of_inodes: metadata.number_of_inodes(),
				
				number_of_free_inodes: metadata.number_of_free_inodes(),
				
				number_of_free_inodes_for_unprivileged_users: metadata.number_of_free_inodes_for_unprivileged_users(),
				
				file_system_identifier: metadata.file_system_identifier(),
				
				maximum_file_name_length: metadata.maximum_file_name_length(),
				
				mount_flags: metadata.mount_flags(),
			}
		)
	}
}
