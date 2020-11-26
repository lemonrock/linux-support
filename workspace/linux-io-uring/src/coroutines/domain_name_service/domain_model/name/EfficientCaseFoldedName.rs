// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This design is intended to cut down on the pointer chasing that occurs to try to make names more CPU cache friendly, and to make construction and cloning faster.
#[derive(Deserialize, Serialize)]
pub struct EfficientCaseFoldedName
{
	name_length_including_trailing_periods_after_labels: NonZeroU8,
	
	inner: Rc<EfficientCaseFoldedNameInner>,
}

impl Default for EfficientCaseFoldedName
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::root()
	}
}

impl Debug for EfficientCaseFoldedName
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.display(f)
	}
}

impl Display for EfficientCaseFoldedName
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.display(f)
	}
}

impl Clone for EfficientCaseFoldedName
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			name_length_including_trailing_periods_after_labels: self.name_length_including_trailing_periods_after_labels,
			
			inner: Rc::clone(&self.inner)
		}
	}
}

impl PartialEq for EfficientCaseFoldedName
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.equals(rhs)
	}
}

impl Eq for EfficientCaseFoldedName
{
}

impl PartialOrd for EfficientCaseFoldedName
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl Ord for EfficientCaseFoldedName
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.compare(rhs)
	}
}

impl Hash for EfficientCaseFoldedName
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.hash_slice(state)
	}
}

impl<'message> PartialEq<ParsedName<'message>> for EfficientCaseFoldedName
{
	#[inline(always)]
	fn eq(&self, rhs: &ParsedName<'message>) -> bool
	{
		self.equals(rhs)
	}
}

impl<'message> PartialOrd<ParsedName<'message>> for EfficientCaseFoldedName
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &ParsedName<'message>) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl HasTypeEquality for EfficientCaseFoldedName
{
	type TypeEquality = OwnedTypeEquality;
}

impl Name for EfficientCaseFoldedName
{
	type Label<'label> = EfficientCaseFoldedLabel<'label>;
	
	#[inline(always)]
	fn parent(&self) -> Option<Self>
	{
		self.inner.parent().map(|(inner, child_label_length_excluding_trailing_period)| Self
		{
			name_length_including_trailing_periods_after_labels: new_non_zero_u8(self.name_length_including_trailing_periods_after_labels.get() - (child_label_length_excluding_trailing_period.get() + 1)),
			inner
		})
	}
	
	#[inline(always)]
	fn label<'label>(&'label self, index: u8) -> Self::Label
	{
		self.inner.label(index)
	}
	
	#[inline(always)]
	fn number_of_labels_including_root(&self) -> NonZeroU8
	{
		self.inner.number_of_labels_including_root()
	}
	
	#[inline(always)]
	fn name_length_including_trailing_periods_after_labels(&self) -> NonZeroU8
	{
		self.name_length_including_trailing_periods_after_labels
	}
}

impl<'label> TryFrom<&'label [u8]> for EfficientCaseFoldedName
{
	type Error = EfficientCaseFoldedNameParseError;
	
	#[inline(always)]
	fn try_from(value: &'label [u8]) -> Result<Self, Self::Error>
	{
		Self::from_byte_string_ending_with_optional_trailing_period(value)
	}
}

impl<'message> From<ParsedName<'message>> for EfficientCaseFoldedName
{
	#[inline(always)]
	fn from(value: ParsedName<'message>) -> Self
	{
		Self::from(&value)
	}
}

impl<'a, 'message> From<&'a ParsedName<'message>> for EfficientCaseFoldedName
{
	#[inline(always)]
	fn from(name: &'a ParsedName<'message>) -> Self
	{
		Self::from_name(name)
	}
}

impl EfficientCaseFoldedName
{
	/// Prepends `relative_name`.
	pub fn prepend_relative_name(&self, relative_name: &RelativeDomainName) -> Result<Self, EfficientCaseFoldedNameParseError>
	{
		let mut this = self.clone();
		
		let inner = relative_name.as_ref();
		
		for index in 1 .. inner.number_of_labels_including_root().get()
		{
			this.child_mutate(inner.label(index))
		}
		
		Ok(this)
	}
	
	/// Prepends `host_name`.
	#[inline(always)]
	pub fn prepend_host_name(&self, host_name: &HostNameLabel) -> Result<Self, EfficientCaseFoldedNameParseError>
	{
		self.child(host_name.as_label())
	}
	
	/// Host name.
	#[inline(always)]
	pub fn host_name(&self) -> Option<HostNameLabel>
	{
		self.last_label().map(|last_label| HostNameLabel(last_label.0.to_vec().into_boxed_slice()))
	}
	
	/// Push a child to the front of this name.
	///
	/// This will always succeed if this is a root, top level or second level name.
	///
	/// eg if this is `.example.net.` and `child` is `www`, this creates `.www.example.net.`.
	#[inline(always)]
	pub fn child(&self, child: EfficientCaseFoldedLabel) -> Result<Self, EfficientCaseFoldedNameParseError>
	{
		Ok
		(
			Self
			{
				name_length_including_trailing_periods_after_labels: self.child_name_length_including_trailing_periods_after_labels(child)?,
				inner: self.inner.child_clone(child),
			}
		)
	}
	
	/// Push a child to the front of this name.
	///
	/// This will always succeed if this is a root, top level or second level name.
	///
	/// eg if this is `.example.net.` and `child` is `www`, this creates `.www.example.net.`.
	#[inline(always)]
	pub fn child_mutate(&mut self, child: EfficientCaseFoldedLabel) -> Result<(), EfficientCaseFoldedNameParseError>
	{
		self.name_length_including_trailing_periods_after_labels = self.child_name_length_including_trailing_periods_after_labels(child)?;
		self.inner.child_mutate(child);
		
		Ok(())
	}
	
	/// Push a child to the front of this name.
	///
	/// This will always succeed if this is a root, top level or second level name.
	///
	/// eg if this is `.example.net.` and `child` is `www`, this creates `.www.example.net.`.
	#[inline(always)]
	pub fn child_move(mut self, child: EfficientCaseFoldedLabel) -> Result<Self, EfficientCaseFoldedNameParseError>
	{
		self.child_mutate(child)?;
		Ok(self)
	}
	
	#[inline(always)]
	fn child_name_length_including_trailing_periods_after_labels(&self, child: EfficientCaseFoldedLabel) -> Result<NonZeroU8, EfficientCaseFoldedNameParseError>
	{
		use self::EfficientCaseFoldedNameParseError::*;
		
		if self.number_of_labels_including_root().get() == (RawLabel::MaximumNumber as u8)
		{
			return Err(NumberOfLabelsExceed127)
		}
		
		let name_length_including_trailing_periods_after_labels = self.name_length_including_trailing_periods_after_labels().get().checked_add(child.length_including_trailing_period().get()).ok_or(TotalNameLengthExceed255Bytes)?;
		Ok(new_non_zero_u8(name_length_including_trailing_periods_after_labels))
	}
	
	/// From a name.
	#[inline(always)]
	pub fn from_name<'label>(name: &impl Name<'label>) -> Self
	{
		let mut label_offsets_and_lengths_excluding_root: ArrayVec<[(u8, NonZeroU8); EfficientCaseFoldedNameInner::LabelsCount]> = ArrayVec::new();
		let mut label_data: [u8; EfficientCaseFoldedNameInner::LabelDataSize] = unsafe_uninitialized();
		
		let mut label_offset = 0u8;
		
		let number_of_labels_including_root = name.number_of_labels_including_root().get();
		debug_assert!(number_of_labels_including_root <= 127);
		
		if unlikely!(number_of_labels_including_root == 1)
		{
			return Self::root()
		}
		
		for label_index in 1 .. number_of_labels_including_root
		{
			let label = name.label(label_index);
			let label_length = label.not_root_length_checked(label_offset);
			
			for byte_index in 0 .. label_length
			{
				let case_folded_byte = label.get_unchecked_safe_case_folded_byte(byte_index);
				unsafe { label_data.as_mut_ptr().add((label_offset + byte_index) as usize).write(case_folded_byte) }
			}
			
			unsafe { label_offsets_and_lengths_excluding_root.push_unchecked((label_offset, new_non_zero_u8(label_length))) };
			label_offset += label_length;
		}
		
		Self
		{
			name_length_including_trailing_periods_after_labels: name.name_length_including_trailing_periods_after_labels(),
			
			inner: EfficientCaseFoldedNameInner::new(label_offsets_and_lengths_excluding_root, label_data),
		}
	}
	
	/// Converts from a byte string, case folding to lower case as needed.
	///
	/// Supports byte strings that may omit the final (root) trailing period, viz supports both `example.com` and `example.com.`.
	#[inline(always)]
	pub fn from_byte_string_ending_with_optional_trailing_period(byte_string: &[u8]) -> Result<Self, EfficientCaseFoldedNameParseError>
	{
		use self::EfficientCaseFoldedLabelParseError::*;
		use self::EfficientCaseFoldedNameParseError::*;
		
		let byte_string_length = byte_string.len();
		
		if unlikely!(byte_string_length == 0)
		{
			return Ok(Self::root())
		}
		
		if unlikely!(byte_string_length > ParsedNameParser::NameMaximumSize)
		{
			return Err(TotalNameLengthExceed255Bytes)
		}
		
		const TrailingPeriod: u8 = b'.';
		
		let last_index = byte_string_length - 1;
		let ends_with_trailing_period = byte_string.get_unchecked_safe_value(last_index) == TrailingPeriod;
		let byte_string_to_split = if ends_with_trailing_period
		{
			&byte_string[0 .. ]
		}
		else
		{
			byte_string
		};
		
		let mut name_length_including_trailing_periods_after_labels = ParsedNameParser::SizeOfTrailingPeriod;
		let mut label_offsets_and_lengths_excluding_root = ArrayVec::new();
		let mut label_data: [u8; EfficientCaseFoldedNameInner::LabelDataSize] = unsafe_uninitialized();
		let mut label_offset = 0;
		for label_top_level_domain_first in byte_string_to_split.split_bytes_reverse(TrailingPeriod)
		{
			let label_length = label_top_level_domain_first.len() as u8;
			if unlikely!(label_length == 0)
			{
				return Err(NonRootLabelWasEmpty)
			}
			if unlikely!(label_length > 63)
			{
				return Err(CaseFoldedLabelParse(LabelExceeded63Bytes))
			}
			for byte_index in 0 .. label_length
			{
				let case_folded_byte = case_fold_byte(label_top_level_domain_first.get_unchecked_safe(byte_index));
				unsafe { label_data.get_mut_ptr().add((label_offset + byte_index) as usize).write(case_folded_byte) }
			}
			label_offsets_and_lengths_excluding_root.try_push((label_offset, new_non_zero_u8(label_length))).map_err(|_| NumberOfLabelsExceed127)?;
			label_offset += label_length;
			name_length_including_trailing_periods_after_labels += (label_length + ParsedNameParser::SizeOfTrailingPeriod);
		}
		
		Ok
		(
			Self
			{
				name_length_including_trailing_periods_after_labels: new_non_zero_u8(name_length_including_trailing_periods_after_labels),
				inner: EfficientCaseFoldedNameInner::new(label_offsets_and_lengths_excluding_root, label_data)
			}
		)
	}
	
	/// Root, ie `.`.
	#[inline(always)]
	pub fn root() -> Self
	{
		Self
		{
			name_length_including_trailing_periods_after_labels: new_non_zero_u8(1),
			
			inner: EfficientCaseFoldedNameInner::root(),
		}
	}
	
	/// Top-level, eg `.net.`; contains 2 labels.
	#[inline(always)]
	pub fn top_level(top_level_label: EfficientCaseFoldedLabel) -> Self
	{
		Self
		{
			// Can be unchecked addition as maximum length is `(63 + 1) + (0 + 1) < 255`.
			name_length_including_trailing_periods_after_labels: new_non_zero_u8
			(
				top_level_label.length_including_trailing_period().get()
				+ EfficientCaseFoldedLabel::Root.length_including_trailing_period().get()
			),
			
			inner: EfficientCaseFoldedNameInner::collect_excluding_root(&[top_level_label]),
		}
	}
	
	/// Second-level, eg `.example.net.`; contains 3 labels.
	#[inline(always)]
	pub fn second_level(second_level_label: EfficientCaseFoldedLabel, top_level_label: EfficientCaseFoldedLabel) -> Self
	{
		Self
		{
			// Can be unchecked addition as maximum length is `((63 + 1) * 2) + (0 + 1) < 255`.
			name_length_including_trailing_periods_after_labels: new_non_zero_u8
			(
				second_level_label.length_including_trailing_period().get()
				+ top_level_label.length_including_trailing_period().get()
				+ EfficientCaseFoldedLabel::Root.length_including_trailing_period().get()
			),
			
			inner: EfficientCaseFoldedNameInner::collect_excluding_root(&[second_level_label, top_level_label]),
		}
	}
	
	/// Third-level, eg `.www.example.net.`; contains 4 labels.
	#[inline(always)]
	pub fn third_level(third_level_label: EfficientCaseFoldedLabel, second_level_label: EfficientCaseFoldedLabel, top_level_label: EfficientCaseFoldedLabel) -> Self
	{
		Self
		{
			// Can be unchecked addition as maximum length is `((63 + 1) * 3) + (0 + 1) < 255`.
			name_length_including_trailing_periods_after_labels: new_non_zero_u8
			(
				third_level_label.length_including_trailing_period().get()
				+ second_level_label.length_including_trailing_period().get()
				+ top_level_label.length_including_trailing_period().get()
				+ EfficientCaseFoldedLabel::Root.length_including_trailing_period().get()
			),
			
			inner: EfficientCaseFoldedNameInner::collect_excluding_root(&[third_level_label, second_level_label, top_level_label]),
		}
	}
	
	/// Fourth-level, eg `.www.example.co.uk.`; contains 5 labels.
	#[inline(always)]
	pub fn fourth_level(fourth_level_label: EfficientCaseFoldedLabel, third_level_label: EfficientCaseFoldedLabel, second_level_label: EfficientCaseFoldedLabel, top_level_label: EfficientCaseFoldedLabel) -> Result<Self, EfficientCaseFoldedNameParseError>
	{
		Self::third_level(third_level_label, second_level_label, top_level_label).child_move(fourth_level_label)
	}
	
	#[inline(always)]
	fn fourth_level_unwrap(fourth_level_label: EfficientCaseFoldedLabel, third_level_label: EfficientCaseFoldedLabel, second_level_label: EfficientCaseFoldedLabel, top_level_label: EfficientCaseFoldedLabel) -> Self
	{
		Self::third_level(third_level_label, second_level_label, top_level_label).child_move(fourth_level_label).unwrap()
	}
	
	#[inline(always)]
	fn fifth_level_unwrap(fifth_level_label: EfficientCaseFoldedLabel, fourth_level_label: EfficientCaseFoldedLabel, third_level_label: EfficientCaseFoldedLabel, second_level_label: EfficientCaseFoldedLabel, top_level_label: EfficientCaseFoldedLabel) -> Self
	{
		Self::fourth_level_unwrap(fourth_level_label, third_level_label, second_level_label, top_level_label).child_move(fifth_level_label).unwrap()
	}
	
	#[inline(always)]
	fn sixth_level_unwrap(sixth_level_label: EfficientCaseFoldedLabel, fifth_level_label: EfficientCaseFoldedLabel, fourth_level_label: EfficientCaseFoldedLabel, third_level_label: EfficientCaseFoldedLabel, second_level_label: EfficientCaseFoldedLabel, top_level_label: EfficientCaseFoldedLabel) -> Self
	{
		Self::fifth_level_unwrap(fifth_level_label, fourth_level_label, third_level_label, second_level_label, top_level_label).child_move(sixth_level_label).unwrap()
	}
	
	/// A name suitable only for a `PTR` query.
	///
	/// Not all addresses are suitable, in particular, the following are rejected:-
	///
	/// * Private.
	/// * Loopback.
	/// * Link-Local.
	/// * Broadcast.
	/// * Documentation.
	/// * The unspecified address `0.0.0.0/32`.
	/// * Reserved.
	/// * Benchmarking.
	#[inline(always)]
	pub fn internet_protocol_version_4_pointer(address: Ipv4Addr) -> Option<Self>
	{
		if address.is_private()
		{
			return None
		}
		
		if address.is_loopback()
		{
			return None
		}
		
		if address.is_link_local()
		{
			return None
		}
		
		if address.is_broadcast()
		{
			return None
		}
		
		if address.is_documentation()
		{
			return None
		}
		
		if address.is_unspecified()
		{
			return None
		}
		
		if address.is_reserved()
		{
			return None
		}
		
		if address.is_benchmarking()
		{
			return None
		}
		
		Some(Self::internet_protocol_version_4_pointer_unchecked(address))
	}
	
	#[inline(always)]
	fn internet_protocol_version_4_pointer_unchecked(address: Ipv4Addr) -> Self
	{
		let octets = address.octets();
		
		#[inline(always)]
		fn label(octets: &[u8; 4], index: usize) -> EfficientCaseFoldedLabel<'static>
		{
			EfficientCaseFoldedLabel::byte(octets.get_unchecked_safe_value(index))
		}
		
		let label3 = label(&octets, 3);
		let label2 = label(&octets, 2);
		let label1 = label(&octets, 1);
		let label0 = label(&octets, 0);
		
		Self
		{
			name_length_including_trailing_periods_after_labels: new_non_zero_u8
			(
				label3.length_including_trailing_period().get()
				+ label2.length_including_trailing_period().get()
				+ label1.length_including_trailing_period().get()
				+ label0.length_including_trailing_period().get()
				+ EfficientCaseFoldedLabel::in_addr.length_including_trailing_period().get()
				+ EfficientCaseFoldedLabel::arpa.length_including_trailing_period().get()
				+ EfficientCaseFoldedLabel::Root.length_including_trailing_period().get()
			),
		
			inner: EfficientCaseFoldedNameInner::collect_excluding_root
			(
				&[
					label3,
					label2,
					label1,
					label0,
					EfficientCaseFoldedLabel::in_addr,
					EfficientCaseFoldedLabel::arpa,
				]
			),
		}
	}
	
	/// A name suitable only for a `PTR` query.
	///
	/// Not all addresses are suitable, in particular, the following are rejected:-
	///
	/// * Loopback.
	/// * Documentation.
	/// * The unspecified addres.
	/// * Multicast
	/// * Unicast link-local.
	/// * Unicast site-local.
	#[inline(always)]
	pub fn internet_protocol_version_6_pointer(address: Ipv6Addr) -> Option<Self>
	{
		if address.is_loopback()
		{
			return None
		}
		
		if address.is_documentation()
		{
			return None
		}
		
		if address.is_unspecified()
		{
			return None
		}
		
		if address.is_multicast()
		{
			return None
		}
		
		if address.is_unicast_link_local_strict()
		{
			return None
		}
		
		if address.is_unicast_site_local()
		{
			return None
		}
		
		Some(Self::internet_protocol_version_6_pointer_unchecked(address))
	}
	
	#[inline(always)]
	fn internet_protocol_version_6_pointer_unchecked(address: Ipv6Addr) -> Self
	{
		let octets = address.octets();
		
		#[inline(always)]
		fn label_pair(octets: &[u8; 16], index: usize) -> (EfficientCaseFoldedLabel<'static>, EfficientCaseFoldedLabel<'static>)
		{
			let label1 = EfficientCaseFoldedLabel::nibble(octets.get_unchecked_safe_value(index) & 0b1111);
			let label0 = EfficientCaseFoldedLabel::nibble(octets.get_unchecked_safe_value(index) >> 4);
			(label1, label0)
		}
		
		const NumberOfNibbles: u8 = 32;
		const NibbleLabelSize: u8 = 1 + ParsedNameParser::SizeOfTrailingPeriod;
		
		let (label31, label30) = label_pair(&octets, 15);
		let (label29, label28) = label_pair(&octets, 14);
		let (label27, label26) = label_pair(&octets, 13);
		let (label25, label24) = label_pair(&octets, 12);
		let (label23, label22) = label_pair(&octets, 11);
		let (label21, label20) = label_pair(&octets, 10);
		let (label19, label18) = label_pair(&octets, 9);
		let (label17, label16) = label_pair(&octets, 8);
		let (label15, label14) = label_pair(&octets, 7);
		let (label13, label12) = label_pair(&octets, 6);
		let (label11, label10) = label_pair(&octets, 5);
		let (label9, label8) = label_pair(&octets, 4);
		let (label7, label6) = label_pair(&octets, 3);
		let (label5, label4) = label_pair(&octets, 2);
		let (label3, label2) = label_pair(&octets, 1);
		let (label1, label0) = label_pair(&octets, 0);
		
		Self
		{
			name_length_including_trailing_periods_after_labels: new_non_zero_u8
			(
				(NibbleLabelSize * NumberOfNibbles)
				+ EfficientCaseFoldedLabel::in_addr.length_including_trailing_period().get()
				+ EfficientCaseFoldedLabel::arpa.length_including_trailing_period().get()
				+ EfficientCaseFoldedLabel::Root.length_including_trailing_period().get()
			),
			
			inner: EfficientCaseFoldedNameInner::collect_excluding_root
			(
				&[
					label31,
					label30,
					label29,
					label28,
					label27,
					label26,
					label25,
					label24,
					label23,
					label22,
					label21,
					label20,
					label19,
					label18,
					label17,
					label16,
					label15,
					label14,
					label13,
					label12,
					label11,
					label10,
					label9,
					label8,
					label7,
					label6,
					label5,
					label4,
					label3,
					label2,
					label1,
					label0,
					EfficientCaseFoldedLabel::in_addr,
					EfficientCaseFoldedLabel::arpa,
				]
			),
		}
	}
	
	/// Concept created by RFC 6761.
	///
	/// See <https://www.iana.org/assignments/special-use-domain-names/special-use-domain-names.xhtml>.
	#[inline(always)]
	pub fn special_use_domain_names() -> &'static HashSet<Self>
	{
		lazy_static!
		{
			static ref special_use_domain_names: HashSet<EfficientCaseFoldedName<'static>> = fast_secure_hash_set!
			{
				
				// RFC 8880, Section 7.2, Names '170.0.0.192.in-addr.arpa' and '171.0.0.192.in-addr.arpa'.
				EfficientCaseFoldedName::internet_protocol_version_4_pointer_unchecked(Ipv4Addr::new(170, 0, 0, 192)),
				EfficientCaseFoldedName::internet_protocol_version_4_pointer_unchecked(Ipv4Addr::new(171, 0, 0, 192)),
				
			};
		}
		
		&special_use_domain_names
	}
}

include!(concat!(env!("OUT_DIR"), "/EfficientCaseFoldedName.top_level_domains.rs"));
