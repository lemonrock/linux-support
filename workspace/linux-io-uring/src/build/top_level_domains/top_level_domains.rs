// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn top_level_domains(manifest_dir: &OsString) -> io::Result<(String, Vec<CaseFoldedLabel>)>
{
	let tlds_alpha_by_domain_file_path = Path::new(&manifest_dir).join(TopLevelDomainsRelativeFilePath);
	
	let mut tlds_alpha_by_domain = read(tlds_alpha_by_domain_file_path)?;
	
	// Prevent a final empty line.
	if tlds_alpha_by_domain.get(tlds_alpha_by_domain.len() - 1) == Some(&b'\n')
	{
		tlds_alpha_by_domain.pop();
	}
	
	let mut version_and_updated_at = None;
	let mut top_level_domains = Vec::with_capacity(2048);
	for line in tlds_alpha_by_domain.split(|byte| *byte == b'\n')
	{
		let line_starts_with_hash = line.starts_with(b"#");
		if version_and_updated_at.is_none()
		{
			if line_starts_with_hash
			{
				// "# "
				version_and_updated_at = Some(String::from_utf8_lossy(&line[2 ..]).into_owned());
				continue
			}
			else
			{
				panic!("First line was not a comment line")
			}
		}
		else
		{
			if line_starts_with_hash
			{
				panic!("Subsequent line started with a hash")
			}
		}
		
		let case_folded_label = CaseFoldedLabel::case_fold_label(line);
		top_level_domains.push(case_folded_label)
	}
	
	Ok((version_and_updated_at.expect("No first line"), top_level_domains))
}
