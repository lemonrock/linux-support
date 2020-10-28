// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A case-folded (normalized to lower case) name which consists of labels, including a terminal root label.
#[derive(Default, Debug, Clone)]
pub struct CaseFoldedName<'label>
{
	labels: Box<[CaseFoldedLabel<'label>]>,
	
	name_length_including_trailing_periods_after_labels: NonZeroU8,
}

impl<'label, 'b> TryFrom<&'b [u8]> for CaseFoldedName<'label>
{
	type Error = CaseFoldedNameParseError;
	
	#[inline(always)]
	fn try_from(value: &'b [u8]) -> Result<Self, Self::Error>
	{
		if unlikely!(value.len() < ParsedNameParser::NameMinimumSize)
		{
			return Err(CaseFoldedNameParseError::TotalNameLengthWasEmpty)
		}
		
		if unlikely!(value.len() > ParsedNameParser::NameMaximumSize)
		{
			return Err(CaseFoldedNameParseError::TotalNameLengthExceed255Bytes)
		}
		
		let mut labels: ReversePopulatedVecBuilder<CaseFoldedLabel>  = ReversePopulatedVecBuilder::new(RawLabel::MaximumNumber);
		
		let mut iterator = value.split_bytes_reverse(b'.');
		let mut name_length_including_trailing_periods_after_labels = labels.try_push_root_label(iterator.next().unwrap())?;
		for label in iterator
		{
			labels.try_push_non_empty_label(label, &mut name_length_including_trailing_periods_after_labels)?;
		}
		
		Ok
		(
			Self
			{
				labels: labels.to_vec().into_boxed_slice(),
				
				name_length_including_trailing_periods_after_labels,
			}
		)
	}
}

impl<'label: 'message, 'message, 'b> From<&'b ParsedName<'message>> for CaseFoldedName<'label>
{
	#[inline(always)]
	fn from(value: &'b ParsedName<'message>) -> Self
	{
		Self
		{
			labels:
			{
				let mut labels = Vec::with_capacity(value.label_data_starts_at_pointers_and_label_length_excluding_trailing_period.len());
				
				for index in 0 .. value.number_of_labels_including_root().get()
				{
					let parsed_label_cow = value.label(index);
					labels.push(CaseFoldedLabel::from(parsed_label_cow.deref()))
				}
				
				labels.into_boxed_slice()
			},
			
			name_length_including_trailing_periods_after_labels: value.name_length_including_trailing_periods_after_labels
		}
	}
}

impl<'label: 'message, 'message> From<ParsedName<'message>> for CaseFoldedName<'label>
{
	#[inline(always)]
	fn from(value: ParsedName<'message>) -> Self
	{
		Self::from(&value)
	}
}

impl<'label> Name<'label> for CaseFoldedName<'label>
{
	type Label = CaseFoldedLabel<'label>;
	
	#[inline(always)]
	fn parent(&self) -> Option<Self>
	{
		if unlikely!(self.is_root())
		{
			None
		}
		else
		{
			let child_label_length = self.label(0).len();
			Some
			(
				Self
				{
					labels: (self.labels[1 .. ]).to_vec().into_boxed_slice(),
					
					name_length_including_trailing_periods_after_labels: unsafe { NonZeroU8::new_unchecked(self.name_length_including_trailing_periods_after_labels.get() - child_label_length) },
				}
			)
		}
	}
	
	#[inline(always)]
	fn label(&self, index: u8) -> Cow<'label, Self::Label>
	{
		Cow::Borrowed(unsafe { self.labels.get_unchecked(index as usize) })
	}
	
	#[inline(always)]
	fn number_of_labels_including_root(&self) -> NonZeroU8
	{
		unsafe { NonZeroU8::new_unchecked(self.labels.len() as u8) }
	}
	
	#[inline(always)]
	fn name_length_including_trailing_periods_after_labels(&self) -> NonZeroU8
	{
		self.name_length_including_trailing_periods_after_labels
	}
}

impl<'a> PartialEq for CaseFoldedName<'a>
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.equals(rhs)
	}
}

impl<'a> Eq for CaseFoldedName<'a>
{
}

impl<'a> PartialOrd for CaseFoldedName<'a>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'a> Ord for CaseFoldedName<'a>
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.compare(rhs)
	}
}

impl<'a> Hash for CaseFoldedName<'a>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.hash_slice(state)
	}
}

impl<'a, 'message> PartialEq<ParsedName<'message>> for CaseFoldedName<'a>
{
	#[inline(always)]
	fn eq(&self, rhs: &ParsedName<'message>) -> bool
	{
		self.equals(rhs)
	}
}

impl<'a, 'message> PartialOrd<ParsedName<'message>> for CaseFoldedName<'a>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &ParsedName<'message>) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'a> CaseFoldedName<'a>
{
	#[inline(always)]
	pub(crate) fn map<'message>(parsed_name: ParsedName<'message>) -> Self
	where 'a: 'message
	{
		Self::from(parsed_name)
	}
	
	/// Push a child to the front of this name.
	///
	/// This will always succeed if this is a root, top level or second level name.
	///
	/// eg if this is `.example.net.` and `child` is `www`, this creates `.www.example.net.`.
	#[inline(always)]
	pub fn child(&self, child: CaseFoldedLabel<'a>) -> Result<Self, CaseFoldedNameParseError>
	{
		use self::CaseFoldedNameParseError::*;
		
		if self.labels.number_of_labels_including_root().get() == RawLabel::MaximumNumber
		{
			return Err(NumberOfLabelsExceed127)
		}
		
		let name_length_including_trailing_periods_after_labels = self.name_length_including_trailing_periods_after_labels().get().checked_add(child.length_including_trailing_period().get()).ok_or(TotalNameLengthExceed255Bytes)?;
		Ok
		(
			Self
			{
				labels:
				{
					let mut labels = Vec::with_capacity(self.labels.len() + 1);
					labels.push(child);
					labels.extend_from_slice(&self.labels[..]);
					labels.into_boxed_slice()
				},
				
				name_length_including_trailing_periods_after_labels: unsafe { NonZeroU8::new_unchecked(name_length_including_trailing_periods_after_labels) },
			}
		)
	}
	
	/// Root, ie `.`.
	#[inline(always)]
	pub fn root() -> Self
	{
		Self
		{
			name_length_including_trailing_periods_after_labels: unsafe
			{
				NonZeroU8::new_unchecked
				(
					CaseFoldedLabel::Root.length_including_trailing_period().get()
				)
			},
			
			labels: vec!
			[
				CaseFoldedLabel::Root
			].into_boxed_slice(),
		}
	}
	
	/// Top-level, eg `.net.`; contains 2 labels.
	#[inline(always)]
	pub fn top_level(top_level_label: CaseFoldedLabel<'a>) -> Self
	{
		Self
		{
			// Can be unchecked addition as maximum length is `(63 + 1) + (0 + 1) < 255`.
			name_length_including_trailing_periods_after_labels: unsafe
			{
				NonZeroU8::new_unchecked
				(
					top_level_label.length_including_trailing_period().get()
					+ CaseFoldedLabel::Root.length_including_trailing_period().get()
				)
			},
			
			labels: vec!
			[
				top_level_label,
				CaseFoldedLabel::Root,
			].into_boxed_slice(),
		}
	}
	
	/// Second-level, eg `.example.net.`; contains 3 labels.
	#[inline(always)]
	pub fn second_level(second_level_label: CaseFoldedLabel<'a>, top_level_label: CaseFoldedLabel<'a>) -> Self
	{
		Self
		{
			// Can be unchecked addition as maximum length is `((63 + 1) * 2) + (0 + 1) < 255`.
			name_length_including_trailing_periods_after_labels: unsafe
			{
				NonZeroU8::new_unchecked
				(
					second_level_label.length_including_trailing_period().get()
					+ top_level_label.length_including_trailing_period().get()
					+ CaseFoldedLabel::Root.length_including_trailing_period().get()
				)
			},
		
			labels: vec!
			[
				second_level_label,
				top_level_label,
				CaseFoldedLabel::Root
			].into_boxed_slice(),
		}
	}
	
	/// Third-level, eg `.www.example.net.`; contains 4 labels.
	#[inline(always)]
	pub fn third_level(third_level_label: CaseFoldedLabel<'a>, second_level_label: CaseFoldedLabel<'a>, top_level_label: CaseFoldedLabel<'a>) -> Self
	{
		Self
		{
			// Can be unchecked addition as maximum length is `((63 + 1) * 3) + (0 + 1) < 255`.
			name_length_including_trailing_periods_after_labels: unsafe
			{
				NonZeroU8::new_unchecked
				(
					third_level_label.length_including_trailing_period().get()
					+ second_level_label.length_including_trailing_period().get()
					+ top_level_label.length_including_trailing_period().get()
					+ CaseFoldedLabel::Root.length_including_trailing_period().get()
				)
			},
		
			labels: vec!
			[
				third_level_label,
				second_level_label,
				top_level_label,
				CaseFoldedLabel::Root
			].into_boxed_slice(),
		}
	}
	
	/// Fourth-level, eg `.www.example.co.uk.`; contains 5 labels.
	#[inline(always)]
	pub fn fourth_level(fourth_level_label: CaseFoldedLabel<'a>, third_level_label: CaseFoldedLabel<'a>, second_level_label: CaseFoldedLabel<'a>, top_level_label: CaseFoldedLabel<'a>) -> Result<Self, CaseFoldedNameParseError>
	{
		Self::third_level(third_level_label, second_level_label, top_level_label).child(fourth_level_label)
	}
}

impl CaseFoldedName<'static>
{
	/// Ends with special use name.
	pub fn ends_with_special_use(&self) -> bool
	{
		if self.is_root_or_special_use_name()
		{
			return true
		}
		
		for special_use in Self::special_uses()
		{
			if self.ends_with(special_use)
			{
				return true
			}
		}
		
		false
	}
	
	/// An exact match.
	#[inline(always)]
	pub fn is_root_or_special_use_name(&self) -> bool
	{
		self.is_root() || Self::special_uses().contains(self)
	}
	
	#[inline(always)]
	fn special_uses() -> &'static HashSet<Self>
	{
		lazy_static!
		{
    		static ref SpecialUses: CaseFoldedName<'static> = CaseFoldedName::special_use_domain_names().bitand(&CaseFoldedName::rfc_7788_local_name_mistake()).bitand(&CaseFoldedName::recommended_local_names_in_rfc_6762_appendix_g()).bitand(&CaseFoldedName::iana_test_internationalized_domain_names());
    	}
		
		&SpecialUses
	}
	
	/// Concept created by RFC 6761.
	///
	/// See <https://www.iana.org/assignments/special-use-domain-names/special-use-domain-names.xhtml>.
	#[inline(always)]
	pub fn special_use_domain_names() -> HashSet<Self>
	{
		hashset!
		{
			// Reference: RFC-ietf-6tisch-minimal-security-15.
			Self::second_level(CaseFoldedLabel::_6tisch, CaseFoldedLabel::arpa),
			
			// RFC 6761, Section 6.1, Domain Name Reservation Considerations for Private Addresses.
			Self::third_level(CaseFoldedLabel::byte(10), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(16), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(17), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(18), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(19), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(20), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(21), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(22), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(23), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(24), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(25), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(26), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(27), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(28), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(29), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(30), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(31), CaseFoldedLabel::byte(172), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(168), CaseFoldedLabel::byte(192), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			
			// RFC 8880, Section 7.2, Names '170.0.0.192.in-addr.arpa' and '171.0.0.192.in-addr.arpa'.
			Self::internet_protocol_version_4_pointer(Ipv4Addr::new(170, 0, 0, 192)),
			Self::internet_protocol_version_4_pointer(Ipv4Addr::new(171, 0, 0, 192)),
			
			// RFC 6762, Section 12, Special Characteristics of Multicast DNS Domains.
			Self::fourth_level_unwrap(CaseFoldedLabel::byte(254), CaseFoldedLabel::byte(169), CaseFoldedLabel::in_addr, CaseFoldedLabel::arpa),
			Self::fifth_level_unwrap(CaseFoldedLabel::nibble(0x8), CaseFoldedLabel::nibble(0xE), CaseFoldedLabel::nibble(0xF), CaseFoldedLabel::ip6, CaseFoldedLabel::arpa),
			Self::fifth_level_unwrap(CaseFoldedLabel::nibble(0x9), CaseFoldedLabel::nibble(0xE), CaseFoldedLabel::nibble(0xF), CaseFoldedLabel::ip6, CaseFoldedLabel::arpa),
			Self::fifth_level_unwrap(CaseFoldedLabel::nibble(0xA), CaseFoldedLabel::nibble(0xE), CaseFoldedLabel::nibble(0xF), CaseFoldedLabel::ip6, CaseFoldedLabel::arpa),
			Self::fifth_level_unwrap(CaseFoldedLabel::nibble(0xB), CaseFoldedLabel::nibble(0xE), CaseFoldedLabel::nibble(0xF), CaseFoldedLabel::ip6, CaseFoldedLabel::arpa),
			
			// RFC 8375.
			Self::second_level(CaseFoldedLabel::home, CaseFoldedLabel::arpa),
			
			// RFC 6761, Section 6.5 Domain Name Reservation Considerations for Example Domains.
			Self::top_level(CaseFoldedLabel::example),
			Self::second_level(CaseFoldedLabel::example, CaseFoldedLabel::com),
			Self::second_level(CaseFoldedLabel::example, CaseFoldedLabel::net),
			Self::second_level(CaseFoldedLabel::example, CaseFoldedLabel::org),
			
			// RFC 6761, Section 6.4, Domain Name Reservation Considerations for "invalid.".
			Self::top_level(CaseFoldedLabel::invalid),
			
			// RFC 8880, Section 7.1, Special Use Domain Name 'ipv4only.arpa'.
			//
			// See also RFC 7050.
			//
			// This domain name is valid only for Internet Protocol version 4.
			// It is not valid only for Internet Protocol version 6.
			// Furthermore, for Internet Protocol version 4, it has the fixed `A` records `192.0.0.170` and `192.0.0.171`; these are defined in the [IANA IPv5 Special-Purpose Address Registry](https://www.iana.org/assignments/iana-ipv4-special-registry/iana-ipv4-special-registry.xhtml).
			Self::second_level(CaseFoldedLabel::ipv4only, CaseFoldedLabel::arpa),
			
			// RFC 6762, Section 12, Special Characteristics of Multicast DNS Domains.
			Self::top_level(CaseFoldedLabel::local),
			
			// RFC 6761, Section 6.3, Domain Name Reservation Considerations for "localhost.".
			Self::top_level(CaseFoldedLabel::localhost),
			
			// RFC 7686, Section 2, The ".onion" Special-Use Domain Name.
			Self::top_level(CaseFoldedLabel::onion),
			
			// RFC 6761, Section 6.2, Domain Name Reservation Considerations for "test.".
			Self::top_level(CaseFoldedLabel::test),
		}
	}
	
	/// Based on RFC 7788.
	///
	/// This usage has been deprecated by RFC 8375 replacing ".home." with ".home.arpa.".
	/// However, it is also used by RFC 6762, Appendix G.
	#[inline(always)]
	pub fn rfc_7788_local_name_mistake() -> HashSet<Self>
	{
		hashset!
		{
			Self::top_level(CaseFoldedLabel::home),
		}
	}
	
	/// Based on RFC 6762, Appendix G: Private DNS Namespace.
	///
	/// This contains 'sepcial use' multicast DNS domain names in common use.
	/// It excludes ".local." and ".home.".
	#[inline(always)]
	pub fn recommended_local_names_in_rfc_6762_appendix_g() -> HashSet<Self>
	{
		hashset!
		{
			Self::top_level(CaseFoldedLabel::intranet),
			Self::top_level(CaseFoldedLabel::internal),
			Self::top_level(CaseFoldedLabel::private),
			Self::top_level(CaseFoldedLabel::corp),
			Self::top_level(CaseFoldedLabel::lan),
		}
	}
	
	/// See <https://www.iana.org/domains/reserved>.
	#[inline(always)]
	pub fn iana_test_internationalized_domain_names() -> HashSet<Self>
	{
		hashset!
		{
			// Language: Arabic.
			// Script: Arabic
			// Non-punycode name `.إختبار.`
			Self::from(b"xn--kgbechtv"),
			
			// Language: Persian.
			// Script: Arabic.
			// Non-punycode name `.آزمایشی.`.
			Self::from(b"xn--hgbk6aj7f53bba"),
			
			// Language: Chinese.
			// Script: Han (Simplified variant).
			// Non-punycode name `.测试.`.
			Self::from(b"xn--0zwm56d"),
			
			// Language: Chinese.
			// Script: Han (Traditional variant).
			// Non-punycode name `.測試.`.
			Self::from(b"xn--g6w251d"),
			
			// Language: Russian.
			// Script: Cyrillic.
			// Non-punycode name `.испытание.`.
			Self::from(b"xn--80akhbyknj4f"),
			
			// Language: Hindi.
			// Script: Devanagari (Nagari).
			// Non-punycode name `.परीक्षा.`.
			Self::from(b"xn--11b5bs3a9aj6g"),
			
			// Language: Greek, Modern (1453-).
			// Script: Greek.
			// Non-punycode name `.δοκιμή.`.
			Self::from(b"xn--jxalpdlp"),
			
			// Language: Korean.
			// Script: Hangul (Hangŭl, Hangeul).
			// Non-punycode name `.테스트.`.
			Self::from(b"xn--9t4b11yi5a"),
			
			// Language: Yiddish
			// Script: Hebrew
			// Non-punycode name `.טעסט.`
			Self::from(b"xn--deba0ad"),
			
			// Language: Japanese.
			// Script: Katakana.
			// Non-punycode name `.テスト.`.
			Self::from(b"xn--zckzah"),
			
			// Language: Tamil.
			// Script: Tamil.
			// Non-punycode name `.பரிட்சை.`.
			Self::from(b"xn--hlcj6aya9esc7a"),
		}
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
		fn label(octets: &[u8; 4], index: usize) -> CaseFoldedLabel<'static>
		{
			CaseFoldedLabel::byte(*unsafe { octets.get_unchecked(index) })
		}
		
		let label3 = label(&octets, 3);
		let label2 = label(&octets, 2);
		let label1 = label(&octets, 1);
		let label0 = label(&octets, 0);
		
		Self
		{
			name_length_including_trailing_periods_after_labels: unsafe
			{
				NonZeroU8::new_unchecked
				(
					label3.length_including_trailing_period().get()
					+ label2.length_including_trailing_period().get()
					+ label1.length_including_trailing_period().get()
					+ label0.length_including_trailing_period().get()
					+ CaseFoldedLabel::in_addr.length_including_trailing_period().get()
					+ CaseFoldedLabel::arpa.length_including_trailing_period().get()
					+ CaseFoldedLabel::Root.length_including_trailing_period().get()
				)
			},
		
			labels: vec!
			[
				label3,
				label2,
				label1,
				label0,
				CaseFoldedLabel::in_addr,
				CaseFoldedLabel::arpa,
				CaseFoldedLabel::Root
			].into_boxed_slice(),
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
		fn label_pair(octets: &[u8; 16], index: usize) -> (CaseFoldedLabel<'static>, CaseFoldedLabel<'static>)
		{
			let label1 = CaseFoldedLabel::nibble((*unsafe { octets.get_unchecked(index) }) & 0b1111);
			let label0 = CaseFoldedLabel::nibble((*unsafe { octets.get_unchecked(index) }) >> 4);
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
			name_length_including_trailing_periods_after_labels: unsafe
			{
				NonZeroU8::new_unchecked
				(
					(NibbleLabelSize * NumberOfNibbles)
					+ CaseFoldedLabel::in_addr.length_including_trailing_period().get()
					+ CaseFoldedLabel::arpa.length_including_trailing_period().get()
					+ CaseFoldedLabel::Root.length_including_trailing_period().get()
				)
			},
			
			labels: vec!
			[
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
				CaseFoldedLabel::in_addr,
				CaseFoldedLabel::arpa,
				CaseFoldedLabel::Root,
			].into_boxed_slice(),
		}
	}
	
	#[inline(always)]
	fn fourth_level_unwrap(fourth_level_label: CaseFoldedLabel<'static>, third_level_label: CaseFoldedLabel<'static>, second_level_label: CaseFoldedLabel<'static>, top_level_label: CaseFoldedLabel<'static>) -> Self
	{
		Self::third_level(third_level_label, second_level_label, top_level_label).child(fourth_level_label).unwrap()
	}
	
	#[inline(always)]
	fn fifth_level_unwrap(fifth_level_label: CaseFoldedLabel<'static>, fourth_level_label: CaseFoldedLabel<'static>, third_level_label: CaseFoldedLabel<'static>, second_level_label: CaseFoldedLabel<'static>, top_level_label: CaseFoldedLabel<'static>) -> Self
	{
		Self::fourth_level_unwrap(fourth_level_label, third_level_label, second_level_label, top_level_label).child(fifth_level_label).unwrap()
	}
	
	#[inline(always)]
	fn sixth_level_unwrap(sixth_level_label: CaseFoldedLabel<'static>, fifth_level_label: CaseFoldedLabel<'static>, fourth_level_label: CaseFoldedLabel<'static>, third_level_label: CaseFoldedLabel<'static>, second_level_label: CaseFoldedLabel<'static>, top_level_label: CaseFoldedLabel<'static>) -> Self
	{
		Self::fifth_level_unwrap(fifth_level_label, fourth_level_label, third_level_label, second_level_label, top_level_label).child(sixth_level_label).unwrap()
	}
}
