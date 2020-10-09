// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn parse_matches(matches: ArgMatches) -> (bool, Configuration)
{
	let run_as_daemon = matches.is_present(RunAsDaemonName);
	
	let configuration = match matches.value_of_os(ConfigurationName)
	{
		Some(FILE) => match Configuration::from_json_file(FILE)
		{
			Ok(configuration) => configuration,
			Err(error) =>
			{
				eprintln!("The --{} {} {} is invalid: {}", ConfigurationName, ConfigurationValue, FILE.to_string_lossy(), error);
				exit(1);
			}
		},
		
		None => Configuration::default(),
	};
	
	(run_as_daemon, configuration)
}
