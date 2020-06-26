// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `BPF_MAP_CREATE` command.
	#[allow(missing_docs)]
	pub(crate) struct _bindgen_ty_2: u32
	{
		const BPF_F_NO_PREALLOC = 1;
		
		/// Instead of having one common LRU list in the `BPF_MAP_TYPE_LRU_[PERCPU_]HASH` map, use a percpu LRU list which can scale and perform better.
		/// Note, the LRU nodes (including free nodes) cannot be moved across different LRU lists.
		const BPF_F_NO_COMMON_LRU = 2;
		
		/// Specify numa node during map creation.
		const BPF_F_NUMA_NODE = 4;
		
		/// Flag for accessing BPF object from syscall side.
		const BPF_F_RDONLY = 8;
		
		/// Flag for accessing BPF object from syscall side.
		const BPF_F_WRONLY = 16;
		
		/// Flag for stack_map, store build_id+offset instead of pointer.
		const BPF_F_STACK_BUILD_ID = 32;
		
		/// Zero-initialize hash function seed.
		/// This should only be used for testing.
		#[deprecated]
		const BPF_F_ZERO_SEED = 64;
		
		/// Flag for accessing BPF object from program side.
		const BPF_F_RDONLY_PROG = 128;
		
		/// Flag for accessing BPF object from program side.
		const BPF_F_WRONLY_PROG = 256;
		
		/// Clone map from listener for newly accepted socket.
		const BPF_F_CLONE = 512;
		
		/// Enable memory-mapping BPF map.
		const BPF_F_MMAPABLE = 1024;
	}
}
