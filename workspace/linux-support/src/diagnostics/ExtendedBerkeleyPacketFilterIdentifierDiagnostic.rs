// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


trait ExtendedBerkeleyPacketFilterIdentifierDiagnostic: Sized
{
	type BFD: BpfFileDescriptor;
	
	fn map(information: <Self::BFD as BpfFileDescriptor>::Information) -> Self;
	
	#[inline(always)]
	fn gather(file_descriptor: &Self::BFD) -> Result<Self, Errno>
	{
		file_descriptor.get_information().map(Self::map)
	}
	
	fn all() -> Vec<DiagnosticUnobtainableResult<Self>>
	{
		let mut diagnostics = Vec::new();
		
		let mut next = <<<Self::BFD as BpfFileDescriptor>::Information as Information>::Identifier as Identifier>::first();
		while let Some(some_next) = next
		{
			if let Ok(Some(file_descriptor)) = Self::BFD::from_identifier(some_next, Self::BFD::DefaultAccess)
			{
				diagnostics.push(Self::gather(&file_descriptor).map_err(DiagnosticUnobtainable::from));
			}
			next = some_next.next();
		}
		
		diagnostics
	}
	
	/// There is no way to know in advance whether an `absolute_path` represents either of `ExtendedBpfProgramFileDescriptor`, `BpfTypeFormatFileDescriptor`, or `MapFileDescriptor`.
	///
	/// A returned value of `Err()` implies that the file path itself is not one of `ExtendedBpfProgramFileDescriptor`, `BpfTypeFormatFileDescriptor`, or `MapFileDescriptor`, or does not exist or is not a file.
	fn gather_from_pinned_object(absolute_path: &impl AsRef<Path>) -> DiagnosticUnobtainableResult<Option<Self>>
	{
		let file_descriptor = Self::BFD::get_pinned_absolute_path(absolute_path, KernelOnlyAccessPermissions::KernelReadAndWriteUserspaceReadWrite).map_err(|errno| DiagnosticUnobtainable::from(format!("Could not open pinned Extended BPF file descriptor: {}", errno)))?;
		Ok(Self::gather(&file_descriptor).ok())
	}
}
