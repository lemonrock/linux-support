// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct perf_event_attr
{
	pub(crate) type_: u32,
	pub(crate) size: u32,
	pub(crate) config: u64,
	pub(crate) __bindgen_anon_1: perf_event_attr__bindgen_ty_1,
	pub(crate) sample_type: u64,
	pub(crate) read_format: u64,
	pub(crate) _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize], u32>,
	pub(crate) __bindgen_anon_2: perf_event_attr__bindgen_ty_2,
	pub(crate) bp_type: u32,
	pub(crate) __bindgen_anon_3: perf_event_attr__bindgen_ty_3,
	pub(crate) __bindgen_anon_4: perf_event_attr__bindgen_ty_4,
	pub(crate) branch_sample_type: u64,
	pub(crate) sample_regs_user: u64,
	pub(crate) sample_stack_user: u32,
	pub(crate) clockid: i32,
	pub(crate) sample_regs_intr: u64,
	pub(crate) aux_watermark: u32,
	pub(crate) sample_max_stack: u16,
	pub(crate) __reserved_2: u16,
	pub(crate) aux_sample_size: u32,
	pub(crate) __reserved_3: u32,
}

impl Default for perf_event_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for perf_event_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "perf_event_attr {{ type: {:?}, size: {:?}, config: {:?}, __bindgen_anon_1: {:?}, sample_type: {:?}, read_format: {:?}, disabled: {:?}, inherit: {:?}, pinned: {:?}, exclusive: {:?}, exclude_user: {:?}, exclude_kernel: {:?}, exclude_hv: {:?}, exclude_idle: {:?}, mmap: {:?}, comm: {:?}, freq: {:?}, inherit_stat: {:?}, enable_on_exec: {:?}, task: {:?}, watermark: {:?}, precise_ip: {:?}, mmap_data: {:?}, sample_id_all: {:?}, exclude_host: {:?}, exclude_guest: {:?}, exclude_callchain_kernel: {:?}, exclude_callchain_user: {:?}, mmap2: {:?}, comm_exec: {:?}, use_clockid: {:?}, context_switch: {:?}, write_backward: {:?}, namespaces: {:?}, ksymbol: {:?}, bpf_event: {:?}, aux_output: {:?}, __reserved_1: {:?}, __bindgen_anon_2: {:?}, bp_type: {:?}, __bindgen_anon_3: {:?}, __bindgen_anon_4: {:?}, branch_sample_type: {:?}, sample_regs_user: {:?}, sample_stack_user: {:?}, clockid: {:?}, sample_regs_intr: {:?}, aux_watermark: {:?}, sample_max_stack: {:?}, __reserved_2: {:?}, aux_sample_size: {:?}, __reserved_3: {:?} }}", self.type_, self.size, self.config, self.__bindgen_anon_1, self.sample_type, self.read_format, self.disabled(), self.inherit(), self.pinned(), self.exclusive(), self.exclude_user(), self.exclude_kernel(), self.exclude_hv(), self.exclude_idle(), self.mmap(), self.comm(), self.freq(), self.inherit_stat(), self.enable_on_exec(), self.task(), self.watermark(), self.precise_ip(), self.mmap_data(), self.sample_id_all(), self.exclude_host(), self.exclude_guest(), self.exclude_callchain_kernel(), self.exclude_callchain_user(), self.mmap2(), self.comm_exec(), self.use_clockid(), self.context_switch(), self.write_backward(), self.namespaces(), self.ksymbol(), self.bpf_event(), self.aux_output(), self.__reserved_1(), self.__bindgen_anon_2, self.bp_type, self.__bindgen_anon_3, self.__bindgen_anon_4, self.branch_sample_type, self.sample_regs_user, self.sample_stack_user, self.clockid, self.sample_regs_intr, self.aux_watermark, self.sample_max_stack, self.__reserved_2, self.aux_sample_size, self.__reserved_3)
	}
}

impl perf_event_attr
{
	#[inline(always)]
	pub(crate) fn disabled(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_disabled(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(0usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn inherit(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(1usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_inherit(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(1usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn pinned(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(2usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_pinned(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(2usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn exclusive(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(3usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_exclusive(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(3usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn exclude_user(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(4usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_exclude_user(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(4usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn exclude_kernel(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(5usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_exclude_kernel(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(5usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn exclude_hv(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(6usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_exclude_hv(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(6usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn exclude_idle(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(7usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_exclude_idle(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(7usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mmap(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(8usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mmap(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(8usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn comm(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(9usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_comm(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(9usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn freq(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(10usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_freq(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(10usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn inherit_stat(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(11usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_inherit_stat(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(11usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn enable_on_exec(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(12usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_enable_on_exec(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(12usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn task(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(13usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_task(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(13usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn watermark(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(14usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_watermark(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(14usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn precise_ip(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(15usize, 2u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_precise_ip(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(15usize, 2u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mmap_data(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(17usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mmap_data(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(17usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn sample_id_all(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(18usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_sample_id_all(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(18usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn exclude_host(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(19usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_exclude_host(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(19usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn exclude_guest(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(20usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_exclude_guest(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(20usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn exclude_callchain_kernel(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(21usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_exclude_callchain_kernel(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(21usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn exclude_callchain_user(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(22usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_exclude_callchain_user(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(22usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn mmap2(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(23usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_mmap2(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(23usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn comm_exec(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(24usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_comm_exec(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(24usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn use_clockid(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(25usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_use_clockid(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(25usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn context_switch(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(26usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_context_switch(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(26usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn write_backward(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(27usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_write_backward(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(27usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn namespaces(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(28usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_namespaces(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(28usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn ksymbol(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(29usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_ksymbol(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(29usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn bpf_event(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(30usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_bpf_event(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(30usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn aux_output(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(31usize, 1) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set_aux_output(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(31usize, 1, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn __reserved_1(&self) -> u64
	{
		unsafe { transmute(self._bitfield_1.get(32usize, 32u8) as u64) }
	}

	#[inline(always)]
	pub(crate) fn set___reserved_1(&mut self, val: u64)
	{
		unsafe
		{
			let val: u64 = transmute(val);
			self._bitfield_1.set(32usize, 32u8, val as u64)
		}
	}

	#[inline(always)]
	pub(crate) fn new_bitfield_1
	(
		disabled: u64,
		inherit: u64,
		pinned: u64,
		exclusive: u64,
		exclude_user: u64,
		exclude_kernel: u64,
		exclude_hv: u64,
		exclude_idle: u64,
		mmap: u64,
		comm: u64,
		freq: u64,
		inherit_stat: u64,
		enable_on_exec: u64,
		task: u64,
		watermark: u64,
		precise_ip: u64,
		mmap_data: u64,
		sample_id_all: u64,
		exclude_host: u64,
		exclude_guest: u64,
		exclude_callchain_kernel: u64,
		exclude_callchain_user: u64,
		mmap2: u64,
		comm_exec: u64,
		use_clockid: u64,
		context_switch: u64,
		write_backward: u64,
		namespaces: u64,
		ksymbol: u64,
		bpf_event: u64,
		aux_output: u64,
		__reserved_1: u64,
	) -> __BindgenBitfieldUnit<[u8; 8], u32>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8], u32> = Default::default();
		__bindgen_bitfield_unit.set(0, 1,
		{
			let disabled: u64 = unsafe { transmute(disabled) };
			disabled as u64
		});
		__bindgen_bitfield_unit.set(1, 1,
		{
			let inherit: u64 = unsafe { transmute(inherit) };
			inherit as u64
		});
		__bindgen_bitfield_unit.set(2, 1,
		{
			let pinned: u64 = unsafe { transmute(pinned) };
			pinned as u64
		});
		__bindgen_bitfield_unit.set(3, 1,
		{
			let exclusive: u64 = unsafe { transmute(exclusive) };
			exclusive as u64
		});
		__bindgen_bitfield_unit.set(4, 1,
		{
			let exclude_user: u64 = unsafe { transmute(exclude_user) };
			exclude_user as u64
		});
		__bindgen_bitfield_unit.set(5, 1,
		{
			let exclude_kernel: u64 = unsafe { transmute(exclude_kernel) };
			exclude_kernel as u64
		});
		__bindgen_bitfield_unit.set(6, 1,
		{
			let exclude_hv: u64 = unsafe { transmute(exclude_hv) };
			exclude_hv as u64
		});
		__bindgen_bitfield_unit.set(7, 1,
		{
			let exclude_idle: u64 = unsafe { transmute(exclude_idle) };
			exclude_idle as u64
		});
		__bindgen_bitfield_unit.set(8, 1,
		{
			let mmap: u64 = unsafe { transmute(mmap) };
			mmap as u64
		});
		__bindgen_bitfield_unit.set(9, 1,
		{
			let comm: u64 = unsafe { transmute(comm) };
			comm as u64
		});
		__bindgen_bitfield_unit.set(10, 1,
		{
			let freq: u64 = unsafe { transmute(freq) };
			freq as u64
		});
		__bindgen_bitfield_unit.set(11, 1,
		{
			let inherit_stat: u64 = unsafe { transmute(inherit_stat) };
			inherit_stat as u64
		});
		__bindgen_bitfield_unit.set(12, 1,
		{
			let enable_on_exec: u64 = unsafe { transmute(enable_on_exec) };
			enable_on_exec as u64
		});
		__bindgen_bitfield_unit.set(13, 1,
		{
			let task: u64 = unsafe { transmute(task) };
			task as u64
		});
		__bindgen_bitfield_unit.set(14, 1,
		{
			let watermark: u64 = unsafe { transmute(watermark) };
			watermark as u64
		});
		__bindgen_bitfield_unit.set(15, 2u8,
		{
			let precise_ip: u64 = unsafe { transmute(precise_ip) };
			precise_ip as u64
		});
		__bindgen_bitfield_unit.set(17, 1,
		{
			let mmap_data: u64 = unsafe { transmute(mmap_data) };
			mmap_data as u64
		});
		__bindgen_bitfield_unit.set(18, 1,
		{
			let sample_id_all: u64 = unsafe { transmute(sample_id_all) };
			sample_id_all as u64
		});
		__bindgen_bitfield_unit.set(19, 1,
		{
			let exclude_host: u64 = unsafe { transmute(exclude_host) };
			exclude_host as u64
		});
		__bindgen_bitfield_unit.set(20, 1,
		{
			let exclude_guest: u64 = unsafe { transmute(exclude_guest) };
			exclude_guest as u64
		});
		__bindgen_bitfield_unit.set(21, 1,
		{
			let exclude_callchain_kernel: u64 =
				unsafe { transmute(exclude_callchain_kernel) };
			exclude_callchain_kernel as u64
		});
		__bindgen_bitfield_unit.set(22, 1,
		{
			let exclude_callchain_user: u64 =
				unsafe { transmute(exclude_callchain_user) };
			exclude_callchain_user as u64
		});
		__bindgen_bitfield_unit.set(23, 1,
		{
			let mmap2: u64 = unsafe { transmute(mmap2) };
			mmap2 as u64
		});
		__bindgen_bitfield_unit.set(24, 1,
		{
			let comm_exec: u64 = unsafe { transmute(comm_exec) };
			comm_exec as u64
		});
		__bindgen_bitfield_unit.set(25, 1,
		{
			let use_clockid: u64 = unsafe { transmute(use_clockid) };
			use_clockid as u64
		});
		__bindgen_bitfield_unit.set(26, 1,
		{
			let context_switch: u64 = unsafe { transmute(context_switch) };
			context_switch as u64
		});
		__bindgen_bitfield_unit.set(27, 1,
		{
			let write_backward: u64 = unsafe { transmute(write_backward) };
			write_backward as u64
		});
		__bindgen_bitfield_unit.set(28, 1,
		{
			let namespaces: u64 = unsafe { transmute(namespaces) };
			namespaces as u64
		});
		__bindgen_bitfield_unit.set(29, 1,
		{
			let ksymbol: u64 = unsafe { transmute(ksymbol) };
			ksymbol as u64
		});
		__bindgen_bitfield_unit.set(30, 1,
		{
			let bpf_event: u64 = unsafe { transmute(bpf_event) };
			bpf_event as u64
		});
		__bindgen_bitfield_unit.set(31, 1,
		{
			let aux_output: u64 = unsafe { transmute(aux_output) };
			aux_output as u64
		});
		__bindgen_bitfield_unit.set(32, 32,
		{
			let __reserved_1: u64 = unsafe { transmute(__reserved_1) };
			__reserved_1 as u64
		});
		__bindgen_bitfield_unit
	}
}
