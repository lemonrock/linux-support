// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a RFC 5424 message template, which just needs a timestamp and message.
pub struct Rfc5424MessageTemplate
{
	before_timestamp: ArrayVec<[u8; 7]>,
	after_timestamp: Vec<u8>,
}

impl MessageTemplate for Rfc5424MessageTemplate
{
	/// Writes a message to a `buffer` with the `timestamp` and `message`.
	///
	/// Returns the length written.
	///
	/// Timestamp ***MUST NOT*** include leap seconds.
	#[inline(always)]
	fn format(&self, buffer: &mut [u8], timestamp: DateTime<Utc>, message: &str) -> (usize, bool)
	{
		let timestamp_bytes = timestamp.to_rfc3339_opts(SecondsFormat::Micros, true).into_bytes();
		
		let start_pointer = buffer.as_mut_ptr();
		let mut write_to = start_pointer;
		unsafe
		{
			let end_pointer = write_to.add(buffer.len());
			write_to = write_slice_unchecked(write_to, &self.before_timestamp[..], end_pointer);
			write_to = write_slice_unchecked(write_to, &timestamp_bytes[..], end_pointer);
			write_to = write_slice_unchecked(write_to, &self.after_timestamp[..], end_pointer);
			let (write_to, truncated) = write_message_with_line_feed_escaped_truncated(write_to, message, end_pointer);
			let written_length = (write_to as usize) - (start_pointer as usize);
			(written_length, truncated)
		}
	}
}

impl Rfc5424MessageTemplate
{
	/// New instance.
	///
	/// ***SLOW***.
	pub fn new(facility: KnownFacility, severity: Severity, host_name: &HostName, application_name: &ApplicationName, message_identifier: &MessageIdentifier, internet_protocol_addresses: &[IpAddr], private_enterprise_number: &PrivateEnterpriseNumber) -> Self
	{
		let process_identifier = ProcessIdentifierOrName::new().expect("Process identifier should not be that long!");
		
		let before_timestamp =
		{
			let mut before_timestamp = ArrayVec::new();
			let string = format!("<{}>{} ", PriorityValue::from_facility_and_severity(facility, severity).as_u8(), Version::_1 as u8);
			before_timestamp.try_extend_from_slice(string.as_bytes()).unwrap();
			before_timestamp
		};
		
		let mut after_timestamp: Vec<u8> =
		{
			let mut header_after_timestamp = Vec::with_capacity(2048);
			
			header_after_timestamp.push(PrintableAsciiCharacter::Space);
			
			header_after_timestamp.extend_from_slice(&host_name[..]);
			header_after_timestamp.push(PrintableAsciiCharacter::Space);
			
			header_after_timestamp.extend_from_slice(&application_name[..]);
			header_after_timestamp.push(PrintableAsciiCharacter::Space);
			
			header_after_timestamp.extend_from_slice(&process_identifier[..]);
			header_after_timestamp.push(PrintableAsciiCharacter::Space);
			
			header_after_timestamp.extend_from_slice(&message_identifier[..]);
			header_after_timestamp.push(PrintableAsciiCharacter::Space);
			
			unsafe { transmute(header_after_timestamp) }
		};
		
		Self::write_timeQuality_structured_element(&mut after_timestamp);
		Self::write_origin_structured_element(&mut after_timestamp, internet_protocol_addresses, private_enterprise_number);
		Self::write_message_start(&mut after_timestamp);
		
		after_timestamp.shrink_to_fit();
		
		Self
		{
			before_timestamp,
			after_timestamp,
		}
	}
	
	#[inline(always)]
	fn write_timeQuality_structured_element(structured_data: &mut Vec<u8>)
	{
		Self::write_structured_element(structured_data, StructuredDataIdentifier::timeQuality(), vec!
		[
			(ParameterName::tzKnown(), UnescapedParameterValue::True),
			(ParameterName::isSynced(), UnescapedParameterValue::True)
		])
	}
	
	#[inline(always)]
	fn write_origin_structured_element(structured_data: &mut Vec<u8>, internet_protocol_addresses: &[IpAddr], private_enterprise_number: &PrivateEnterpriseNumber)
	{
		let mut parameters = Vec::with_capacity(internet_protocol_addresses.len() + 3);
		for internet_protocol_address in internet_protocol_addresses
		{
			parameters.push((ParameterName::ip(), UnescapedParameterValue::from_internet_protocol_address(internet_protocol_address)))
		}
		parameters.push((ParameterName::enterpriseId(), UnescapedParameterValue::from_private_enterprise_number(private_enterprise_number)));
		parameters.push((ParameterName::software(), UnescapedParameterValue::CargoPackageName));
		parameters.push((ParameterName::swVersion(), UnescapedParameterValue::CargoPackageVersion));
		
		Self::write_structured_element(structured_data, StructuredDataIdentifier::origin(), parameters)
	}
	
	#[inline(always)]
	fn write_message_start(after_structured_data: &mut Vec<u8>)
	{
		const ByteOrderMark: [u8; 3] = [0xEF, 0xBB, 0xBF];
		after_structured_data.extend_from_slice(&ByteOrderMark);
	}
	
	/// `sequence_identifier` should be a global atomic counter.
	///
	/// Obtaining the system up time requires a system call.
	#[allow(dead_code)]
	fn write_meta_structured_element(structured_data: &mut Vec<u8>, sequence_identifier: NonZeroU64)
	{
		Self::write_structured_element(structured_data, StructuredDataIdentifier::meta(), vec!
		[
			(ParameterName::sequenceId(), UnescapedParameterValue::from_sequence_identifier(sequence_identifier)),
			(ParameterName::sysUpTime(), UnescapedParameterValue::system_up_time()),
			(ParameterName::language(), UnescapedParameterValue::EnglishLanguage),
		])
	}
	
	fn write_structured_element(structured_data: &mut Vec<u8>, structured_data_identifier: &StructuredDataIdentifier, parameters: Vec<(&ParameterName, UnescapedParameterValue)>)
	{
		structured_data.push(b'[');
		
		structured_data.extend_from_slice(unsafe { transmute(&structured_data_identifier[..]) });
		
		for (parameter_name, parameter_value) in parameters
		{
			structured_data.push(b' ');
			structured_data.extend_from_slice(unsafe { transmute(&parameter_name[..]) });
			structured_data.extend_from_slice(b"=\"");
			Self::copy_and_escape_parameter_value(parameter_value, structured_data);
			structured_data.push(b'"');
		}
		
		structured_data.extend_from_slice(b"] ");
	}
	
	fn copy_and_escape_parameter_value(parameter_value: UnescapedParameterValue, structured_data: &mut Vec<u8>)
	{
		let mut haystack = parameter_value.0.as_bytes();
		let mut inclusive_from_index = 0;
		loop
		{
			match memchr3(b'=', b']', b'"', haystack)
			{
				None =>
				{
					structured_data.extend_from_slice(&haystack[inclusive_from_index .. ]);
					break
				}
				
				Some(index) =>
				{
					structured_data.extend_from_slice(&haystack[inclusive_from_index .. index]);
					inclusive_from_index = index + 1;
					haystack = &haystack[inclusive_from_index .. ];
				}
			}
		}
	}
}
