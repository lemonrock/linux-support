// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Group identifier diagnostic.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct GroupIdentifierDiagnostic
{
	/// Group identifier.
	pub group_identifier: GroupIdentifier,
	
	/// `/etc/group`.
	pub etc_group_record: DiagnosticUnobtainableResult<EtcGroupRecordDiagnostic>,
}

impl GroupIdentifierDiagnostic
{
	#[inline(always)]
	fn new(etc_path: &EtcPath, group_identifier: GroupIdentifier) -> Self
	{
		Self
		{
			group_identifier,
			etc_group_record: UserAndGroupChoice::etc_group_record_for_group_identifier(etc_path, group_identifier, |etc_group_record| Ok(etc_group_record.into())).map_err(|cause| cause.into()),
		}
	}
}
