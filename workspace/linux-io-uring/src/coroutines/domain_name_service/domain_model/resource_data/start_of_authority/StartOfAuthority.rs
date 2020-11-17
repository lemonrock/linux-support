// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Start of Authority (`SOA`) data.
#[derive(Debug, Clone)]
pub struct StartOfAuthority<N: Name>
{
	/// `MNAME`.
	///
	/// This is the FQDN of the primary name server.
	pub primary_name_server: N,

	/// `RNAME`.
	///
	/// First label is the name `@`, eg `hostmaster.example.com.` is the email address `hostmaster@example.com`.
	///
	/// This has been validated to be at least a second-level domain (ie at least `hostmaster@com`).
	pub(crate) responsible_person_email_address: N,
	
	/// `SERIAL`.
	///
	/// Serial number of the zone file that is incremented each time a change is made.
	///
	/// A common practice is to use `YYYYMMDDnn`, where `YYYY` is the year, `MM` is the month, `DD` is the day, and `nn` is the revision number within the day.
	pub zone_file_serial_number: SerialNumber,
	
	/// `REFRESH`.
	///
	/// Refresh interval
	///
	/// Time in seconds that a secondary name server should wait between zone file update checks.
	///
	/// A typical value is between 30 minutes (1,800 seconds) and 2 hours (7,200 seconds).
	pub referesh_interval: U31SecondsDuration,
	
	/// `RETRY`.
	///
	/// Retry interval.
	///
	/// Time in seconds that a secondary name server should wait before trying to contact the primary name server again after a failed attempt to check for a zone file update.
	///
	/// A typical value is between 10 minutes (600 seconds) and 1 hour (3,600 seconds), and should take be ***less*** than the `refresh_interval`.
	pub retry_interval: U31SecondsDuration,
	
	/// `EXPIRE`.
	///
	/// Expiry interval.
	///
	/// Time in seconds that a secondary name server will treat its zone file as valid when the primary name server cannot be contacted.
	///
	/// A typical value is between 2 weeks (1,209,600 seconds) and 4 weeks (2,419,200 seconds).
	pub expire_interval: U31SecondsDuration,
}

impl<'message> ParsedRecord for StartOfAuthority<ParsedName<'message>>
{
	type OrderPriorityAndWeight = ();
	
	type OwnedRecord = StartOfAuthority<EfficientCaseFoldedName>;
	
	#[inline(always)]
	fn into_owned_record(self) -> Self::OwnedRecord
	{
		StartOfAuthority
		{
			primary_name_server: EfficientCaseFoldedName::from(self.primary_name_server),
			responsible_person_email_address: EfficientCaseFoldedName::from(self.responsible_person_email_address),
			zone_file_serial_number: self.zone_file_serial_number,
			referesh_interval: self.referesh_interval,
			retry_interval: self.retry_interval,
			expire_interval: self.expire_interval,
		}
	}
	
	#[inline(always)]
	fn store(cache: &mut QueryTypesCache, records: OwnerNameToRecordsValue<Self>)
	{
		let cache_until = records.cache_until();
		
		cache.SOA = QueryTypeCache::data(cache_until, records.solitary().into_owned_record());
	}
	
	#[inline(always)]
	fn no_data(cache: &mut QueryTypesCache, negative_cache_until: NegativeCacheUntil)
	{
		cache.SOA = QueryTypeCache::no_data(negative_cache_until);
	}
}

impl<N: Name> StartOfAuthority<N>
{
	/// eg `hostmaster` for `hostmaster@example.com`.
	///
	/// This has been validated to be at least a second-level domain (ie at least `hostmaster@com`).
	#[inline(always)]
	pub fn responsible_person_email_address_name<'label>(&'label self) -> N::Label<'label>
	{
		self.responsible_person_email_address.last_label().unwrap()
	}
	
	/// eg `example.com` for `hostmaster@example.com`.
	///
	/// This has been validated to be at least a second-level domain (ie at least `hostmaster@com`).
	#[inline(always)]
	pub fn responsible_person_email_address_domain(&self) -> N
	{
		self.responsible_person_email_address.parent().unwrap()
	}
}
