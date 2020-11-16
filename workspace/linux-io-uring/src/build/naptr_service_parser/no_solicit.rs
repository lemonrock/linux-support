// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 4095.
fn no_solicit() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		"no-solicit" => "NoSolicit { uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_NoSolicit(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }",
	}
}
