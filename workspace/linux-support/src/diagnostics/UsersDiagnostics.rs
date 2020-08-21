// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct UsersDiagnostics
{
	pub real: UserIdentifierDiagnostic,
	
	pub effective: UserIdentifierDiagnostic,
	
	pub saved_set: UserIdentifierDiagnostic,
	
	pub audit_login: DiagnosticUnobtainableResult<Option<UserIdentifierDiagnostic>>,
}

impl UsersDiagnostics
{
	fn gather(proc_path: &ProcPath, etc_path: &EtcPath, process_identifier_choice: ProcessIdentifierChoice) -> Self
	{
		let (real, effective, saved_set) = UserIdentifier::current_real_effective_and_saved_set();
		
		Self
		{
			real: UserIdentifierDiagnostic::new(etc_path, real),
			effective: UserIdentifierDiagnostic::new(etc_path, effective),
			saved_set: UserIdentifierDiagnostic::new(etc_path, saved_set),
			audit_login: match UserIdentifier::audit_login(proc_path, process_identifier_choice)
			{
				Err(cause) => Err(DiagnosticUnobtainable::from(cause)),
				Ok(None) => Ok(None),
				Ok(Some(audit_login)) => Ok(Some(UserIdentifierDiagnostic::new(etc_path, audit_login)))
			},
		}
	}
}
