// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn write_case_folded_names(out_dir: &OsString, top_level_domains: &Vec<CaseFoldedLabel>, version_and_updated_at: &str) -> io::Result<()>
{
	let mut file = BufWriter::new(File::create(&Path::new(&out_dir).join("EfficientCaseFoldedName.top-level-domains.rs"))?);
	writeln!(file, "impl EfficientCaseFoldedName<'static>")?;
	writeln!(file, "{{")?;
	
		from_iana_comment(&mut file, version_and_updated_at)?;
		writeln!(file, "\t#[inline(always)]")?;
		writeln!(file, "\tpub fn top_level_domain_names() -> &'static HashSet<Self>")?;
		writeln!(file, "\t{{")?;
			writeln!(file, "\t\tlazy_static!")?;
			writeln!(file, "\t\t{{")?;
				writeln!(file, "\t\t\tstatic ref top_level_domain_names: HashSet<EfficientCaseFoldedName<'static>> = fast_secure_hash_set!")?;
				writeln!(file, "\t\t\t{{")?;
				for case_folded_label in top_level_domains
				{
					writeln!(file, "\t\t\t\tEfficientCaseFoldedName::{}().clone(),", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
				}
				writeln!(file, "\t\t\t}};")?;
				writeln!(file, "\t\t\t")?;
			writeln!(file, "\t\t}}")?;
			writeln!(file, "\t\t&top_level_domain_names")?;
		writeln!(file, "\t}}")?;
	
		for case_folded_label in top_level_domains
		{
			writeln!(file)?;
			writeln!(file, "\t/// `{}.`", case_folded_label.as_str())?;
			from_iana_comment(&mut file, version_and_updated_at)?;
			writeln!(file, "\t#[inline(always)]")?;
			writeln!(file, "\tpub fn {}() -> &'static Self", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
			writeln!(file, "\t{{")?;
			writeln!(file, "\t\tlazy_static!")?;
			writeln!(file, "\t\t{{")?;
				writeln!(file, "\t\t\tstatic ref {}: EfficientCaseFoldedName<'static> = EfficientCaseFoldedName::top_level(EfficientCaseFoldedLabel::{});", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit(), case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
			writeln!(file, "\t\t}}")?;
			writeln!(file, "\t\t")?;
			writeln!(file, "\t\t&{}", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
			writeln!(file, "\t}}")?;
		}
	
	writeln!(file, "}}")?;
	
	Ok(())
}
