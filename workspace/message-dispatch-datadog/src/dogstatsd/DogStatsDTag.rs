// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A tag must:-
///
/// * start with a letter;
/// * contains only ASCII Alphanumerics, the minus sign, a colon, a period or a slash.
///
/// A tag must not:-
///
/// * end with a colon;
/// * exceed 200 bytes;
/// * be empty;
/// * be one of the reserved values `host`, `device`, `source` or `service`.
///
/// A tag can be a key-value pair, such as `my_tag:my_value` but is subject to the above rules; the value is considered to be the remainder of the tag after the first colon.
/// Consquently values can not be empty or end with a colon.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DogStatsDTag(ArrayVec<[u8; Self::Length]>);

static mut process_name: Option<DogStatsDTag> = None;

static mut environment: Option<DogStatsDTag> = None;

impl DogStatsDTag
{
	const Length: usize = 200;
	
	/// Into additional tag.
	#[inline(always)]
	pub fn into_additional_dog_stats_d_tag<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(self, global_allocator: &'static GTACSA) -> AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>
	{
		AdditionalDogStatsDTag::from(self, global_allocator)
	}
	
	/// Process name.
	///
	/// Panics if already initialized.
	#[inline(always)]
	pub fn initialize_process_name(name: &ProcessName)
	{
		unsafe
		{
			if process_name.is_none()
			{
				process_name = Some(Self::name(name).expect("Invalid name for process_name"));
			}
			else
			{
				panic!("Already initialized process_name")
			}
		}
	}
	
	/// Process name.
	///
	/// Panics if not initialized.
	#[inline(always)]
	pub fn process_name() -> &'static Self
	{
		(unsafe { &process_name }).as_ref().expect("process_name not initialized")
	}
	
	/// Environment name based on Linux kernel domain name; uses first (least specific) label.
	///
	/// Panics if already initialized.
	#[inline(always)]
	pub fn initialize_environment(linux_kernel_domain_name: &LinuxKernelDomainName)
	{
		unsafe
		{
			if environment.is_none()
			{
				environment = Some
				(
					{
						let bytes = &linux_kernel_domain_name[..];
						let mut least_specific_label_iterator = bytes.split_bytes(b'.');
						let least_specific_label = least_specific_label_iterator.next().unwrap();
						
						let _length = Label::validate(least_specific_label).expect("Invalid linux_kernel_domain_name");
						Self::env(least_specific_label).expect("Invalid linux_kernel_domain_name for environment")
					}
				);
			}
			else
			{
				panic!("Already initialized environment")
			}
		}
	}
	
	/// Environment name based on domain name.
	///
	/// Panics if not initialized.
	#[inline(always)]
	pub fn environment() -> &'static Self
	{
		(unsafe { &environment }).as_ref().expect("environment not initialized")
	}
	
	/// Thread name; initialized once per thread.
	#[inline(always)]
	pub fn thread_name() -> &'static Self
	{
		#[thread_local] static mut thread_name: Option<DogStatsDTag> = None;
		
		unsafe
		{
			if unlikely!(thread_name.is_none())
			{
				let tag = Self::from_name_and_value("thread_name", &ThreadName::get_current_thread_name().to_string()).expect("Invalid thread name");
				thread_name = Some(tag);
			}
			
			thread_name.as_ref().unwrap()
		}
	}
	
	/// Hyper thread number; initialized once per thread.
	#[inline(always)]
	pub fn hyper_thread() -> &'static Self
	{
		#[thread_local] static mut hyper_thread: Option<DogStatsDTag> = None;
		
		unsafe
		{
			if unlikely!(hyper_thread.is_none())
			{
				hyper_thread = Some(Self::from_u16("hyper_thread", NumaNode::current().1).unwrap());
			}
			
			hyper_thread.as_ref().unwrap()
		}
	}
	
	/// NUMA node number; initialized once per thread.
	#[inline(always)]
	pub fn numa_node() -> &'static Self
	{
		#[thread_local] static mut numa_node: Option<DogStatsDTag> = None;
		
		unsafe
		{
			if unlikely!(numa_node.is_none())
			{
				numa_node = Some(Self::from_u16("numa_node", NumaNode::current().0).unwrap());
			}
			
			numa_node.as_ref().unwrap()
		}
	}
	
	/// From an u8.
	#[inline(always)]
	pub fn from_u8(name: impl AsRef<[u8]>, value: impl Into<u8>) -> Result<Self, String>
	{
		Self::from_name_and_value(name, &format!("{}", value.into()))
	}
	
	/// From an u16.
	#[inline(always)]
	pub fn from_u16(name: impl AsRef<[u8]>, value: impl Into<u16>) -> Result<Self, String>
	{
		Self::from_name_and_value(name, &format!("{}", value.into()))
	}
	
	/// From an usize.
	#[inline(always)]
	pub fn from_usize(name: impl AsRef<[u8]>, value: impl Into<usize>) -> Result<Self, String>
	{
		Self::from_name_and_value(name, &format!("{}", value.into()))
	}
	
	/// Process identifier; initialized once.
	#[inline(always)]
	pub fn process_identifier() -> &'static Self
	{
		lazy_static!
		{
			static ref process_identifier: DogStatsDTag =
			{
				let current_process_identifier: pid_t = ProcessIdentifier::default().into();
				DogStatsDTag::from_name_and_value("process_identifier", &format!("{}", current_process_identifier)).unwrap()
			};
		}
		
		&process_identifier
	}
	
	/// Thread identifier; initialized once per thread.
	#[inline(always)]
	pub fn thread_identifier() -> &'static Self
	{
		#[thread_local] static mut thread_identifier: Option<DogStatsDTag> = None;
		
		unsafe
		{
			if unlikely!(thread_identifier.is_none())
			{
				
				let current_thread_identifier: pid_t = ThreadIdentifier::default().into();
				let tag = Self::from_name_and_value("thread_identifier", &format!("{}", current_thread_identifier)).unwrap();
				thread_identifier = Some(tag);
			}
			
			thread_identifier.as_ref().unwrap()
		}
	}
	
	/// Value of `CARGO_PKG_NAME`.
	///
	/// Panics if package name far too long (exceedingly unlikely).
	#[inline(always)]
	pub fn cargo_name() -> &'static Self
	{
		lazy_static!
		{
			static ref cargo_name: DogStatsDTag = DogStatsDTag::from_name_and_value("cargo_name", env!("CARGO_PKG_NAME")).unwrap();
		}
		
		&cargo_name
	}
	
	/// Value of `CARGO_PKG_VERSION`.
	///
	/// Panics if version far too long (exceedingly unlikely).
	#[inline(always)]
	pub fn cargo_version() -> &'static Self
	{
		lazy_static!
		{
			static ref cargo_version: DogStatsDTag = DogStatsDTag::from_name_and_value("cargo_version", env!("CARGO_PKG_VERSION")).unwrap();
		}
		
		&cargo_version
	}
	
	/// Tag-value `env:<value>`.
	#[inline(always)]
	fn env(value: impl AsRef<[u8]>) -> Result<Self, String>
	{
		Self::from_name_and_value("env", value)
	}
	
	/// Tag-value `instance:<value>`.
	#[inline(always)]
	pub fn instance(value: impl AsRef<[u8]>) -> Result<Self, String>
	{
		Self::from_name_and_value("instance", value)
	}
	
	/// Tag-value `name:<value>`.
	#[inline(always)]
	pub fn name(value: impl AsRef<[u8]>) -> Result<Self, String>
	{
		Self::from_name_and_value("name", value)
	}
	
	/// Name should not end with `:`.
	/// Value must not end with `:` and must not be empty.
	#[inline(always)]
	pub fn from_name_and_value(name: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<Self, String>
	{
		const Colon: u8 = b':';
		
		#[inline(always)]
		fn validate_then_push(name_and_value: &mut Vec<u8>, bytes: &[u8], description: &'static str) -> Result<(), String>
		{
			let bytes = bytes.as_ref();
			
			if unlikely!(bytes.is_empty())
			{
				return Err(format!("{} is empty", description))
			}
			
			if unlikely!((* bytes.get_unchecked(bytes.len() - 1)) == Colon)
			{
				return Err(format!("{} `{:?}` ends with colon", description, bytes))
			}
			
			name_and_value.extend_from_slice(bytes);
			
			Ok(())
		}
		
		let name = name.as_ref();
		let value = value.as_ref();
		
		let mut name_and_value = Vec::with_capacity(name.len() + 1 + value.len());
		validate_then_push(&mut name_and_value, name, "name")?;
		name_and_value.push(Colon);
		validate_then_push(&mut name_and_value, value, "value")?;
		
		Self::from_name(name_and_value)
	}
	
	/// From name.
	pub fn from_name(name: impl AsRef<[u8]>) -> Result<Self, String>
	{
		let bytes = name.as_ref();
		
		let length = bytes.len();
		
		if unlikely!(length == 0)
		{
			return Err("Empty".to_string())
		}
		
		if unlikely!(length > Self::Length)
		{
			return Err(format!("Length `{}` exceeds maximum of {} in `{:?}`", length, Self::Length, bytes))
		}
		
		match unsafe { * bytes.get_unchecked(0) }
		{
			b'A' ..= b'Z' => (),
			b'a' ..= b'z' => (),
			first_byte @ _ => return Err(format!("First byte can not be '0x{:02X}' in `{:?}`", first_byte, bytes))
		}
		
		let final_byte_index = length - 1;
		
		let subsequent_bytes = &bytes[1 ..final_byte_index];
		for subsequent_byte in subsequent_bytes
		{
			match *subsequent_byte
			{
				b'0' ..= b'9' => (),
				b'A' ..= b'Z' => (),
				b'a' ..= b'z' => (),
				b'_' => (),
				b'.' => (),
				b'-' => (),
				b'/' => (),
				b':' => (),
				subsequent_byte @ _ => return Err(format!("Subsequent byte can not be '0x{:02X}' in `{:?}`", subsequent_byte, bytes))
			}
		}
		
		match unsafe { * bytes.get_unchecked(final_byte_index) }
		{
			b'0' ..= b'9' => (),
			b'A' ..= b'Z' => (),
			b'a' ..= b'z' => (),
			b'_' => (),
			b'.' => (),
			b'-' => (),
			b'/' => (),
			final_byte @ _ => return Err(format!("Final byte can not be '0x{:02X}' in `{:?}`", final_byte, bytes))
		}
		
		match bytes
		{
			b"host" | b"device" | b"source" | b"service" => Err(format!("Name can not be the reserved name `{:?}`", bytes)),
			_ =>
			{
				let mut array_vec = ArrayVec::new();
				array_vec.try_extend_from_slice(bytes).expect("Length check occurs above");
				Ok(Self(array_vec))
			}
		}
	}
	
	#[inline(always)]
	fn dog_stats_d_write(&self, dog_stats_d_writer: &mut DogStatsDWriter) -> Result<(), ()>
	{
		dog_stats_d_writer.write_array_vec(&self.0)
	}
}
