// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn write_case_folded_names(out_dir: &OsString, top_level_domains: &Vec<CaseFoldedLabel>, version_and_updated_at: &str) -> io::Result<()>
{
	let mut writer = new_buf_writer(out_dir, "EfficientCaseFoldedName.top_level_domains.rs")?;
	
	writeln!(writer, "impl EfficientCaseFoldedName<'static>")?;
	writeln!(writer, "{{")?;
	
		from_iana_comment(&mut writer, version_and_updated_at)?;
		writeln!(writer, "\t#[inline(always)]")?;
		writeln!(writer, "\tpub fn top_level_domain_names() -> &'static HashSet<Self>")?;
		writeln!(writer, "\t{{")?;
			writeln!(writer, "\t\tlazy_static!")?;
			writeln!(writer, "\t\t{{")?;
				writeln!(writer, "\t\t\tstatic ref top_level_domain_names: HashSet<EfficientCaseFoldedName<'static>> = fast_secure_hash_set!")?;
				writeln!(writer, "\t\t\t{{")?;
				for case_folded_label in top_level_domains
				{
					writeln!(writer, "\t\t\t\tEfficientCaseFoldedName::{}().clone(),", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
				}
				writeln!(writer, "\t\t\t}};")?;
				writeln!(writer, "\t\t\t")?;
			writeln!(writer, "\t\t}}")?;
			writeln!(writer, "\t\t&top_level_domain_names")?;
		writeln!(writer, "\t}}")?;
	
		for case_folded_label in top_level_domains
		{
			writeln!(writer)?;
			writeln!(writer, "\t/// `{}.`", case_folded_label.as_str())?;
			from_iana_comment(&mut writer, version_and_updated_at)?;
			writeln!(writer, "\t#[inline(always)]")?;
			writeln!(writer, "\tpub fn {}() -> &'static Self", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
			writeln!(writer, "\t{{")?;
			writeln!(writer, "\t\tlazy_static!")?;
			writeln!(writer, "\t\t{{")?;
				writeln!(writer, "\t\t\tstatic ref {}: EfficientCaseFoldedName<'static> = EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::{});", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit(), case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
			writeln!(writer, "\t\t}}")?;
			writeln!(writer, "\t\t")?;
			writeln!(writer, "\t\t&{}", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
			writeln!(writer, "\t}}")?;
		}
	
	writeln!(writer, "}}")?;
	
	Ok(())
}
