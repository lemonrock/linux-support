// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


trait GenerateParseTreeCallback: Sized
{
	const NoMatchingPattern: &'static str;
	
	fn push_function_start(code: &mut Code<Self>) -> io::Result<()>;
	
	fn push_function_end(code: Code<Self>) -> io::Result<()>;
	
	fn length_comparison_and_exact_match_string(byte_index: usize, naive_trie_node_value: Option<&Self>) -> (&str, String);
}
