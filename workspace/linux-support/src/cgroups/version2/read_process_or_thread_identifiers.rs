// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn read_process_or_thread_identifiers<Identifier: ParseNumber>(file_path: PathBuf) -> io::Result<Vec<Identifier>>
{
	let reader = file_path.read_raw()?;
	
	const GuessOfRatioOfBytesToIdentifiers: usize = 6;
	let mut identifiers = Vec::with_capacity(reader.len() / GuessOfRatioOfBytesToIdentifiers);
	for line in reader.split_bytes(b'\n')
	{
		let identifier = Identifier::parse_decimal_number(line).map_err(io_error_invalid_data)?;
		identifiers.push(identifier);
	}
	
	identifiers.shrink_to_fit();
	
	Ok(identifiers)
}
