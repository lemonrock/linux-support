// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `BPF_PROG_LOAD`.
	#[allow(missing_docs)]
	pub(crate) struct BPF_PROG_LOAD_flags: u32
	{
		/// If `BPF_F_STRICT_ALIGNMENT` is used in `BPF_PROG_LOAD` command, the verifier will perform strict alignment checking as if the kernel has been built with `CONFIG_EFFICIENT_UNALIGNED_ACCESS` not set, and `NET_IP_ALIGN` defined to 2.
		const BPF_F_STRICT_ALIGNMENT = 1;
		
		/// If `BPF_F_ANY_ALIGNMENT` is used in `BPF_PROF_LOAD` command, the verifier will allow any alignment whatsoever.
		///
		/// On platforms with strict alignment requirements for loads ands stores (such as sparc and mips) the verifier validates that all loads and stores provably follow this requirement.
		/// This flag turns that checking and enforcement off.
		///
		/// It is mostly used for testing when we want to validate the context and memory access aspects of the verifier, but because of an unaligned access the alignment check would trigger before the one we are interested in.
		const BPF_F_ANY_ALIGNMENT = 2;
		
		/// `BPF_F_TEST_RND_HI32` is used in `BPF_PROG_LOAD` command for testing purpose.
		///
		/// Verifier does sub-register def/use analysis and identifies instructions whose definition only matters for low 32-bit, high 32-bit is never referenced later through implicit zero extension.
		/// Therefore verifier notifies JIT back-ends that it is safe to ignore clearing high 32-bit for these instructions.
		/// This saves some back-ends a lot of code-gen.
		/// However such optimization is not necessary on some arches, for example x86_64, arm64 etc, whose JIT back-ends hence hasn't used verifier's analysis result.
		/// But, we really want to have a way to be able to verify the correctness of the described optimization on* x86_64 on which testsuites are frequently exercised.
		///
		/// So, this flag is introduced.
		/// Once it is set, verifier will randomize high 32-bit for those instructions who has been identified as safe to ignore them
		/// Then, if verifier is not doing correct analysis, such randomization will regress tests to expose bugs.
		#[deprecated]
		const BPF_F_TEST_RND_HI32 = 4;

		/// The verifier internal test flag.
		///
		/// Behavior is undefined.
		#[deprecated]
		const BPF_F_TEST_STATE_FREQ = 8;
	}
}
