// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn write_case_folded_labels(out_dir: &OsString, top_level_domains: &Vec<CaseFoldedLabel>) -> io::Result<()>
{
	let mut file = BufWriter::new(File::create(&Path::new(&out_dir).join("EfficientCaseFoldedLabel.top-level-domains.rs"))?);
	
	writeln!(file, "impl EfficientCaseFoldedLabel<'static>")?;
	writeln!(file, "{{")?;
	
		for case_folded_label in top_level_domains
		{
			writeln!(file, "\tconst {}: Self = Self::new(b\"{}\");", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit(), case_folded_label.as_str())?;
		}
	
	writeln!(file, "}}")?;
	
	Ok(())
}
