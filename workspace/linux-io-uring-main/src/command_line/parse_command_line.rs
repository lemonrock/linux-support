// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) fn parse_command_line<'a, R>(use_arg_matches: impl FnOnce(ArgMatches<'a>) -> Result<R, ConfigurationError>) -> R
{
	#[allow(deprecated)]
	use_arg_matches
	(
		 App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!("\n"))
		.about("Ultra-high performance socket server using Linux's io_uring")
		.before_help("")
		.after_help("")
		.arg
		(
			Arg::with_name(ConfigurationName)
			.required(false)
			.short("c")
			.long(ConfigurationName)
			.takes_value(true)
			.value_name(ConfigurationValue)
			.allow_hyphen_values(true)
			.empty_values(false)
			.help("Sets a JSON configuration file")
			.validator_os(|FILE|
			{
				if PathBuf::from(FILE).is_file()
				{
					Ok(())
				}
				else
				{
					Err(OsString::from(format!("The --{} {} {} is not a extant, readable and regular file", ConfigurationName, ConfigurationValue, FILE.to_string_lossy())))
				}
			})
		)
		.arg
		(
			Arg::with_name(RunAsDaemonName)
			.short("d")
			.long(RunAsDaemonName)
			.takes_value(false)
			.multiple(false)
			.help("Runs as a daemon")
		)
		.get_matches()
	)
}
