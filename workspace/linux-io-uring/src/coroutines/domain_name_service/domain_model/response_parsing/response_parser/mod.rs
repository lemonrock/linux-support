// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::super::message::header::*;
use super::super::message::resource_record::ResourceRecord;


include!("validate_raw_message_length.rs");
include!("validate_number_of_entries_in_the_question_section_is_one.rs");
include!("validate_is_response.rs");
include!("validate_opcode.rs");
include!("validate_reserved_header_bits_are_zero.rs");
include!("validate_response_is_not_truncated.rs");
include!("validate_recursion_desired_bit_was_copied_from_query_and_is_one.rs");
include!("validate_checking_bit_was_copied_from_query_and_is_zero.rs");
include!("validate_authentic_answers_do_not_have_authoritative_data_bit_set.rs");
include!("validate_message_response_code.rs");


include!("DnsOverTlsConnection.rs");
include!("Outcome.rs");
include!("RequestQuery.rs");
include!("ResponseHeader.rs");
