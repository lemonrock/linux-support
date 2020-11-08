// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn from_iana_comment(file: &mut BufWriter<File>, version_and_updated_at: &str) -> io::Result<()>
{
	writeln!(file, "\t///")?;
	writeln!(file, "\t/// From [IANA](https://data.iana.org/TLD/tlds-alpha-by-domain.txt) as of `{}`.", version_and_updated_at)?;
	Ok(())
}
