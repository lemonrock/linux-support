// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Environment variables and related settings.
#[derive(Deserialize, Serialize)]
pub struct EnvironmentDiagnostic
{
	/// Arguments, including `arg0`.
	pub rust_arguments: Vec<OsString>,

	/// Environment variables.
	pub rust_environment: Vec<(OsString, OsString)>,

	/// Current working directory.
	pub rust_current_working_directory: DiagnosticUnobtainableResult<PathBuf>,

	/// Current executable path.
	pub rust_current_executable_path: DiagnosticUnobtainableResult<PathBuf>,
	
	/// Arguments, including `arg0`.
	pub current_environment: Option<Environment>,
}

impl EnvironmentDiagnostic
{
	fn gather() -> Self
	{
		Self
		{
			rust_arguments: args_os().collect(),
			rust_environment: vars_os().collect(),
			rust_current_working_directory: current_dir().map_err(DiagnosticUnobtainable::from),
			rust_current_executable_path: current_exe().map_err(DiagnosticUnobtainable::from),
			current_environment: Environment::from_environ_pointer(),
		}
	}
}
