// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl<'a> From<&'a cpu_set_t> for BitSet<HyperThread>
{
	#[inline(always)]
	fn from(value: &'a cpu_set_t) -> Self
	{
		
		BitSet::new_from_words(value as *const cpu_set_t as *const usize, Self::CpuSetTSizeInWords)
	}
}

/// Returns a cpu_set_t and the size, in bytes, of the valid range of `HyperThread`s in the `cpu_set_t`.
///
/// HyperThreads outside of this range will be `0`, ie not in the `cpu_set_t`.
///
/// This seconds value is `<= size_of::<cpu_set_t>()`.
impl TryInto<(cpu_set_t, usize)> for BitSet<HyperThread>
{
	type Error = ();

	/// Fails if the set would contain more `HyperThread`s than can fit into `cpu_set_t` (currently this is more than 1024 `HyperThread`s).
	///
	/// Expands with unset `HyperThread`s if size if less than `cpu_set_t`.
	#[inline(always)]
	fn try_into(self) -> Result<(cpu_set_t, usize), Self::Error>
	{
		let size_in_words = Self::CpuSetTSizeInBytes / Self::UsizeSizeInBytes;

		if unlikely!(self.capacity() > size_in_words)
		{
			return Err(())
		}

		let cpu_set =
		{
			#[allow(deprecated)] let mut cpu_set = unsafe { uninitialized() };
			let (pointer, length) = self.to_raw_parts();
			let cpu_set_usize_pointer = &mut cpu_set as *mut cpu_set_t as *mut usize;
			unsafe { pointer.copy_to_nonoverlapping(cpu_set_usize_pointer, length) };
			unsafe { cpu_set_usize_pointer.offset(Self::CpuSetTSizeInWords as isize).write_bytes(0x00, length - Self::CpuSetTSizeInWords) };
			cpu_set
		};

		Ok((cpu_set, self.cpu_set_t_size_in_bytes()))
	}
}

impl BitSet<HyperThread>
{
	const CpuSetTSizeInBytes: usize = size_of::<cpu_set_t>();

	const UsizeSizeInBytes: usize = size_of::<usize>();

	/// Maximum C library Hyper Thread.
	pub const MaximumCLibrary: u16 = (Self::CpuSetTSizeInBytes * BitsInAByte) as u16;

	const CpuSetTSizeInWords: usize = (Self::MaximumCLibrary as usize) / Self::UsizeSizeInBytes;
	
	/// A new bit set suitable for converting to a cpu_set_t.
	#[inline(always)]
	pub fn new_for_cpu_set_t() -> Self
	{
		Self::with_capacity_in_words(Self::CpuSetTSizeInBytes / Self::UsizeSizeInBytes)
	}

	/// Set process affinity for current process.
	#[inline(always)]
	pub fn set_current_process_affinity(&self) -> Result<(), String>
	{
		self.set_process_affinity(0)
	}

	/// Set process affinity.
	#[inline(always)]
	pub fn set_process_affinity(&self, process_identifier: pid_t) -> Result<(), String>
	{
		let result = unsafe { sched_setaffinity(process_identifier, self.cpu_set_t_size_in_bytes(), self.cpu_set_t_pointer()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => Err("The affinity bit mask mask contains no processors that are currently physically on the system and permitted to the process according to any restrictions that may be imposed by the cpuset mechanism described in cpuset(7)".to_string()),

				EPERM => if unlikely!(process_identifier == 0)
				{
					panic!("Can not set our own affinity")
				}
				else
				{
					Err(format!("The calling process does not have appropriate privileges. The caller needs an effective user ID equal to the real user ID or effective user ID of the process identified by process_identifier {}, or it must possess the CAP_SYS_NICE capability", process_identifier).to_string())
				},

				ESRCH => if unlikely!(process_identifier == 0)
				{
					panic!("Can not set our own process affinity")
				}
				else
				{
					Err(format!("The thread whose ID is process_identifier '{}' could not be found", process_identifier).to_string())
				},

				EFAULT => panic!("A supplied memory address was invalid"),

				unknown @ _ => panic!("Unknown error number {}", unknown),
			}
		}
		else
		{
			panic!("Unexpected result {}", result)
		}
	}

	/// Set thread affinity for current thread.
	#[inline(always)]
	pub fn set_current_thread_affinity(&self) -> Result<(), String>
	{
		self.set_thread_affinity(unsafe { pthread_self() })
	}

	/// Set thread affinity.
	#[inline(always)]
	pub fn set_thread_affinity(&self, thread_identifier: pthread_t) -> Result<(), String>
	{
		let result = unsafe { pthread_setaffinity_np(thread_identifier, self.cpu_set_t_size_in_bytes(), self.cpu_set_t_pointer()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => Err("The affinity bit mask mask contains no processors that are currently physically on the system and permitted to the process according to any restrictions that may be imposed by the cpuset mechanism described in cpuset(7)".to_string()),

				ESRCH => if unlikely!(thread_identifier == pthread_self())
				{
					panic!("Can not set our own thread affinity")
				}
				else
				{
					Err(format!("The thread whose ID is thread_identifier '{:?}' could not be found", thread_identifier).to_string())
				},

				EFAULT => panic!("A supplied memory address was invalid"),

				unknown @ _ => panic!("Unknown error number {}", unknown),
			}
		}
		else
		{
			panic!("Unexpected result {}", result)
		}
	}

	#[inline(always)]
	fn cpu_set_t_pointer(&self) -> *const cpu_set_t
	{
		self.to_raw_parts().0 as _
	}

	#[inline(always)]
	fn cpu_set_t_size_in_bytes(&self) -> usize
	{
		self.capacity() * Self::UsizeSizeInBytes
	}
}
