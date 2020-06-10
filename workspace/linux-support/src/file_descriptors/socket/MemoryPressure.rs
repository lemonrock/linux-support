// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory pressure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryPressure
{
	/// Low.
	pub low: NumberOfPages,
	
	/// Start reducing memory usage.
	pub medium: NumberOfPages,
	
	/// Maximum usage.
	pub maximum: NumberOfPages,
}

impl MemoryPressure
{
	/// Value of `/proc/sys/net/ipv4/tcp_mem`.
	#[inline(always)]
	pub fn global(proc_path: &ProcPath) -> io::Result<Self>
	{
		let line = Self::sys_net_ipv4_tcp_mem_file_path(proc_path).read_raw_without_line_feed()?;
		let mut fields = line.splitn(3, |byte| *byte == b'\t');
		
		#[inline(always)]
		fn parse_field<'a>(fields: &mut impl Iterator<Item=&'a [u8]>) -> io::Result<NumberOfPages>
		{
			NumberOfPages::from_bytes(fields.next().unwrap()).map_err(|cause| io::Error::new(ErrorKind::Other, cause))
		}
		
		Ok
		(
			Self
			{
				low: parse_field(&mut fields)?,
				medium: parse_field(&mut fields)?,
				maximum: parse_field(&mut fields)?,
			}
		)
	}
	
	/// Set value of `/proc/sys/net/ipv4/tcp_mem` if it exists.
	#[inline(always)]
	pub fn set_global(&self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert!(self.low < self.medium);
		assert!(self.medium < self.maximum);
		
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_wmem");
		
		let file_path = Self::sys_net_ipv4_tcp_mem_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(format!("{}\t{}\t{}", self.low, self.medium, self.maximum))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_ipv4_tcp_mem_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_ipv4_file_path("tcp_mem")
	}
}
