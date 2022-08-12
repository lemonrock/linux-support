// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A system call number.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(EnumIter)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(usize)]
pub enum SystemCallNumber
{
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] _llseek = 140,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] _newselect = 5022,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] _newselect = 142,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] _sysctl = 5152,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] _sysctl = 149,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] _sysctl = 149,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] _sysctl = 156,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] accept = 202,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] accept = 5042,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] accept = 330,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] accept = 202,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] accept = 43,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] accept4 = 242,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] accept4 = 5293,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] accept4 = 344,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] accept4 = 242,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] accept4 = 364,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] accept4 = 288,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] access = 5020,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] access = 33,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] access = 33,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] access = 21,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] acct = 89,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] acct = 5158,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] acct = 51,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] acct = 89,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] acct = 51,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] acct = 163,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] add_key = 217,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] add_key = 5239,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] add_key = 269,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] add_key = 217,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] add_key = 278,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] add_key = 248,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] adjtimex = 171,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] adjtimex = 5154,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] adjtimex = 124,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] adjtimex = 171,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] adjtimex = 124,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] adjtimex = 159,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] afs_syscall = 5176,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] afs_syscall = 137,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] afs_syscall = 137,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] afs_syscall = 183,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] alarm = 5037,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] alarm = 27,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] alarm = 27,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] alarm = 37,
	
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] arch_prctl = 158,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] bdflush = 134,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] bdflush = 134,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] bind = 200,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] bind = 5048,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] bind = 327,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] bind = 200,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] bind = 361,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] bind = 49,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] bpf = 280,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] bpf = 5315,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] bpf = 361,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] bpf = 280,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] bpf = 351,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] bpf = 321,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] break_ = 17,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] brk = 214,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] brk = 5012,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] brk = 45,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] brk = 214,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] brk = 45,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] brk = 12,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] cachectl = 5198,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] cacheflush = 5197,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] capget = 90,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] capget = 5123,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] capget = 183,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] capget = 90,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] capget = 184,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] capget = 125,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] capset = 91,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] capset = 5124,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] capset = 184,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] capset = 91,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] capset = 185,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] capset = 126,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] chdir = 49,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] chdir = 5078,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] chdir = 12,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] chdir = 49,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] chdir = 12,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] chdir = 80,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] chmod = 5088,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] chmod = 15,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] chmod = 15,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] chmod = 90,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] chown = 5090,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] chown = 181,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] chown = 212,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] chown = 92,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] chroot = 51,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] chroot = 5156,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] chroot = 61,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] chroot = 51,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] chroot = 61,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] chroot = 161,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] clock_adjtime = 266,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] clock_adjtime = 5300,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] clock_adjtime = 347,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] clock_adjtime = 266,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] clock_adjtime = 337,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] clock_adjtime = 305,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] clock_getres = 114,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] clock_getres = 5223,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] clock_getres = 247,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] clock_getres = 114,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] clock_getres = 261,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] clock_getres = 229,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] clock_gettime = 113,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] clock_gettime = 5222,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] clock_gettime = 246,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] clock_gettime = 113,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] clock_gettime = 260,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] clock_gettime = 228,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] clock_nanosleep = 115,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] clock_nanosleep = 5224,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] clock_nanosleep = 248,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] clock_nanosleep = 115,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] clock_nanosleep = 262,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] clock_nanosleep = 230,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] clock_settime = 112,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] clock_settime = 5221,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] clock_settime = 245,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] clock_settime = 112,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] clock_settime = 259,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] clock_settime = 227,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] clone = 220,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] clone = 5055,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] clone = 120,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] clone = 220,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] clone = 120,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] clone = 56,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] clone3 = 435,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] clone3 = 5435,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] clone3 = 435,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] clone3 = 435,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] clone3 = 435,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] clone3 = 435,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] close = 57,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] close = 5003,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] close = 6,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] close = 57,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] close = 6,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] close = 3,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] connect = 203,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] connect = 5041,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] connect = 328,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] connect = 203,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] connect = 362,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] connect = 42,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] copy_file_range = 285,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] copy_file_range = 5320,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] copy_file_range = 379,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] copy_file_range = 285,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] copy_file_range = 375,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] copy_file_range = 326,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] creat = 5083,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] creat = 8,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] creat = 8,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] creat = 85,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] create_module = 5167,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] create_module = 127,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] create_module = 127,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] create_module = 174,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] delete_module = 106,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] delete_module = 5169,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] delete_module = 129,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] delete_module = 106,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] delete_module = 129,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] delete_module = 176,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] dup = 23,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] dup = 5031,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] dup = 41,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] dup = 23,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] dup = 41,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] dup = 32,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] dup2 = 5032,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] dup2 = 63,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] dup2 = 63,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] dup2 = 33,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] dup3 = 24,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] dup3 = 5286,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] dup3 = 316,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] dup3 = 24,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] dup3 = 326,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] dup3 = 292,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] epoll_create = 5207,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] epoll_create = 236,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] epoll_create = 249,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] epoll_create = 213,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] epoll_create1 = 20,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] epoll_create1 = 5285,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] epoll_create1 = 315,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] epoll_create1 = 20,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] epoll_create1 = 327,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] epoll_create1 = 291,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] epoll_ctl = 21,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] epoll_ctl = 5208,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] epoll_ctl = 237,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] epoll_ctl = 21,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] epoll_ctl = 250,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] epoll_ctl = 233,
	
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] epoll_ctl_old = 214,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] epoll_pwait = 22,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] epoll_pwait = 5272,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] epoll_pwait = 303,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] epoll_pwait = 22,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] epoll_pwait = 312,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] epoll_pwait = 281,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] epoll_wait = 5209,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] epoll_wait = 238,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] epoll_wait = 251,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] epoll_wait = 232,
	
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] epoll_wait_old = 215,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] eventfd = 5278,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] eventfd = 307,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] eventfd = 318,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] eventfd = 284,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] eventfd2 = 19,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] eventfd2 = 5284,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] eventfd2 = 314,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] eventfd2 = 19,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] eventfd2 = 323,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] eventfd2 = 290,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] execve = 221,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] execve = 5057,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] execve = 11,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] execve = 221,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] execve = 11,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] execve = 59,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] execveat = 281,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] execveat = 5316,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] execveat = 362,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] execveat = 281,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] execveat = 354,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] execveat = 322,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] exit = 93,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] exit = 5058,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] exit = 1,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] exit = 93,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] exit = 1,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] exit = 60,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] exit_group = 94,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] exit_group = 5205,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] exit_group = 234,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] exit_group = 94,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] exit_group = 248,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] exit_group = 231,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] faccessat = 48,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] faccessat = 5259,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] faccessat = 298,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] faccessat = 48,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] faccessat = 300,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] faccessat = 269,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fadvise64 = 223,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fadvise64 = 5215,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fadvise64 = 233,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fadvise64 = 223,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fadvise64 = 253,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fadvise64 = 221,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fallocate = 47,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fallocate = 5279,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fallocate = 309,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fallocate = 47,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fallocate = 314,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fallocate = 285,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fanotify_init = 262,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fanotify_init = 5295,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fanotify_init = 323,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fanotify_init = 262,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fanotify_init = 332,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fanotify_init = 300,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fanotify_mark = 263,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fanotify_mark = 5296,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fanotify_mark = 324,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fanotify_mark = 263,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fanotify_mark = 333,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fanotify_mark = 301,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fchdir = 50,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fchdir = 5079,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fchdir = 133,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fchdir = 50,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fchdir = 133,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fchdir = 81,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fchmod = 52,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fchmod = 5089,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fchmod = 94,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fchmod = 52,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fchmod = 94,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fchmod = 91,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fchmodat = 53,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fchmodat = 5258,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fchmodat = 297,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fchmodat = 53,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fchmodat = 299,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fchmodat = 268,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fchown = 55,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fchown = 5091,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fchown = 95,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fchown = 55,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fchown = 207,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fchown = 93,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fchownat = 54,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fchownat = 5250,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fchownat = 289,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fchownat = 54,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fchownat = 291,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fchownat = 260,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fcntl = 25,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fcntl = 5070,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fcntl = 55,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fcntl = 25,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fcntl = 55,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fcntl = 72,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fdatasync = 83,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fdatasync = 5073,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fdatasync = 148,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fdatasync = 83,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fdatasync = 148,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fdatasync = 75,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fgetxattr = 10,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fgetxattr = 5185,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fgetxattr = 214,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fgetxattr = 10,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fgetxattr = 229,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fgetxattr = 193,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] finit_module = 273,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] finit_module = 5307,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] finit_module = 353,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] finit_module = 273,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] finit_module = 344,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] finit_module = 313,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] flistxattr = 13,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] flistxattr = 5188,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] flistxattr = 217,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] flistxattr = 13,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] flistxattr = 232,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] flistxattr = 196,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] flock = 32,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] flock = 5071,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] flock = 143,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] flock = 32,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] flock = 143,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] flock = 73,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fork = 5056,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fork = 2,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fork = 2,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fork = 57,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fremovexattr = 16,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fremovexattr = 5191,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fremovexattr = 220,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fremovexattr = 16,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fremovexattr = 235,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fremovexattr = 199,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fsconfig = 431,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fsconfig = 5431,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fsconfig = 431,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fsconfig = 431,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fsconfig = 431,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fsconfig = 431,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fsetxattr = 7,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fsetxattr = 5182,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fsetxattr = 211,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fsetxattr = 7,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fsetxattr = 226,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fsetxattr = 190,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fsmount = 432,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fsmount = 5432,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fsmount = 432,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fsmount = 432,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fsmount = 432,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fsmount = 432,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fsopen = 430,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fsopen = 5430,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fsopen = 430,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fsopen = 430,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fsopen = 430,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fsopen = 430,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fspick = 433,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fspick = 5433,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fspick = 433,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fspick = 433,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fspick = 433,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fspick = 433,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fstat = 80,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fstat = 5005,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fstat = 108,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fstat = 80,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fstat = 108,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fstat = 5,
	
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fstatat = 79,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fstatfs = 44,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fstatfs = 5135,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fstatfs = 100,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fstatfs = 44,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fstatfs = 100,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fstatfs = 138,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fstatfs64 = 253,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fstatfs64 = 266,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] fsync = 82,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] fsync = 5072,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] fsync = 118,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] fsync = 82,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] fsync = 118,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] fsync = 74,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ftime = 35,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] ftruncate = 46,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] ftruncate = 5075,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ftruncate = 93,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] ftruncate = 46,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] ftruncate = 93,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] ftruncate = 77,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] futex = 98,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] futex = 5194,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] futex = 221,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] futex = 98,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] futex = 238,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] futex = 202,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] futimesat = 5251,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] futimesat = 290,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] futimesat = 292,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] futimesat = 261,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] get_kernel_syms = 5170,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] get_kernel_syms = 130,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] get_kernel_syms = 130,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] get_kernel_syms = 177,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] get_mempolicy = 236,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] get_mempolicy = 5228,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] get_mempolicy = 260,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] get_mempolicy = 236,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] get_mempolicy = 269,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] get_mempolicy = 239,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] get_robust_list = 100,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] get_robust_list = 5269,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] get_robust_list = 299,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] get_robust_list = 100,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] get_robust_list = 305,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] get_robust_list = 274,
	
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] get_thread_area = 211,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getcpu = 168,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getcpu = 5271,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getcpu = 302,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getcpu = 168,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getcpu = 311,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getcpu = 309,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getcwd = 17,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getcwd = 5077,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getcwd = 182,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getcwd = 17,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getcwd = 183,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getcwd = 79,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getdents = 5076,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getdents = 141,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getdents = 141,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getdents = 78,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getdents64 = 61,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getdents64 = 5308,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getdents64 = 202,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getdents64 = 61,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getdents64 = 220,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getdents64 = 217,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getegid = 177,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getegid = 5106,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getegid = 50,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getegid = 177,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getegid = 202,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getegid = 108,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] geteuid = 175,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] geteuid = 5105,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] geteuid = 49,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] geteuid = 175,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] geteuid = 201,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] geteuid = 107,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getgid = 176,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getgid = 5102,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getgid = 47,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getgid = 176,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getgid = 200,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getgid = 104,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getgroups = 158,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getgroups = 5113,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getgroups = 80,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getgroups = 158,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getgroups = 205,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getgroups = 115,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getitimer = 102,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getitimer = 5035,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getitimer = 105,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getitimer = 102,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getitimer = 105,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getitimer = 36,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getpeername = 205,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getpeername = 5051,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getpeername = 332,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getpeername = 205,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getpeername = 368,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getpeername = 52,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getpgid = 155,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getpgid = 5119,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getpgid = 132,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getpgid = 155,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getpgid = 132,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getpgid = 121,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getpgrp = 5109,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getpgrp = 65,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getpgrp = 65,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getpgrp = 111,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getpid = 172,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getpid = 5038,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getpid = 20,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getpid = 172,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getpid = 20,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getpid = 39,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getpmsg = 5174,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getpmsg = 187,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getpmsg = 188,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getpmsg = 181,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getppid = 173,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getppid = 5108,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getppid = 64,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getppid = 173,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getppid = 64,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getppid = 110,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getpriority = 141,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getpriority = 5137,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getpriority = 96,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getpriority = 141,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getpriority = 96,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getpriority = 140,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getrandom = 278,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getrandom = 5313,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getrandom = 359,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getrandom = 278,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getrandom = 349,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getrandom = 318,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getresgid = 150,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getresgid = 5118,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getresgid = 170,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getresgid = 150,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getresgid = 211,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getresgid = 120,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getresuid = 148,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getresuid = 5116,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getresuid = 165,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getresuid = 148,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getresuid = 209,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getresuid = 118,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getrlimit = 163,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getrlimit = 5095,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getrlimit = 76,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getrlimit = 163,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getrlimit = 191,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getrlimit = 97,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getrusage = 165,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getrusage = 5096,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getrusage = 77,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getrusage = 165,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getrusage = 77,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getrusage = 98,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getsid = 156,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getsid = 5122,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getsid = 147,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getsid = 156,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getsid = 147,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getsid = 124,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getsockname = 204,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getsockname = 5050,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getsockname = 331,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getsockname = 204,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getsockname = 367,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getsockname = 51,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getsockopt = 209,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getsockopt = 5054,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getsockopt = 340,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getsockopt = 209,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getsockopt = 365,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getsockopt = 55,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] gettid = 178,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] gettid = 5178,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] gettid = 207,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] gettid = 178,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] gettid = 236,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] gettid = 186,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] gettimeofday = 169,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] gettimeofday = 5094,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] gettimeofday = 78,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] gettimeofday = 169,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] gettimeofday = 78,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] gettimeofday = 96,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getuid = 174,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getuid = 5100,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getuid = 24,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getuid = 174,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getuid = 199,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getuid = 102,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] getxattr = 8,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] getxattr = 5183,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] getxattr = 212,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] getxattr = 8,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] getxattr = 227,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] getxattr = 191,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] gtty = 32,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] idle = 112,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] idle = 112,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] init_module = 105,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] init_module = 5168,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] init_module = 128,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] init_module = 105,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] init_module = 128,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] init_module = 175,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] inotify_add_watch = 27,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] inotify_add_watch = 5244,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] inotify_add_watch = 276,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] inotify_add_watch = 27,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] inotify_add_watch = 285,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] inotify_add_watch = 254,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] inotify_init = 5243,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] inotify_init = 275,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] inotify_init = 284,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] inotify_init = 253,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] inotify_init1 = 26,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] inotify_init1 = 5288,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] inotify_init1 = 318,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] inotify_init1 = 26,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] inotify_init1 = 324,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] inotify_init1 = 294,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] inotify_rm_watch = 28,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] inotify_rm_watch = 5245,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] inotify_rm_watch = 277,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] inotify_rm_watch = 28,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] inotify_rm_watch = 286,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] inotify_rm_watch = 255,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] io_cancel = 3,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] io_cancel = 5204,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] io_cancel = 231,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] io_cancel = 3,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] io_cancel = 247,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] io_cancel = 210,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] io_destroy = 1,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] io_destroy = 5201,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] io_destroy = 228,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] io_destroy = 1,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] io_destroy = 244,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] io_destroy = 207,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] io_getevents = 4,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] io_getevents = 5202,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] io_getevents = 229,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] io_getevents = 4,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] io_getevents = 245,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] io_getevents = 208,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] io_pgetevents = 292,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] io_pgetevents = 5328,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] io_pgetevents = 388,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] io_pgetevents = 292,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] io_pgetevents = 382,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] io_pgetevents = 333,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] io_setup = 0,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] io_setup = 5200,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] io_setup = 227,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] io_setup = 0,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] io_setup = 243,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] io_setup = 206,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] io_submit = 2,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] io_submit = 5203,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] io_submit = 230,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] io_submit = 2,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] io_submit = 246,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] io_submit = 209,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] io_uring_enter = 426,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] io_uring_enter = 5426,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] io_uring_enter = 426,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] io_uring_enter = 426,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] io_uring_enter = 426,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] io_uring_enter = 426,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] io_uring_register = 427,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] io_uring_register = 5427,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] io_uring_register = 427,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] io_uring_register = 427,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] io_uring_register = 427,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] io_uring_register = 427,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] io_uring_setup = 425,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] io_uring_setup = 5425,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] io_uring_setup = 425,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] io_uring_setup = 425,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] io_uring_setup = 425,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] io_uring_setup = 425,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] ioctl = 29,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] ioctl = 5015,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ioctl = 54,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] ioctl = 29,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] ioctl = 54,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] ioctl = 16,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ioperm = 101,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] ioperm = 173,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] iopl = 110,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] iopl = 172,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] ioprio_get = 31,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] ioprio_get = 5274,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ioprio_get = 274,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] ioprio_get = 31,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] ioprio_get = 283,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] ioprio_get = 252,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] ioprio_set = 30,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] ioprio_set = 5273,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ioprio_set = 273,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] ioprio_set = 30,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] ioprio_set = 282,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] ioprio_set = 251,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ipc = 117,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] ipc = 117,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] kcmp = 272,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] kcmp = 5306,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] kcmp = 354,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] kcmp = 272,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] kcmp = 343,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] kcmp = 312,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] kexec_file_load = 294,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] kexec_file_load = 382,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] kexec_file_load = 294,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] kexec_file_load = 381,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] kexec_file_load = 320,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] kexec_load = 104,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] kexec_load = 5270,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] kexec_load = 268,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] kexec_load = 104,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] kexec_load = 277,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] kexec_load = 246,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] keyctl = 219,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] keyctl = 5241,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] keyctl = 271,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] keyctl = 219,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] keyctl = 280,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] keyctl = 250,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] kill = 129,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] kill = 5060,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] kill = 37,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] kill = 129,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] kill = 37,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] kill = 62,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] lchown = 5092,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] lchown = 16,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] lchown = 198,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] lchown = 94,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] lgetxattr = 9,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] lgetxattr = 5184,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] lgetxattr = 213,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] lgetxattr = 9,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] lgetxattr = 228,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] lgetxattr = 192,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] link = 5084,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] link = 9,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] link = 9,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] link = 86,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] linkat = 37,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] linkat = 5255,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] linkat = 294,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] linkat = 37,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] linkat = 296,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] linkat = 265,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] listen = 201,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] listen = 5049,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] listen = 329,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] listen = 201,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] listen = 363,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] listen = 50,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] listxattr = 11,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] listxattr = 5186,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] listxattr = 215,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] listxattr = 11,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] listxattr = 230,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] listxattr = 194,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] llistxattr = 12,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] llistxattr = 5187,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] llistxattr = 216,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] llistxattr = 12,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] llistxattr = 231,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] llistxattr = 195,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] lock = 53,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] lookup_dcookie = 18,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] lookup_dcookie = 5206,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] lookup_dcookie = 235,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] lookup_dcookie = 18,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] lookup_dcookie = 110,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] lookup_dcookie = 212,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] lremovexattr = 15,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] lremovexattr = 5190,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] lremovexattr = 219,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] lremovexattr = 15,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] lremovexattr = 234,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] lremovexattr = 198,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] lseek = 62,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] lseek = 5008,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] lseek = 19,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] lseek = 62,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] lseek = 19,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] lseek = 8,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] lsetxattr = 6,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] lsetxattr = 5181,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] lsetxattr = 210,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] lsetxattr = 6,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] lsetxattr = 225,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] lsetxattr = 189,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] lstat = 5006,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] lstat = 107,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] lstat = 107,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] lstat = 6,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] madvise = 233,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] madvise = 5027,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] madvise = 205,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] madvise = 233,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] madvise = 219,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] madvise = 28,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mbind = 235,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mbind = 5227,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mbind = 259,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mbind = 235,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mbind = 268,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mbind = 237,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] membarrier = 283,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] membarrier = 5318,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] membarrier = 365,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] membarrier = 283,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] membarrier = 356,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] membarrier = 324,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] memfd_create = 279,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] memfd_create = 5314,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] memfd_create = 360,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] memfd_create = 279,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] memfd_create = 350,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] memfd_create = 319,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] migrate_pages = 238,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] migrate_pages = 5246,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] migrate_pages = 258,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] migrate_pages = 238,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] migrate_pages = 287,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] migrate_pages = 256,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mincore = 232,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mincore = 5026,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mincore = 206,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mincore = 232,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mincore = 218,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mincore = 27,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mkdir = 5081,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mkdir = 39,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mkdir = 39,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mkdir = 83,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mkdirat = 34,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mkdirat = 5248,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mkdirat = 287,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mkdirat = 34,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mkdirat = 289,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mkdirat = 258,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mknod = 5131,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mknod = 14,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mknod = 14,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mknod = 133,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mknodat = 33,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mknodat = 5249,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mknodat = 288,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mknodat = 33,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mknodat = 290,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mknodat = 259,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mlock = 228,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mlock = 5146,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mlock = 150,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mlock = 228,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mlock = 150,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mlock = 149,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mlock2 = 284,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mlock2 = 5319,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mlock2 = 378,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mlock2 = 284,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mlock2 = 374,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mlock2 = 325,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mlockall = 230,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mlockall = 5148,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mlockall = 152,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mlockall = 230,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mlockall = 152,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mlockall = 151,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mmap = 222,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mmap = 5009,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mmap = 90,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mmap = 222,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mmap = 90,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mmap = 9,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] modify_ldt = 123,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] modify_ldt = 154,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mount = 40,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mount = 5160,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mount = 21,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mount = 40,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mount = 21,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mount = 165,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] move_mount = 429,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] move_mount = 5429,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] move_mount = 429,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] move_mount = 429,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] move_mount = 429,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] move_mount = 429,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] move_pages = 239,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] move_pages = 5267,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] move_pages = 301,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] move_pages = 239,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] move_pages = 310,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] move_pages = 279,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mprotect = 226,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mprotect = 5010,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mprotect = 125,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mprotect = 226,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mprotect = 125,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mprotect = 10,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mpx = 56,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mq_getsetattr = 185,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mq_getsetattr = 5235,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mq_getsetattr = 267,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mq_getsetattr = 185,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mq_getsetattr = 276,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mq_getsetattr = 245,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mq_notify = 184,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mq_notify = 5234,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mq_notify = 266,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mq_notify = 184,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mq_notify = 275,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mq_notify = 244,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mq_open = 180,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mq_open = 5230,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mq_open = 262,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mq_open = 180,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mq_open = 271,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mq_open = 240,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mq_timedreceive = 183,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mq_timedreceive = 5233,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mq_timedreceive = 265,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mq_timedreceive = 183,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mq_timedreceive = 274,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mq_timedreceive = 243,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mq_timedsend = 182,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mq_timedsend = 5232,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mq_timedsend = 264,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mq_timedsend = 182,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mq_timedsend = 273,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mq_timedsend = 242,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mq_unlink = 181,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mq_unlink = 5231,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mq_unlink = 263,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mq_unlink = 181,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mq_unlink = 272,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mq_unlink = 241,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] mremap = 216,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] mremap = 5024,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] mremap = 163,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] mremap = 216,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] mremap = 163,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] mremap = 25,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] msgctl = 187,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] msgctl = 5069,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] msgctl = 402,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] msgctl = 187,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] msgctl = 402,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] msgctl = 71,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] msgget = 186,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] msgget = 5066,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] msgget = 399,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] msgget = 186,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] msgget = 399,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] msgget = 68,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] msgrcv = 188,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] msgrcv = 5068,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] msgrcv = 401,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] msgrcv = 188,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] msgrcv = 401,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] msgrcv = 70,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] msgsnd = 189,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] msgsnd = 5067,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] msgsnd = 400,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] msgsnd = 189,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] msgsnd = 400,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] msgsnd = 69,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] msync = 227,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] msync = 5025,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] msync = 144,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] msync = 227,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] msync = 144,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] msync = 26,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] multiplexer = 201,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] munlock = 229,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] munlock = 5147,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] munlock = 151,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] munlock = 229,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] munlock = 151,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] munlock = 150,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] munlockall = 231,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] munlockall = 5149,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] munlockall = 153,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] munlockall = 231,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] munlockall = 153,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] munlockall = 152,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] munmap = 215,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] munmap = 5011,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] munmap = 91,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] munmap = 215,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] munmap = 91,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] munmap = 11,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] name_to_handle_at = 264,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] name_to_handle_at = 5298,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] name_to_handle_at = 345,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] name_to_handle_at = 264,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] name_to_handle_at = 335,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] name_to_handle_at = 303,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] nanosleep = 101,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] nanosleep = 5034,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] nanosleep = 162,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] nanosleep = 101,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] nanosleep = 162,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] nanosleep = 35,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] newfstatat = 79,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] newfstatat = 5252,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] newfstatat = 291,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] newfstatat = 293,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] newfstatat = 262,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] nfsservctl = 42,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] nfsservctl = 5173,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] nfsservctl = 168,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] nfsservctl = 42,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] nfsservctl = 169,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] nfsservctl = 180,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] nice = 34,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] nice = 34,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] oldfstat = 28,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] oldlstat = 84,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] oldolduname = 59,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] oldstat = 18,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] olduname = 109,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] open = 5002,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] open = 5,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] open = 5,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] open = 2,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] open_by_handle_at = 265,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] open_by_handle_at = 5299,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] open_by_handle_at = 346,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] open_by_handle_at = 265,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] open_by_handle_at = 336,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] open_by_handle_at = 304,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] open_tree = 428,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] open_tree = 5428,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] open_tree = 428,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] open_tree = 428,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] open_tree = 428,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] open_tree = 428,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] openat = 56,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] openat = 5247,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] openat = 286,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] openat = 56,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] openat = 288,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] openat = 257,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] openat2 = 437,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] openat2 = 5437,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] openat2 = 437,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] openat2 = 437,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] openat2 = 437,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] openat2 = 437,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pause = 5033,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pause = 29,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pause = 29,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pause = 34,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pciconfig_iobase = 200,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pciconfig_read = 198,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pciconfig_write = 199,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] perf_event_open = 241,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] perf_event_open = 5292,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] perf_event_open = 319,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] perf_event_open = 241,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] perf_event_open = 331,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] perf_event_open = 298,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] personality = 92,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] personality = 5132,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] personality = 136,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] personality = 92,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] personality = 136,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] personality = 135,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pidfd_open = 434,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pidfd_open = 5434,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pidfd_open = 434,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pidfd_open = 434,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pidfd_open = 434,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pidfd_open = 434,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pidfd_getfd = 438,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pidfd_getfd = 5438,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pidfd_getfd = 438,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pidfd_getfd = 438,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pidfd_getfd = 438,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pidfd_getfd = 438,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pidfd_send_signal = 424,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pidfd_send_signal = 5424,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pidfd_send_signal = 424,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pidfd_send_signal = 424,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pidfd_send_signal = 424,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pidfd_send_signal = 424,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pipe = 5021,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pipe = 42,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pipe = 42,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pipe = 22,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pipe2 = 59,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pipe2 = 5287,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pipe2 = 317,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pipe2 = 59,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pipe2 = 325,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pipe2 = 293,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pivot_root = 41,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pivot_root = 5151,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pivot_root = 203,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pivot_root = 41,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pivot_root = 217,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pivot_root = 155,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pkey_alloc = 289,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pkey_alloc = 5324,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pkey_alloc = 384,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pkey_alloc = 289,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pkey_alloc = 385,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pkey_alloc = 330,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pkey_free = 290,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pkey_free = 5325,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pkey_free = 385,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pkey_free = 290,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pkey_free = 386,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pkey_free = 331,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pkey_mprotect = 288,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pkey_mprotect = 5323,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pkey_mprotect = 386,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pkey_mprotect = 288,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pkey_mprotect = 384,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pkey_mprotect = 329,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] poll = 5007,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] poll = 167,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] poll = 168,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] poll = 7,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] ppoll = 73,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] ppoll = 5261,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ppoll = 281,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] ppoll = 73,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] ppoll = 302,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] ppoll = 271,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] prctl = 167,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] prctl = 5153,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] prctl = 171,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] prctl = 167,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] prctl = 172,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] prctl = 157,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pread64 = 67,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pread64 = 5016,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pread64 = 179,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pread64 = 67,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pread64 = 180,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pread64 = 17,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] preadv = 69,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] preadv = 5289,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] preadv = 320,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] preadv = 69,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] preadv = 328,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] preadv = 295,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] preadv2 = 286,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] preadv2 = 5321,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] preadv2 = 380,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] preadv2 = 286,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] preadv2 = 376,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] preadv2 = 327,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] prlimit64 = 261,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] prlimit64 = 5297,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] prlimit64 = 325,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] prlimit64 = 261,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] prlimit64 = 334,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] prlimit64 = 302,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] process_vm_readv = 270,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] process_vm_readv = 5304,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] process_vm_readv = 351,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] process_vm_readv = 270,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] process_vm_readv = 340,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] process_vm_readv = 310,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] process_vm_writev = 271,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] process_vm_writev = 5305,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] process_vm_writev = 352,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] process_vm_writev = 271,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] process_vm_writev = 341,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] process_vm_writev = 311,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] prof = 44,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] profil = 98,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pselect6 = 72,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pselect6 = 5260,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pselect6 = 280,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pselect6 = 72,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pselect6 = 301,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pselect6 = 270,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] ptrace = 117,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] ptrace = 5099,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ptrace = 26,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] ptrace = 117,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] ptrace = 26,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] ptrace = 101,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] putpmsg = 5175,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] putpmsg = 188,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] putpmsg = 189,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] putpmsg = 182,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pwrite64 = 68,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pwrite64 = 5017,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pwrite64 = 180,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pwrite64 = 68,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pwrite64 = 181,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pwrite64 = 18,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pwritev = 70,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pwritev = 5290,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pwritev = 321,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pwritev = 70,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pwritev = 329,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pwritev = 296,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] pwritev2 = 287,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pwritev2 = 5322,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pwritev2 = 381,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] pwritev2 = 287,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pwritev2 = 377,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] pwritev2 = 328,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] query_module = 5171,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] query_module = 166,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] query_module = 167,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] query_module = 178,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] quotactl = 60,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] quotactl = 5172,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] quotactl = 131,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] quotactl = 60,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] quotactl = 131,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] quotactl = 179,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] read = 63,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] read = 5000,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] read = 3,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] read = 63,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] read = 3,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] read = 0,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] readahead = 213,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] readahead = 5179,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] readahead = 191,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] readahead = 213,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] readahead = 222,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] readahead = 187,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] readdir = 89,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] readdir = 89,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] readlink = 5087,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] readlink = 85,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] readlink = 85,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] readlink = 89,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] readlinkat = 78,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] readlinkat = 5257,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] readlinkat = 296,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] readlinkat = 78,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] readlinkat = 298,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] readlinkat = 267,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] readv = 65,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] readv = 5018,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] readv = 145,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] readv = 65,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] readv = 145,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] readv = 19,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] reboot = 142,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] reboot = 5164,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] reboot = 88,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] reboot = 142,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] reboot = 88,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] reboot = 169,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] recv = 336,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] recvfrom = 207,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] recvfrom = 5044,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] recvfrom = 337,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] recvfrom = 207,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] recvfrom = 371,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] recvfrom = 45,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] recvmmsg = 243,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] recvmmsg = 5294,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] recvmmsg = 343,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] recvmmsg = 243,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] recvmmsg = 357,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] recvmmsg = 299,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] recvmsg = 212,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] recvmsg = 5046,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] recvmsg = 342,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] recvmsg = 212,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] recvmsg = 372,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] recvmsg = 47,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] remap_file_pages = 234,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] remap_file_pages = 5210,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] remap_file_pages = 239,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] remap_file_pages = 234,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] remap_file_pages = 267,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] remap_file_pages = 216,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] removexattr = 14,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] removexattr = 5189,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] removexattr = 218,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] removexattr = 14,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] removexattr = 233,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] removexattr = 197,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rename = 5080,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rename = 38,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rename = 38,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rename = 82,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] renameat = 38,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] renameat = 5254,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] renameat = 293,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] renameat = 295,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] renameat = 264,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] renameat2 = 276,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] renameat2 = 5311,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] renameat2 = 357,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] renameat2 = 276,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] renameat2 = 347,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] renameat2 = 316,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] request_key = 218,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] request_key = 5240,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] request_key = 270,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] request_key = 218,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] request_key = 279,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] request_key = 249,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] reserved177 = 5177,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] reserved193 = 5193,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] restart_syscall = 128,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] restart_syscall = 5213,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] restart_syscall = 0,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] restart_syscall = 128,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] restart_syscall = 7,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] restart_syscall = 219,
	
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] riscv_flush_icache = 244 + 15,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rmdir = 5082,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rmdir = 40,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rmdir = 40,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rmdir = 84,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] rseq = 293,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rseq = 5327,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rseq = 387,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] rseq = 293,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rseq = 383,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rseq = 334,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] rt_sigaction = 134,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rt_sigaction = 5013,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rt_sigaction = 173,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] rt_sigaction = 134,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rt_sigaction = 174,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rt_sigaction = 13,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] rt_sigpending = 136,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rt_sigpending = 5125,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rt_sigpending = 175,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] rt_sigpending = 136,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rt_sigpending = 176,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rt_sigpending = 127,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] rt_sigprocmask = 135,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rt_sigprocmask = 5014,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rt_sigprocmask = 174,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] rt_sigprocmask = 135,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rt_sigprocmask = 175,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rt_sigprocmask = 14,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] rt_sigqueueinfo = 138,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rt_sigqueueinfo = 5127,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rt_sigqueueinfo = 177,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] rt_sigqueueinfo = 138,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rt_sigqueueinfo = 178,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rt_sigqueueinfo = 129,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] rt_sigreturn = 139,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rt_sigreturn = 5211,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rt_sigreturn = 172,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] rt_sigreturn = 139,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rt_sigreturn = 173,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rt_sigreturn = 15,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] rt_sigsuspend = 133,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rt_sigsuspend = 5128,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rt_sigsuspend = 178,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] rt_sigsuspend = 133,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rt_sigsuspend = 179,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rt_sigsuspend = 130,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] rt_sigtimedwait = 137,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rt_sigtimedwait = 5126,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rt_sigtimedwait = 176,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] rt_sigtimedwait = 137,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rt_sigtimedwait = 177,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rt_sigtimedwait = 128,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] rt_tgsigqueueinfo = 240,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] rt_tgsigqueueinfo = 5291,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rt_tgsigqueueinfo = 322,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] rt_tgsigqueueinfo = 240,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] rt_tgsigqueueinfo = 330,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] rt_tgsigqueueinfo = 297,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] rtas = 255,
	
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] s390_guarded_storage = 378,
	
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] s390_pci_mmio_read = 353,
	
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] s390_pci_mmio_write = 352,
	
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] s390_runtime_instr = 342,
	
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] s390_sthyi = 380,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_get_priority_max = 125,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_get_priority_max = 5143,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_get_priority_max = 159,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_get_priority_max = 125,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_get_priority_max = 159,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_get_priority_max = 146,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_get_priority_min = 126,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_get_priority_min = 5144,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_get_priority_min = 160,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_get_priority_min = 126,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_get_priority_min = 160,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_get_priority_min = 147,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_getaffinity = 123,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_getaffinity = 5196,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_getaffinity = 223,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_getaffinity = 123,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_getaffinity = 240,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_getaffinity = 204,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_getattr = 275,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_getattr = 5310,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_getattr = 356,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_getattr = 275,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_getattr = 346,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_getattr = 315,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_getparam = 121,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_getparam = 5140,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_getparam = 155,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_getparam = 121,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_getparam = 155,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_getparam = 143,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_getscheduler = 120,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_getscheduler = 5142,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_getscheduler = 157,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_getscheduler = 120,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_getscheduler = 157,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_getscheduler = 145,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_rr_get_interval = 127,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_rr_get_interval = 5145,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_rr_get_interval = 161,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_rr_get_interval = 127,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_rr_get_interval = 161,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_rr_get_interval = 148,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_setaffinity = 122,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_setaffinity = 5195,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_setaffinity = 222,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_setaffinity = 122,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_setaffinity = 239,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_setaffinity = 203,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_setattr = 274,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_setattr = 5309,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_setattr = 355,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_setattr = 274,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_setattr = 345,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_setattr = 314,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_setparam = 118,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_setparam = 5139,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_setparam = 154,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_setparam = 118,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_setparam = 154,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_setparam = 142,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_setscheduler = 119,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_setscheduler = 5141,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_setscheduler = 156,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_setscheduler = 119,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_setscheduler = 156,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_setscheduler = 144,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sched_yield = 124,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sched_yield = 5023,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sched_yield = 158,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sched_yield = 124,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sched_yield = 158,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sched_yield = 24,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] seccomp = 277,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] seccomp = 5312,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] seccomp = 358,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] seccomp = 277,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] seccomp = 348,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] seccomp = 317,
	
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] security = 185,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] select = 82,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] select = 142,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] select = 23,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] semctl = 191,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] semctl = 5064,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] semctl = 394,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] semctl = 191,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] semctl = 394,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] semctl = 66,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] semget = 190,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] semget = 5062,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] semget = 393,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] semget = 190,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] semget = 393,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] semget = 64,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] semop = 193,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] semop = 5063,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] semop = 193,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] semop = 65,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] semtimedop = 192,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] semtimedop = 5214,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] semtimedop = 392,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] semtimedop = 192,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] semtimedop = 392,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] semtimedop = 220,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] send = 334,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sendfile = 71,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sendfile = 5039,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sendfile = 186,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sendfile = 71,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sendfile = 187,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sendfile = 40,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sendmmsg = 269,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sendmmsg = 5302,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sendmmsg = 349,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sendmmsg = 269,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sendmmsg = 358,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sendmmsg = 307,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sendmsg = 211,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sendmsg = 5045,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sendmsg = 341,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sendmsg = 211,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sendmsg = 370,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sendmsg = 46,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sendto = 206,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sendto = 5043,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sendto = 335,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sendto = 206,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sendto = 369,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sendto = 44,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] set_mempolicy = 237,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] set_mempolicy = 5229,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] set_mempolicy = 261,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] set_mempolicy = 237,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] set_mempolicy = 270,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] set_mempolicy = 238,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] set_robust_list = 99,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] set_robust_list = 5268,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] set_robust_list = 300,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] set_robust_list = 99,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] set_robust_list = 304,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] set_robust_list = 273,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] set_thread_area = 5242,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] set_thread_area = 205,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] set_tid_address = 96,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] set_tid_address = 5212,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] set_tid_address = 232,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] set_tid_address = 96,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] set_tid_address = 252,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] set_tid_address = 218,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setdomainname = 162,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setdomainname = 5166,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setdomainname = 121,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setdomainname = 162,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setdomainname = 121,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setdomainname = 171,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setfsgid = 152,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setfsgid = 5121,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setfsgid = 139,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setfsgid = 152,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setfsgid = 216,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setfsgid = 123,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setfsuid = 151,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setfsuid = 5120,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setfsuid = 138,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setfsuid = 151,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setfsuid = 215,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setfsuid = 122,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setgid = 144,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setgid = 5104,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setgid = 46,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setgid = 144,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setgid = 214,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setgid = 106,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setgroups = 159,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setgroups = 5114,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setgroups = 81,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setgroups = 159,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setgroups = 206,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setgroups = 116,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sethostname = 161,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sethostname = 5165,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sethostname = 74,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sethostname = 161,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sethostname = 74,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sethostname = 170,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setitimer = 103,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setitimer = 5036,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setitimer = 104,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setitimer = 103,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setitimer = 104,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setitimer = 38,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setns = 268,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setns = 5303,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setns = 350,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setns = 268,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setns = 339,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setns = 308,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setpgid = 154,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setpgid = 5107,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setpgid = 57,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setpgid = 154,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setpgid = 57,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setpgid = 109,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setpriority = 140,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setpriority = 5138,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setpriority = 97,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setpriority = 140,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setpriority = 97,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setpriority = 141,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setregid = 143,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setregid = 5112,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setregid = 71,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setregid = 143,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setregid = 204,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setregid = 114,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setresgid = 149,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setresgid = 5117,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setresgid = 169,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setresgid = 149,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setresgid = 210,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setresgid = 119,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setresuid = 147,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setresuid = 5115,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setresuid = 164,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setresuid = 147,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setresuid = 208,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setresuid = 117,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setreuid = 145,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setreuid = 5111,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setreuid = 70,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setreuid = 145,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setreuid = 203,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setreuid = 113,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setrlimit = 164,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setrlimit = 5155,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setrlimit = 75,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setrlimit = 164,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setrlimit = 75,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setrlimit = 160,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setsid = 157,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setsid = 5110,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setsid = 66,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setsid = 157,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setsid = 66,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setsid = 112,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setsockopt = 208,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setsockopt = 5053,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setsockopt = 339,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setsockopt = 208,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setsockopt = 366,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setsockopt = 54,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] settimeofday = 170,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] settimeofday = 5159,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] settimeofday = 79,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] settimeofday = 170,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] settimeofday = 79,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] settimeofday = 164,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setuid = 146,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setuid = 5103,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setuid = 23,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setuid = 146,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setuid = 213,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setuid = 105,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] setxattr = 5,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] setxattr = 5180,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] setxattr = 209,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] setxattr = 5,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] setxattr = 224,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] setxattr = 188,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sgetmask = 68,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] shmat = 196,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] shmat = 5029,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] shmat = 397,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] shmat = 196,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] shmat = 397,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] shmat = 30,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] shmctl = 195,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] shmctl = 5030,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] shmctl = 396,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] shmctl = 195,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] shmctl = 396,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] shmctl = 31,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] shmdt = 197,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] shmdt = 5065,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] shmdt = 398,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] shmdt = 197,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] shmdt = 398,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] shmdt = 67,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] shmget = 194,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] shmget = 5028,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] shmget = 395,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] shmget = 194,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] shmget = 395,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] shmget = 29,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] shutdown = 210,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] shutdown = 5047,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] shutdown = 338,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] shutdown = 210,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] shutdown = 373,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] shutdown = 48,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sigaction = 67,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sigaction = 67,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sigaltstack = 132,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sigaltstack = 5129,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sigaltstack = 185,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sigaltstack = 132,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sigaltstack = 186,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sigaltstack = 131,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] signal = 48,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] signal = 48,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] signalfd = 5276,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] signalfd = 305,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] signalfd = 316,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] signalfd = 282,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] signalfd4 = 74,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] signalfd4 = 5283,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] signalfd4 = 313,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] signalfd4 = 74,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] signalfd4 = 322,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] signalfd4 = 289,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sigpending = 73,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sigpending = 73,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sigprocmask = 126,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sigprocmask = 126,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sigreturn = 119,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sigreturn = 119,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sigsuspend = 72,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sigsuspend = 72,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] socket = 198,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] socket = 5040,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] socket = 326,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] socket = 198,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] socket = 359,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] socket = 41,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] socketcall = 102,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] socketcall = 102,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] socketpair = 199,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] socketpair = 5052,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] socketpair = 333,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] socketpair = 199,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] socketpair = 360,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] socketpair = 53,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] splice = 76,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] splice = 5263,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] splice = 283,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] splice = 76,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] splice = 306,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] splice = 275,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] spu_create = 279,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] spu_run = 278,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ssetmask = 69,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] stat = 5004,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] stat = 106,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] stat = 106,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] stat = 4,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] statfs = 43,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] statfs = 5134,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] statfs = 99,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] statfs = 43,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] statfs = 99,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] statfs = 137,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] statfs64 = 252,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] statfs64 = 265,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] statx = 291,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] statx = 5326,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] statx = 383,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] statx = 291,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] statx = 379,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] statx = 332,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] stime = 25,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] stty = 31,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] subpage_prot = 310,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] swapcontext = 249,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] swapoff = 225,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] swapoff = 5163,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] swapoff = 115,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] swapoff = 225,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] swapoff = 115,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] swapoff = 168,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] swapon = 224,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] swapon = 5162,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] swapon = 87,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] swapon = 224,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] swapon = 87,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] swapon = 167,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] switch_endian = 363,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] symlink = 5086,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] symlink = 83,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] symlink = 83,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] symlink = 88,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] symlinkat = 36,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] symlinkat = 5256,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] symlinkat = 295,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] symlinkat = 36,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] symlinkat = 297,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] symlinkat = 266,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sync = 81,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sync = 5157,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sync = 36,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sync = 81,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sync = 36,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sync = 162,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sync_file_range = 84,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sync_file_range = 5264,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sync_file_range = 84,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sync_file_range = 307,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sync_file_range = 277,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sync_file_range2 = 308,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] syncfs = 267,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] syncfs = 5301,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] syncfs = 348,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] syncfs = 267,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] syncfs = 338,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] syncfs = 306,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sys_debug_setcontext = 256,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sysfs = 5136,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sysfs = 135,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sysfs = 135,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sysfs = 139,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] sysinfo = 179,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sysinfo = 5097,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] sysinfo = 116,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sysinfo = 179,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] sysinfo = 116,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] sysinfo = 99,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] syslog = 116,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] syslog = 5101,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] syslog = 103,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] syslog = 116,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] syslog = 103,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] syslog = 103,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] sysmips = 5199,
	
	/// Also known as __NR_arch_specific_syscall.
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] sysriscv = 244,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] tee = 77,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] tee = 5265,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] tee = 284,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] tee = 77,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] tee = 308,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] tee = 276,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] tgkill = 131,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] tgkill = 5225,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] tgkill = 250,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] tgkill = 131,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] tgkill = 241,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] tgkill = 234,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] time = 13,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] time = 201,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] timer_create = 107,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] timer_create = 5216,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] timer_create = 240,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] timer_create = 107,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] timer_create = 254,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] timer_create = 222,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] timer_delete = 111,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] timer_delete = 5220,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] timer_delete = 244,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] timer_delete = 111,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] timer_delete = 258,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] timer_delete = 226,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] timer_getoverrun = 109,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] timer_getoverrun = 5219,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] timer_getoverrun = 243,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] timer_getoverrun = 109,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] timer_getoverrun = 257,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] timer_getoverrun = 225,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] timer_gettime = 108,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] timer_gettime = 5218,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] timer_gettime = 242,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] timer_gettime = 108,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] timer_gettime = 256,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] timer_gettime = 224,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] timer_settime = 110,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] timer_settime = 5217,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] timer_settime = 241,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] timer_settime = 110,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] timer_settime = 255,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] timer_settime = 223,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] timerfd = 5277,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] timerfd = 317,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] timerfd_create = 85,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] timerfd_create = 5280,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] timerfd_create = 306,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] timerfd_create = 85,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] timerfd_create = 319,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] timerfd_create = 283,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] timerfd_gettime = 87,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] timerfd_gettime = 5281,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] timerfd_gettime = 312,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] timerfd_gettime = 87,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] timerfd_gettime = 321,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] timerfd_gettime = 287,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] timerfd_settime = 86,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] timerfd_settime = 5282,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] timerfd_settime = 311,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] timerfd_settime = 86,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] timerfd_settime = 320,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] timerfd_settime = 286,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] times = 153,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] times = 5098,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] times = 43,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] times = 153,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] times = 43,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] times = 100,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] tkill = 130,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] tkill = 5192,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] tkill = 208,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] tkill = 130,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] tkill = 237,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] tkill = 200,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] truncate = 45,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] truncate = 5074,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] truncate = 92,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] truncate = 45,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] truncate = 92,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] truncate = 76,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] tuxcall = 225,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] tuxcall = 184,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ugetrlimit = 190,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ulimit = 58,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] umask = 166,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] umask = 5093,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] umask = 60,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] umask = 166,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] umask = 60,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] umask = 95,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] umount = 22,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] umount = 22,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] umount2 = 39,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] umount2 = 5161,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] umount2 = 52,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] umount2 = 39,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] umount2 = 52,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] umount2 = 166,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] uname = 160,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] uname = 5061,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] uname = 122,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] uname = 160,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] uname = 122,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] uname = 63,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] unlink = 5085,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] unlink = 10,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] unlink = 10,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] unlink = 87,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] unlinkat = 35,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] unlinkat = 5253,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] unlinkat = 292,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] unlinkat = 35,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] unlinkat = 294,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] unlinkat = 263,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] unshare = 97,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] unshare = 5262,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] unshare = 282,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] unshare = 97,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] unshare = 303,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] unshare = 272,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] uselib = 86,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] uselib = 86,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] uselib = 134,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] userfaultfd = 282,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] userfaultfd = 5317,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] userfaultfd = 364,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] userfaultfd = 282,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] userfaultfd = 355,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] userfaultfd = 323,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] ustat = 5133,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] ustat = 62,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] ustat = 62,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] ustat = 136,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] utime = 5130,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] utime = 30,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] utime = 30,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] utime = 132,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] utimensat = 88,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] utimensat = 5275,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] utimensat = 304,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] utimensat = 88,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] utimensat = 315,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] utimensat = 280,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] utimes = 5226,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] utimes = 251,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] utimes = 313,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] utimes = 235,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] vfork = 189,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] vfork = 190,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] vfork = 58,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] vhangup = 58,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] vhangup = 5150,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] vhangup = 111,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] vhangup = 58,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] vhangup = 111,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] vhangup = 153,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] vm86 = 113,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] vmsplice = 75,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] vmsplice = 5266,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] vmsplice = 285,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] vmsplice = 75,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] vmsplice = 309,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] vmsplice = 278,
	
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] vserver = 5236,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] vserver = 236,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] wait4 = 260,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] wait4 = 5059,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] wait4 = 114,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] wait4 = 260,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] wait4 = 114,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] wait4 = 61,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] waitid = 95,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] waitid = 5237,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] waitid = 272,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] waitid = 95,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] waitid = 281,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] waitid = 247,
	
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] waitpid = 7,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] write = 64,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] write = 5001,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] write = 4,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] write = 64,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] write = 4,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] write = 1,
	
	#[allow(missing_docs)] #[cfg(target_arch = "aarch64")] writev = 66,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] writev = 5019,
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] writev = 146,
	#[allow(missing_docs)] #[cfg(target_arch = "riscv64")] writev = 66,
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] writev = 146,
	#[allow(missing_docs)] #[cfg(target_arch = "x86_64")] writev = 20,
}

impl const AsUsizeIndex for SystemCallNumber
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.0
	}
}

impl SystemCallNumber
{
	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))] pub const InclusiveMinimum: Self = SystemCallNumber::io_setup;
	#[allow(missing_docs)] #[cfg(target_arch = "powerpc64")] pub const InclusiveMinimum: Self = SystemCallNumber::restart_syscall;
	#[allow(missing_docs)] #[cfg(target_arch = "s390x")] pub const InclusiveMinimum: Self = SystemCallNumber::exit;
	#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "x86_64"))] pub const InclusiveMinimum: Self = SystemCallNumber::read;
	
	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "mips64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] pub const InclusiveMaximum: Self = SystemCallNumber::pidfd_getfd;
	
	#[cfg(target_arch = "aarch64")] #[allow(missing_docs)] pub const Undefined: [Range<u32>; 3] =
	[
		244 .. 260,
		295 .. 424,
		436 .. 437,
	];
	#[cfg(target_arch = "mips64")] #[allow(missing_docs)] pub const Undefined: [Range<u32>; 3] =
	[
		5238 .. 5239,
		5329 .. 5424,
		5436 .. 5437,
	];
	#[cfg(target_arch = "powerpc64")] #[allow(missing_docs)] pub const Undefined: [Range<u32>; 10] =
	[
		192 .. 198,
		204 .. 205,
		224 .. 225,
		226 .. 227,
		254 .. 255,
		257 .. 258,
		366 .. 378,
		389 .. 392,
		403 .. 424,
		436 .. 437,
	];
	#[cfg(target_arch = "riscv64")] #[allow(missing_docs)] pub const Undefined: [Range<u32>; 4] =
	[
		38 .. 39,
		245 .. 260,
		295 .. 424,
		436 .. 437,
	];
	#[cfg(target_arch = "s390x")] #[allow(missing_docs)] pub const Undefined: [Range<u32>; 34] =
	[
		13 .. 14,
		16 .. 19,
		23 .. 26,
		28 .. 29,
		31 .. 33,
		35 .. 36,
		44 .. 45,
		46 .. 48,
		49 .. 51,
		53 .. 54,
		56 .. 57,
		58 .. 60,
		68 .. 72,
		76 .. 77,
		80 .. 83,
		84 .. 85,
		95 .. 96,
		98 .. 99,
		101 .. 102,
		109 .. 110,
		113 .. 114,
		123 .. 124,
		138 .. 141,
		164 .. 167,
		170 .. 172,
		182 .. 183,
		192 .. 198,
		221 .. 222,
		223 .. 224,
		242 .. 243,
		263 .. 265,
		387 .. 392,
		403 .. 424,
		436 .. 437,
	];
	#[cfg(target_arch = "x86_64")] #[allow(missing_docs)] pub const Undefined: [Range<u32>; 2] =
	[
		335 .. 424,
		436 .. 437,
	];
}

#[cfg(target_arch = "aarch64")] include!("SystemCallNumber.aarch64.rs");
#[cfg(target_arch = "riscv64")] include!("SystemCallNumber.riscv64.rs");
#[cfg(target_arch = "x86_64")] include!("SystemCallNumber.x86_64.rs");

impl SystemCallNumber
{
	/// Generic system call equivalent to libc's `long syscall(long n, ...)` function.
	#[inline(always)]
	pub unsafe fn system_call<SCA: SystemCallArguments>(self, arguments: SCA) -> SystemCallResult
	{
		arguments.system_call(self)
	}
	
	#[inline(always)]
	pub(crate) fn system_call_bpf<const size: u32>(cmd: bpf_cmd, attr: &mut bpf_attr) -> c_int
	{
		unsafe { SystemCallNumber::bpf.system_call_3(cmd as i32 as usize, attr as *mut bpf_attr as usize, size as usize) as c_int }
	}
	
	#[inline(always)]
	pub(crate) fn system_call_delete_module(name: &CStr, flags: c_long) -> SystemCallResult
	{
		unsafe { SystemCallNumber::delete_module.system_call_2(name.as_ptr() as usize, flags as usize) }
	}
	
	#[inline(always)]
	pub(crate) fn system_call_finit_module(file_descriptor: RawFd, options: &CStr, flags: c_int) -> c_int
	{
		unsafe { SystemCallNumber::finit_module.system_call_3( file_descriptor as usize, options.as_ptr() as usize, flags as usize) as c_int }
	}
	
	/// Note:-
	///
	/// * `nodemask` can be `null()`; in which case, `maxnode` is `0`.
	/// * When `flags` is `0`, addr must be specified as `NULL`.
	///
	/// See <http://man7.org/linux/man-pages/man2/get_mempolicy.2.html>.
	#[inline(always)]
	pub(crate) fn system_call_get_mempolicy(mode: *mut i32, nodemask: *mut usize, maxnode: usize, addr: *const c_void, flags: GetMemoryPolicyFlags) -> isize
	{
		let flags = flags.bits();
		
		if cfg!(debug_assertions)
		{
			if nodemask.is_null()
			{
				debug_assert_eq!(maxnode, 0, "maxnode must be 0 if nodemask is null")
			}
			if flags == 0
			{
				debug_assert!(addr.is_null(), "when flags is 0 addr must be null")
			}
		}
		unsafe { SystemCallNumber::get_mempolicy.system_call_5(mode as usize, nodemask as usize, maxnode, addr as usize, flags as usize) as isize }
	}
	
	#[inline(always)]
	pub(crate) fn system_call_io_uring_enter(fd: RawFd, to_submit: c_uint, min_complete: c_uint, flags: EnterFlags, sig: *mut sigset_t) -> c_int
	{
		let flags = flags.bits();
		unsafe { SystemCallNumber::io_uring_enter.system_call_6(fd as usize, to_submit as usize, min_complete as usize, flags as usize, sig as usize, (_NSIG / 8) as usize) as c_int }
	}
	
	/// `fd` is the file descriptor returned by a call to `io_uring_setup()`.
	/// `args` is a list with `nr_args` elements.
	/// The type of `args` depends on the value of `opcode`.
	///
	/// If `arg` is `NULL` then `nr_args` must be `0`.
	#[inline(always)]
	pub(crate) fn system_call_io_uring_register(fd: RawFd, opcode: RegisterOperation, arg: *mut c_void, nr_args: c_uint) -> c_int
	{
		if cfg!(debug_assertions)
		{
			if arg.is_null()
			{
				debug_assert_eq!(nr_args, 0, "nr_args must be 0 if arg is null")
			}
		}
		
		let opcode = opcode as u32;
		unsafe { SystemCallNumber::io_uring_register.system_call_4(fd as usize, opcode as usize, arg as usize, nr_args as usize) as c_int }
	}
	
	#[inline(always)]
	pub(crate) fn system_call_io_uring_setup(entries: c_uint, p: &mut io_uring_params) -> c_int
	{
		unsafe { SystemCallNumber::io_uring_setup.system_call_2(entries as usize, p as *mut io_uring_params as usize) as c_int }
	}
	
	#[inline(always)]
	pub(crate) fn system_call_ioprio_get(which: c_int, who: IOPRIO_WHO) -> c_int
	{
		let who = who as i32;
		unsafe { SystemCallNumber::ioprio_get.system_call_2(which as usize, who as usize) as c_int }
	}
	
	#[inline(always)]
	pub(crate) fn system_call_ioprio_set(which: c_int, who: IOPRIO_WHO, ioprio: u16) -> c_int
	{
		let who = who as i32;
		let ioprio = ioprio as i32;
		unsafe { SystemCallNumber::ioprio_set.system_call_3(which as usize, who as usize, ioprio as usize) as c_int }
	}
	
	/// `dirfd` is either a `RawFd` or the special value `AT_FDCWD`.
	#[inline(always)]
	pub(crate) fn system_call_openat2(dirfd: &DirectoryFileDescriptor, pathname: &CStr, how: &open_how, size: size_t) -> isize
	{
		unsafe { SystemCallNumber::openat2.system_call_4(dirfd.as_raw_fd() as usize, pathname.as_ptr() as usize, how as *const open_how as usize, size as usize) as isize }
	}
	
	/// `start` is a pointer to memory.
	///
	/// Note:-
	///
	/// * `nodemask` can be `null()`; in which case, `maxnode` is `0`.
	///
	/// See <http://man7.org/linux/man-pages/man2/mbind.2.html>.
	#[inline(always)]
	pub(crate) fn system_call_mbind(start: NonNull<c_void>, len: usize, mode: i32, nodemask: *const usize, maxnode: usize, flags: MemoryBindFlags) -> isize
	{
		if cfg!(debug_assertions)
		{
			if nodemask.is_null()
			{
				debug_assert_eq!(maxnode, 0, "maxnode must be 0 if nodemask is null")
			}
		}
		unsafe { SystemCallNumber::mbind.system_call_6(start.as_ptr() as usize, len as usize, mode as usize, nodemask as usize, maxnode, flags.bits() as usize) as isize }
	}
	
	/// Note:-
	///
	/// * `count` is the size of the `pages` and `status` arrays,
	/// * `count` is the size of the `nodes` array if it is *not* `NULL`.
	/// * `pages` can be `null()`; in which case, `count` is `0`..
	#[inline(always)]
	pub(crate) fn system_call_move_pages(pid: pid_t, count: usize, pages: *const NonNull<c_void>, nodes: *const i32, status: NonNull<i32>, flags: MemoryBindFlags) -> isize
	{
		if cfg!(debug_assertions)
		{
			if pages.is_null()
			{
				debug_assert_eq!(count, 0, "count must be 0 if pages is null")
			}
		}
		unsafe { SystemCallNumber::move_pages.system_call_6(pid as usize, count, pages as usize, nodes as usize, status.as_ptr() as usize, flags.bits() as usize) as isize }
	}
	
	/// Note:-
	///
	/// * `frommask` can be `null()`; in which case, `maxnode` is `0`.
	/// * `tomask` can be `null()`; in which case, `maxnode` is `0`.
	#[inline(always)]
	pub(crate) fn system_call_migrate_pages(pid: i32, maxnode: usize, frommask: *const usize, tomask: *const usize) -> isize
	{
		if cfg!(debug_assertions)
		{
			if frommask.is_null()
			{
				debug_assert_eq!(maxnode, 0, "maxnode must be 0 if frommask is null")
			}
			if tomask.is_null()
			{
				debug_assert_eq!(maxnode, 0, "maxnode must be 0 if tomask is null")
			}
		}
		unsafe { SystemCallNumber::migrate_pages.system_call_4(pid as usize, maxnode, frommask as usize, tomask as usize) as isize }
	}
	
	#[inline(always)]
	pub(crate) fn system_call_perf_event_open(attr: &mut perf_event_attr, pid: pid_t, cpu: c_int, group_fd: RawFd, flags: c_ulong) -> c_int
	{
		unsafe { SystemCallNumber::perf_event_open.system_call_5(attr as *mut perf_event_attr as usize, pid as usize, cpu as usize, group_fd as usize, flags as usize) as c_int }
	}
	
	#[inline(always)]
	pub(crate) fn system_call_renameat2(oldfd: c_int, old: &CStr, newfd: c_int, new: &CStr, flags: c_int) -> c_int
	{
		unsafe { SystemCallNumber::renameat2.system_call_5(oldfd as usize, old.as_ptr() as usize, newfd as usize, new.as_ptr() as usize, flags as usize) as c_int }
	}
	
	/// On failure, the memory pointer to be `attr` may not have been initialized.
	///
	/// The `flags` argument is provided to allow for future extensions to the interface; in the current implementation it must be specified as `0`.
	#[inline(always)]
	pub(crate) fn system_call_sched_getattr(pid: pid_t, attr: NonNull<sched_attr>, size: u32, flags: c_uint) -> c_int
	{
		debug_assert_eq!(flags, 0);
		
		unsafe { SystemCallNumber::sched_getattr.system_call_4(pid as usize, attr.as_ptr() as usize, size as usize, flags as usize) as c_int }
	}
	
	/// The `flags` argument is provided to allow for future extensions to the interface; in the current implementation it must be specified as `0`.
	#[inline(always)]
	pub(crate) fn system_call_sched_setattr(pid: pid_t, attr: &sched_attr, flags: c_uint) -> c_int
	{
		debug_assert_eq!(flags, 0);
		
		unsafe { SystemCallNumber::sched_setattr.system_call_3(pid as usize, attr as *const sched_attr as usize, flags as usize) as c_int }
	}
	
	#[inline(always)]
	pub(crate) fn system_call_seccomp(op: u32, flags: u32, args: *mut c_void) -> c_int
	{
		unsafe { SystemCallNumber::seccomp.system_call_3(op as usize, flags as usize, args as usize) as c_int }
	}
	
	/// Note:-
	///
	/// * `nodemask` can be `null()`; in which case, `maxnode` is `0`.
	///
	/// See <http://man7.org/linux/man-pages/man2/set_mempolicy.2.html>.
	#[inline(always)]
	pub(crate) fn system_call_set_mempolicy(mode: i32, nodemask: *const usize, maxnode: usize) -> isize
	{
		if cfg!(debug_assertions)
		{
			if nodemask.is_null()
			{
				debug_assert_eq!(maxnode, 0, "maxnode must be 0 if nodemask is null")
			}
		}
		unsafe { SystemCallNumber::set_mempolicy.system_call_3(mode as usize, nodemask as usize, maxnode) as isize }
	}
	
	/// On failure, the memory pointer to be `buffer` may not have been initialized.
	#[inline(always)]
	pub(crate) fn system_call_statx(dirfd: &DirectoryFileDescriptor, filename: &CStr, flags: c_uint, mask: c_uint, buffer: NonNull<statx>) -> c_int
	{
		unsafe { SystemCallNumber::statx.system_call_5(dirfd.as_raw_fd() as usize, filename.as_ptr() as usize, flags as usize, mask as usize, buffer.as_ptr() as usize) as c_int }
	}
	
	/// `flags` is a combination of `O_CLOEXEC` (`UFFD_CLOEXEC`), `O_NONBLOCK` (`UFFD_NONBLOCK`) and `UFFD_USER_MODE_ONLY`.
	/// `UFFD_SHARED_FCNTL_FLAGS`, used internally, is `UFFD_CLOEXEC | UFFD_NONBLOCK`.
	#[inline(always)]
	pub(crate) fn system_call_userfaultfd(flags: c_int) -> c_int
	{
		unsafe { SystemCallNumber::userfaultfd.system_call_1(flags as usize) as c_int }
	}
}
