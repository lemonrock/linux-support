// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User identifier diagnostic.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct UserIdentifierDiagnostic
{
	/// User identifier.
	pub user_identifier: UserIdentifier,
	
	/// `/etc/passwd`.
	pub etc_passwd_record: DiagnosticUnobtainableResult<EtcPasswdRecordDiagnostic>,
}

impl UserIdentifierDiagnostic
{
	#[inline(always)]
	fn new(etc_path: &EtcPath, user_identifier: UserIdentifier) -> Self
	{
		Self
		{
			user_identifier,
			etc_passwd_record: UserAndGroupChoice::etc_passwd_record_for_user_identifier(etc_path, user_identifier, |etc_passwd_record| Ok(etc_passwd_record.into())).map_err(|cause| cause.into()),
		}
	}
}
