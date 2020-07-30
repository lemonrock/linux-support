// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MultipleProgramIdentifiers
{
	#[allow(missing_docs)]
	Generic(ExtendedBpfProgramIdentifier),
	
	#[allow(missing_docs)]
	Native(ExtendedBpfProgramIdentifier),
	
	#[allow(missing_docs)]
	Offloaded(ExtendedBpfProgramIdentifier),
	
	#[allow(missing_docs)]
	GenericAndNative(ExtendedBpfProgramIdentifier, ExtendedBpfProgramIdentifier),
	
	#[allow(missing_docs)]
	GenericAndOffloaded(ExtendedBpfProgramIdentifier, ExtendedBpfProgramIdentifier),
	
	#[allow(missing_docs)]
	NativeAndOffloaded(ExtendedBpfProgramIdentifier, ExtendedBpfProgramIdentifier),
	
	#[allow(missing_docs)]
	GenericAndNativeAndOffloaded(ExtendedBpfProgramIdentifier, ExtendedBpfProgramIdentifier, ExtendedBpfProgramIdentifier),
}

impl MultipleProgramIdentifiers
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn choose_most_performant(self) -> (ExtendedBpfProgramIdentifier, AttachMode)
	{
		use self::MultipleProgramIdentifiers::*;
		
		match self
		{
			Generic(extended_bpf_program_identifier) => (extended_bpf_program_identifier, AttachMode::Generic),
			Native(extended_bpf_program_identifier) => (extended_bpf_program_identifier, AttachMode::Native),
			Offloaded(extended_bpf_program_identifier) => (extended_bpf_program_identifier, AttachMode::Offloaded),
			GenericAndNative(_, extended_bpf_program_identifier) => (extended_bpf_program_identifier, AttachMode::Native),
			GenericAndOffloaded(_, extended_bpf_program_identifier) => (extended_bpf_program_identifier, AttachMode::Offloaded),
			NativeAndOffloaded(_, extended_bpf_program_identifier) => (extended_bpf_program_identifier, AttachMode::Offloaded),
			GenericAndNativeAndOffloaded(_, _, extended_bpf_program_identifier) => (extended_bpf_program_identifier, AttachMode::Offloaded),
		}
	}
}
