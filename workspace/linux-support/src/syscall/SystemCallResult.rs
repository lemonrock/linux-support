// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a system call result.
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SystemCallResult(usize);

impl Display for SystemCallResult
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl const From<SystemCallResult> for u64
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as u64
	}
}

impl const From<SystemCallResult> for u128
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as u128
	}
}

impl const From<SystemCallResult> for usize
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0
	}
}

impl const From<SystemCallResult> for i64
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as i64
	}
}

impl const From<SystemCallResult> for i128
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as i128
	}
}

impl const From<SystemCallResult> for isize
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as isize
	}
}

impl const From<SystemCallErrorNumber> for SystemCallResult
{
	#[inline(always)]
	fn from(system_call_error_number: SystemCallErrorNumber) -> Self
	{
		Self((-system_call_error_number) as usize)
	}
}

impl const AsUsizeIndex for SystemCallResult
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.0
	}
}

impl const From<SystemCallResult> for Result<SystemCallResultOkValue, SystemCallErrorNumber>
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.into_result()
	}
}

impl SystemCallResult
{
	/// Use this value in inclusive range clauses as `SystemCallResult::InclusiveOkRangeStartsFrom ..= SystemCallResult::InclusiveOkRangeEndsAt`.
	///
	/// `0x0000_0000_0000_0000`.
	/// Maps to ok result value `0`.
	pub const InclusiveOkRangeStartsFrom: Self = Self(SystemCallResultOkValue::InclusiveMinimum.0);
	
	/// Use this value in inclusive range clauses as `SystemCallResult::InclusiveOkRangeStartsFrom ..= SystemCallResult::InclusiveOkRangeEndsAt`.
	///
	/// `0xFFFF_FFFF_FFFF_F000`.
	/// Maps to ok result value `1,152,921,504,606,846,720`.
	/// In musl libc, this is defined as `-4096UL`; the Rust equivalent would be `4096usize.wrapping_neg()`.
	///
	/// In practice, the maximum used to maintain 32-bit compatibility is `0x0000_0000_FFFF_F000`, or `4,294,963,200`.
	pub const InclusiveOkRangeEndsAt: Self = Self(SystemCallResultOkValue::InclusiveMaximum.0);
	
	/// Use this value in inclusive range clauses as `SystemCallResult::InclusiveErrorRangeStartsFrom ..= SystemCallResult::InclusiveErrorRangeEndsAt`.
	///
	/// `0xFFFF_FFFF_FFFF_F001`.
	/// Maps to error number `4095`.
	//
	// NOTE: This really is the value `SystemCallErrorNumber::InclusiveMaximum`, as error numbers are returned using two's complement arithmetic.
	pub const InclusiveErrorRangeStartsFrom: Self = Self(-isize::from(SystemCallErrorNumber::InclusiveMaximum) as usize);
	
	/// Use this value in inclusive range clauses as `SystemCallResult::InclusiveErrorRangeStartsFrom ..= SystemCallResult::InclusiveErrorRangeEndsAt`.
	///
	/// `0xFFFF_FFFF_FFFF_FFFF`.
	/// Maps to error number `1`.
	//
	// NOTE: This really is the value `SystemCallErrorNumber::InclusiveMinimum`, as error numbers are returned using two's complement arithmetic.
	pub const InclusiveErrorRangeEndsAt: Self = Self(-isize::from(SystemCallErrorNumber::InclusiveMinimum) as usize);
	
	/// Range of `OK` values (inclusive start, exclusive end).
	/// Equivalent to `OkRangeInclusive`, `OkRangeTo` and `OkRangeToInclusive`.
	///
	/// Value is `Self::InclusiveOkRangeStartsFrom .. Self::InclusiveErrorRangeStartsFrom`.
	pub const OkRange: Range<Self> = Range
	{
		start: Self::InclusiveOkRangeStartsFrom,
		
		end: Self::InclusiveErrorRangeStartsFrom,
	};
	
	/// Range of `OK` values (inclusive start, inclusive end).
	/// Equivalent to `OkRange` and `OkRangeTo`.
	///
	/// Value is `Self::InclusiveOkRangeStartsFrom ..= Self::InclusiveOkRangeEndsAt.
	pub const OkRangeInclusive: RangeInclusive<Self> = RangeInclusive::new(Self::InclusiveOkRangeStartsFrom, Self::InclusiveOkRangeEndsAt);
	
	/// Range of `OK` values (exclusive end).
	/// Equivalent to `OkRange`, `OkRangeInclusive` and `OkRangeToInclusive`.
	///
	/// Value is ` .. Self::InclusiveErrorRangeStartsFrom`.
	pub const OkRangeTo: RangeTo<Self> = RangeTo
	{
		end: Self::InclusiveErrorRangeStartsFrom,
	};
	
	/// Range of `OK` values (exclusive end).
	/// Equivalent to `OkRange`, `OkRangeInclusive` and `OkRangeTo`.
	///
	/// Value is ` ..= Self::InclusiveOkRangeEndsAt`.
	pub const OkRangeToInclusive: RangeToInclusive<Self> = RangeToInclusive
	{
		end: Self::InclusiveOkRangeEndsAt,
	};
	
	/// Range of `Error` values (inclusive start, inclusive end).
	/// Equivalent to `ErrorRangeFrom`.
	///
	/// Value is `Self::InclusiveErrorRangeStartsFrom ..= Self::InclusiveErrorRangeEndsAt`.
	pub const ErrorRangeInclusive: RangeInclusive<Self> = RangeInclusive::new(Self::InclusiveErrorRangeStartsFrom, Self::InclusiveErrorRangeEndsAt);
	
	/// Range of `Error` values (inclusive start).
	/// Equivalent to `ErrorRangeInclusive`.
	///
	/// Value is `Self::InclusiveErrorRangeStartsFrom ..`.
	pub const ErrorRangeFrom: RangeFrom<Self> = RangeFrom
	{
		start: Self::InclusiveErrorRangeStartsFrom
	};
	
	/// Use this value in `match` inclusive range clauses as `SystemCallResult::InclusiveOkRangeStartsFrom ..= SystemCallResult::InclusiveOkRangeEndsAt`.
	///
	/// `0xFFFF_FFFF_FFFF_F000`.
	/// Maps to ok result value `1,152,921,504,606,846,720`.
	/// In musl libc, this is defined as `-4096UL`; the Rust equivalent would be `4096usize.wrapping_neg()`.
	///
	/// In practice, the maximum used to maintain 32-bit compatibility is `0x0000_0000_FFFF_F000`, or `4,294,963,200`.
	pub const InclusiveOkRangeEndsAt_usize: usize = Self::InclusiveOkRangeEndsAt.0;
	
	/// Use this value in `match` inclusive range clauses as `SystemCallResult::InclusiveErrorRangeStartsFrom ..= SystemCallResult::InclusiveErrorRangeEndsAt`.
	///
	/// `0xFFFF_FFFF_FFFF_F001`.
	/// Maps to error number `4095`.
	//
	// NOTE: This really is the value `SystemCallErrorNumber::InclusiveMaximum`, as error numbers are returned using two's complement arithmetic.
	pub const InclusiveErrorRangeStartsFrom_usize: usize = Self::InclusiveErrorRangeStartsFrom.0;
	
	/// Use this value in `match` inclusive range clauses as `SystemCallResult::InclusiveErrorRangeStartsFrom ..= SystemCallResult::InclusiveErrorRangeEndsAt`.
	///
	/// `0xFFFF_FFFF_FFFF_FFFF`.
	/// Maps to error number `1`.
	//
	// NOTE: This really is the value `SystemCallErrorNumber::InclusiveMinimum`, as error numbers are returned using two's complement arithmetic.
	pub const InclusiveErrorRangeEndsAt_usize: usize = Self::InclusiveErrorRangeEndsAt.0;
	
	/// Value is `i32::MAX`.
	pub const I32Maximum_usize: usize = Self::InclusiveMaximumRawFileDescriptor_i32 as usize;
	
	/// Value is `0`.
	pub const InclusiveMinimumRawFileDescriptor_usize: usize = Self::InclusiveOkRangeStartsFrom.0;
	
	/// Value is `i32::MAX`
	pub const InclusiveMaximumRawFileDescriptor_usize: usize = Self::I32Maximum_usize;
	
	/// Value is `0`.
	pub const InclusiveMinimumRawFileDescriptor_i32: i32 = Self::InclusiveOkRangeStartsFrom.0 as i32;
	
	/// Value is `i32::MAX`
	pub const InclusiveMaximumRawFileDescriptor_i32: i32 = i32::MAX;
	
	/// `E2BIG`.
	pub const E2BIG: Self = Self::from(SystemCallErrorNumber::E2BIG);
	
	/// `EACCES`.
	pub const EACCES: Self = Self::from(SystemCallErrorNumber::EACCES);
	
	/// `EADDRINUSE`.
	pub const EADDRINUSE: Self = Self::from(SystemCallErrorNumber::EADDRINUSE);
	
	/// `EADDRNOTAVAIL`.
	pub const EADDRNOTAVAIL: Self = Self::from(SystemCallErrorNumber::EADDRNOTAVAIL);
	
	/// `EADV`.
	pub const EADV: Self = Self::from(SystemCallErrorNumber::EADV);
	
	/// `EAFNOSUPPORT`.
	pub const EAFNOSUPPORT: Self = Self::from(SystemCallErrorNumber::EAFNOSUPPORT);
	
	/// `EAGAIN`.
	pub const EAGAIN: Self = Self::from(SystemCallErrorNumber::EAGAIN);
	
	/// `EALREADY`.
	pub const EALREADY: Self = Self::from(SystemCallErrorNumber::EALREADY);
	
	/// `EBADE`.
	pub const EBADE: Self = Self::from(SystemCallErrorNumber::EBADE);
	
	/// `EBADF`.
	pub const EBADF: Self = Self::from(SystemCallErrorNumber::EBADF);
	
	/// `EBADFD`.
	pub const EBADFD: Self = Self::from(SystemCallErrorNumber::EBADFD);
	
	/// `EBADMSG`.
	pub const EBADMSG: Self = Self::from(SystemCallErrorNumber::EBADMSG);
	
	/// `EBADR`.
	pub const EBADR: Self = Self::from(SystemCallErrorNumber::EBADR);
	
	/// `EBADRQC`.
	pub const EBADRQC: Self = Self::from(SystemCallErrorNumber::EBADRQC);
	
	/// `EBADSLT`.
	pub const EBADSLT: Self = Self::from(SystemCallErrorNumber::EBADSLT);
	
	/// `EBFONT`.
	pub const EBFONT: Self = Self::from(SystemCallErrorNumber::EBFONT);
	
	/// `EBUSY`.
	pub const EBUSY: Self = Self::from(SystemCallErrorNumber::EBUSY);
	
	/// `ECANCELED`.
	pub const ECANCELED: Self = Self::from(SystemCallErrorNumber::ECANCELED);
	
	/// `ECHILD`.
	pub const ECHILD: Self = Self::from(SystemCallErrorNumber::ECHILD);
	
	/// `ECHRNG`.
	pub const ECHRNG: Self = Self::from(SystemCallErrorNumber::ECHRNG);
	
	/// `ECOMM`.
	pub const ECOMM: Self = Self::from(SystemCallErrorNumber::ECOMM);
	
	/// `ECONNABORTED`.
	pub const ECONNABORTED: Self = Self::from(SystemCallErrorNumber::ECONNABORTED);
	
	/// `ECONNREFUSED`.
	pub const ECONNREFUSED: Self = Self::from(SystemCallErrorNumber::ECONNREFUSED);
	
	/// `ECONNRESET`.
	pub const ECONNRESET: Self = Self::from(SystemCallErrorNumber::ECONNRESET);
	
	/// `EDEADLK`.
	pub const EDEADLK: Self = Self::from(SystemCallErrorNumber::EDEADLK);
	
	/// `EDEADLOCK`.
	pub const EDEADLOCK: Self = Self::from(SystemCallErrorNumber::EDEADLOCK);
	
	/// `EDESTADDRREQ`.
	pub const EDESTADDRREQ: Self = Self::from(SystemCallErrorNumber::EDESTADDRREQ);
	
	/// `EDOM`.
	pub const EDOM: Self = Self::from(SystemCallErrorNumber::EDOM);
	
	/// `EDOTDOT`.
	pub const EDOTDOT: Self = Self::from(SystemCallErrorNumber::EDOTDOT);
	
	/// `EDQUOT`.
	pub const EDQUOT: Self = Self::from(SystemCallErrorNumber::EDQUOT);
	
	/// `EEXIST`.
	pub const EEXIST: Self = Self::from(SystemCallErrorNumber::EEXIST);
	
	/// `EFAULT`.
	pub const EFAULT: Self = Self::from(SystemCallErrorNumber::EFAULT);
	
	/// `EFBIG`.
	pub const EFBIG: Self = Self::from(SystemCallErrorNumber::EFBIG);
	
	/// `EHOSTDOWN`.
	pub const EHOSTDOWN: Self = Self::from(SystemCallErrorNumber::EHOSTDOWN);
	
	/// `EHOSTUNREACH`.
	pub const EHOSTUNREACH: Self = Self::from(SystemCallErrorNumber::EHOSTUNREACH);
	
	/// `EHWPOISON`.
	pub const EHWPOISON: Self = Self::from(SystemCallErrorNumber::EHWPOISON);
	
	/// `EIDRM`.
	pub const EIDRM: Self = Self::from(SystemCallErrorNumber::EIDRM);
	
	/// `EILSEQ`.
	pub const EILSEQ: Self = Self::from(SystemCallErrorNumber::EILSEQ);
	
	/// `EINPROGRESS`.
	pub const EINPROGRESS: Self = Self::from(SystemCallErrorNumber::EINPROGRESS);
	
	/// `EINTR`.
	pub const EINTR: Self = Self::from(SystemCallErrorNumber::EINTR);
	
	/// `EINVAL`.
	pub const EINVAL: Self = Self::from(SystemCallErrorNumber::EINVAL);
	
	/// `EIO`.
	pub const EIO: Self = Self::from(SystemCallErrorNumber::EIO);
	
	/// `EISCONN`.
	pub const EISCONN: Self = Self::from(SystemCallErrorNumber::EISCONN);
	
	/// `EISDIR`.
	pub const EISDIR: Self = Self::from(SystemCallErrorNumber::EISDIR);
	
	/// `EISNAM`.
	pub const EISNAM: Self = Self::from(SystemCallErrorNumber::EISNAM);
	
	/// `EKEYEXPIRED`.
	pub const EKEYEXPIRED: Self = Self::from(SystemCallErrorNumber::EKEYEXPIRED);
	
	/// `EKEYREJECTED`.
	pub const EKEYREJECTED: Self = Self::from(SystemCallErrorNumber::EKEYREJECTED);
	
	/// `EKEYREVOKED`.
	pub const EKEYREVOKED: Self = Self::from(SystemCallErrorNumber::EKEYREVOKED);
	
	/// `EL2HLT`.
	pub const EL2HLT: Self = Self::from(SystemCallErrorNumber::EL2HLT);
	
	/// `EL2NSYNC`.
	pub const EL2NSYNC: Self = Self::from(SystemCallErrorNumber::EL2NSYNC);
	
	/// `EL3HLT`.
	pub const EL3HLT: Self = Self::from(SystemCallErrorNumber::EL3HLT);
	
	/// `EL3RST`.
	pub const EL3RST: Self = Self::from(SystemCallErrorNumber::EL3RST);
	
	/// `ELIBACC`.
	pub const ELIBACC: Self = Self::from(SystemCallErrorNumber::ELIBACC);
	
	/// `ELIBBAD`.
	pub const ELIBBAD: Self = Self::from(SystemCallErrorNumber::ELIBBAD);
	
	/// `ELIBEXEC`.
	pub const ELIBEXEC: Self = Self::from(SystemCallErrorNumber::ELIBEXEC);
	
	/// `ELIBMAX`.
	pub const ELIBMAX: Self = Self::from(SystemCallErrorNumber::ELIBMAX);
	
	/// `ELIBSCN`.
	pub const ELIBSCN: Self = Self::from(SystemCallErrorNumber::ELIBSCN);
	
	/// `ELNRNG`.
	pub const ELNRNG: Self = Self::from(SystemCallErrorNumber::ELNRNG);
	
	/// `ELOOP`.
	pub const ELOOP: Self = Self::from(SystemCallErrorNumber::ELOOP);
	
	/// `EMEDIUMTYPE`.
	pub const EMEDIUMTYPE: Self = Self::from(SystemCallErrorNumber::EMEDIUMTYPE);
	
	/// `EMFILE`.
	pub const EMFILE: Self = Self::from(SystemCallErrorNumber::EMFILE);
	
	/// `EMLINK`.
	pub const EMLINK: Self = Self::from(SystemCallErrorNumber::EMLINK);
	
	/// `EMSGSIZE`.
	pub const EMSGSIZE: Self = Self::from(SystemCallErrorNumber::EMSGSIZE);
	
	/// `EMULTIHOP`.
	pub const EMULTIHOP: Self = Self::from(SystemCallErrorNumber::EMULTIHOP);
	
	/// `ENAMETOOLONG`.
	pub const ENAMETOOLONG: Self = Self::from(SystemCallErrorNumber::ENAMETOOLONG);
	
	/// `ENAVAIL`.
	pub const ENAVAIL: Self = Self::from(SystemCallErrorNumber::ENAVAIL);
	
	/// `ENETDOWN`.
	pub const ENETDOWN: Self = Self::from(SystemCallErrorNumber::ENETDOWN);
	
	/// `ENETRESET`.
	pub const ENETRESET: Self = Self::from(SystemCallErrorNumber::ENETRESET);
	
	/// `ENETUNREACH`.
	pub const ENETUNREACH: Self = Self::from(SystemCallErrorNumber::ENETUNREACH);
	
	/// `ENFILE`.
	pub const ENFILE: Self = Self::from(SystemCallErrorNumber::ENFILE);
	
	/// `ENOANO`.
	pub const ENOANO: Self = Self::from(SystemCallErrorNumber::ENOANO);
	
	/// `ENOBUFS`.
	pub const ENOBUFS: Self = Self::from(SystemCallErrorNumber::ENOBUFS);
	
	/// `ENOCSI`.
	pub const ENOCSI: Self = Self::from(SystemCallErrorNumber::ENOCSI);
	
	/// `ENODATA`.
	pub const ENODATA: Self = Self::from(SystemCallErrorNumber::ENODATA);
	
	/// `ENODEV`.
	pub const ENODEV: Self = Self::from(SystemCallErrorNumber::ENODEV);
	
	/// `ENOENT`.
	pub const ENOENT: Self = Self::from(SystemCallErrorNumber::ENOENT);
	
	/// `ENOEXEC`.
	pub const ENOEXEC: Self = Self::from(SystemCallErrorNumber::ENOEXEC);
	
	/// `ENOKEY`.
	pub const ENOKEY: Self = Self::from(SystemCallErrorNumber::ENOKEY);
	
	/// `ENOLCK`.
	pub const ENOLCK: Self = Self::from(SystemCallErrorNumber::ENOLCK);
	
	/// `ENOLINK`.
	pub const ENOLINK: Self = Self::from(SystemCallErrorNumber::ENOLINK);
	
	/// `ENOMEDIUM`.
	pub const ENOMEDIUM: Self = Self::from(SystemCallErrorNumber::ENOMEDIUM);
	
	/// `ENOMEM`.
	pub const ENOMEM: Self = Self::from(SystemCallErrorNumber::ENOMEM);
	
	/// `ENOMSG`.
	pub const ENOMSG: Self = Self::from(SystemCallErrorNumber::ENOMSG);
	
	/// `ENONET`.
	pub const ENONET: Self = Self::from(SystemCallErrorNumber::ENONET);
	
	/// `ENOPKG`.
	pub const ENOPKG: Self = Self::from(SystemCallErrorNumber::ENOPKG);
	
	/// `ENOPROTOOPT`.
	pub const ENOPROTOOPT: Self = Self::from(SystemCallErrorNumber::ENOPROTOOPT);
	
	/// `ENOSPC`.
	pub const ENOSPC: Self = Self::from(SystemCallErrorNumber::ENOSPC);
	
	/// `ENOSR`.
	pub const ENOSR: Self = Self::from(SystemCallErrorNumber::ENOSR);
	
	/// `ENOSTR`.
	pub const ENOSTR: Self = Self::from(SystemCallErrorNumber::ENOSTR);
	
	/// `ENOSYS`.
	pub const ENOSYS: Self = Self::from(SystemCallErrorNumber::ENOSYS);
	
	/// `ENOTBLK`.
	pub const ENOTBLK: Self = Self::from(SystemCallErrorNumber::ENOTBLK);
	
	/// `ENOTCONN`.
	pub const ENOTCONN: Self = Self::from(SystemCallErrorNumber::ENOTCONN);
	
	/// `ENOTDIR`.
	pub const ENOTDIR: Self = Self::from(SystemCallErrorNumber::ENOTDIR);
	
	/// `ENOTEMPTY`.
	pub const ENOTEMPTY: Self = Self::from(SystemCallErrorNumber::ENOTEMPTY);
	
	/// `ENOTNAM`.
	pub const ENOTNAM: Self = Self::from(SystemCallErrorNumber::ENOTNAM);
	
	/// `ENOTRECOVERABLE`.
	pub const ENOTRECOVERABLE: Self = Self::from(SystemCallErrorNumber::ENOTRECOVERABLE);
	
	/// `ENOTSOCK`.
	pub const ENOTSOCK: Self = Self::from(SystemCallErrorNumber::ENOTSOCK);
	
	/// `ENOTSUPP`.
	pub const ENOTSUPP: Self = Self::from(SystemCallErrorNumber::ENOTSUPP);
	
	/// `ENOTTY`.
	pub const ENOTTY: Self = Self::from(SystemCallErrorNumber::ENOTTY);
	
	/// `ENOTUNIQ`.
	pub const ENOTUNIQ: Self = Self::from(SystemCallErrorNumber::ENOTUNIQ);
	
	/// `ENXIO`.
	pub const ENXIO: Self = Self::from(SystemCallErrorNumber::ENXIO);
	
	/// `EOPNOTSUPP`.
	pub const EOPNOTSUPP: Self = Self::from(SystemCallErrorNumber::EOPNOTSUPP);
	
	/// `EOVERFLOW`.
	pub const EOVERFLOW: Self = Self::from(SystemCallErrorNumber::EOVERFLOW);
	
	/// `EOWNERDEAD`.
	pub const EOWNERDEAD: Self = Self::from(SystemCallErrorNumber::EOWNERDEAD);
	
	/// `EPERM`.
	pub const EPERM: Self = Self::from(SystemCallErrorNumber::EPERM);
	
	/// `EPFNOSUPPORT`.
	pub const EPFNOSUPPORT: Self = Self::from(SystemCallErrorNumber::EPFNOSUPPORT);
	
	/// `EPIPE`.
	pub const EPIPE: Self = Self::from(SystemCallErrorNumber::EPIPE);
	
	/// `EPROTO`.
	pub const EPROTO: Self = Self::from(SystemCallErrorNumber::EPROTO);
	
	/// `EPROTONOSUPPORT`.
	pub const EPROTONOSUPPORT: Self = Self::from(SystemCallErrorNumber::EPROTONOSUPPORT);
	
	/// `EPROTOTYPE`.
	pub const EPROTOTYPE: Self = Self::from(SystemCallErrorNumber::EPROTOTYPE);
	
	/// `ERANGE`.
	pub const ERANGE: Self = Self::from(SystemCallErrorNumber::ERANGE);
	
	/// `EREMCHG`.
	pub const EREMCHG: Self = Self::from(SystemCallErrorNumber::EREMCHG);
	
	/// `EREMOTE`.
	pub const EREMOTE: Self = Self::from(SystemCallErrorNumber::EREMOTE);
	
	/// `EREMOTEIO`.
	pub const EREMOTEIO: Self = Self::from(SystemCallErrorNumber::EREMOTEIO);
	
	/// `ERESTART`.
	pub const ERESTART: Self = Self::from(SystemCallErrorNumber::ERESTART);
	
	/// `ERFKILL`.
	pub const ERFKILL: Self = Self::from(SystemCallErrorNumber::ERFKILL);
	
	/// `EROFS`.
	pub const EROFS: Self = Self::from(SystemCallErrorNumber::EROFS);
	
	/// `ESHUTDOWN`.
	pub const ESHUTDOWN: Self = Self::from(SystemCallErrorNumber::ESHUTDOWN);
	
	/// `ESOCKTNOSUPPORT`.
	pub const ESOCKTNOSUPPORT: Self = Self::from(SystemCallErrorNumber::ESOCKTNOSUPPORT);
	
	/// `ESPIPE`.
	pub const ESPIPE: Self = Self::from(SystemCallErrorNumber::ESPIPE);
	
	/// `ESRCH`.
	pub const ESRCH: Self = Self::from(SystemCallErrorNumber::ESRCH);
	
	/// `ESRMNT`.
	pub const ESRMNT: Self = Self::from(SystemCallErrorNumber::ESRMNT);
	
	/// `ESTALE`.
	pub const ESTALE: Self = Self::from(SystemCallErrorNumber::ESTALE);
	
	/// `ESTRPIPE`.
	pub const ESTRPIPE: Self = Self::from(SystemCallErrorNumber::ESTRPIPE);
	
	/// `ETIME`.
	pub const ETIME: Self = Self::from(SystemCallErrorNumber::ETIME);
	
	/// `ETIMEDOUT`.
	pub const ETIMEDOUT: Self = Self::from(SystemCallErrorNumber::ETIMEDOUT);
	
	/// `ETOOMANYREFS`.
	pub const ETOOMANYREFS: Self = Self::from(SystemCallErrorNumber::ETOOMANYREFS);
	
	/// `ETXTBSY`.
	pub const ETXTBSY: Self = Self::from(SystemCallErrorNumber::ETXTBSY);
	
	/// `EUCLEAN`.
	pub const EUCLEAN: Self = Self::from(SystemCallErrorNumber::EUCLEAN);
	
	/// `EUNATCH`.
	pub const EUNATCH: Self = Self::from(SystemCallErrorNumber::EUNATCH);
	
	/// `EUSERS`.
	pub const EUSERS: Self = Self::from(SystemCallErrorNumber::EUSERS);
	
	/// `EXDEV`.
	pub const EXDEV: Self = Self::from(SystemCallErrorNumber::EXDEV);
	
	/// `EXFULL`.
	pub const EXFULL: Self = Self::from(SystemCallErrorNumber::EXFULL);
	
	/// `E2BIG` (usize).
	pub const E2BIG_usize: usize = Self::E2BIG.0;
	
	/// `EACCES` (usize).
	pub const EACCES_usize: usize = Self::EACCES.0;
	
	/// `EADDRINUSE` (usize).
	pub const EADDRINUSE_usize: usize = Self::EADDRINUSE.0;
	
	/// `EADDRNOTAVAIL` (usize).
	pub const EADDRNOTAVAIL_usize: usize = Self::EADDRNOTAVAIL.0;
	
	/// `EADV` (usize).
	pub const EADV_usize: usize = Self::EADV.0;
	
	/// `EAFNOSUPPORT` (usize).
	pub const EAFNOSUPPORT_usize: usize = Self::EAFNOSUPPORT.0;
	
	/// `EAGAIN` (usize).
	pub const EAGAIN_usize: usize = Self::EAGAIN.0;
	
	/// `EALREADY` (usize).
	pub const EALREADY_usize: usize = Self::EALREADY.0;
	
	/// `EBADE` (usize).
	pub const EBADE_usize: usize = Self::EBADE.0;
	
	/// `EBADF` (usize).
	pub const EBADF_usize: usize = Self::EBADF.0;
	
	/// `EBADFD` (usize).
	pub const EBADFD_usize: usize = Self::EBADFD.0;
	
	/// `EBADMSG` (usize).
	pub const EBADMSG_usize: usize = Self::EBADMSG.0;
	
	/// `EBADR` (usize).
	pub const EBADR_usize: usize = Self::EBADR.0;
	
	/// `EBADRQC` (usize).
	pub const EBADRQC_usize: usize = Self::EBADRQC.0;
	
	/// `EBADSLT` (usize).
	pub const EBADSLT_usize: usize = Self::EBADSLT.0;
	
	/// `EBFONT` (usize).
	pub const EBFONT_usize: usize = Self::EBFONT.0;
	
	/// `EBUSY` (usize).
	pub const EBUSY_usize: usize = Self::EBUSY.0;
	
	/// `ECANCELED` (usize).
	pub const ECANCELED_usize: usize = Self::ECANCELED.0;
	
	/// `ECHILD` (usize).
	pub const ECHILD_usize: usize = Self::ECHILD.0;
	
	/// `ECHRNG` (usize).
	pub const ECHRNG_usize: usize = Self::ECHRNG.0;
	
	/// `ECOMM` (usize).
	pub const ECOMM_usize: usize = Self::ECOMM.0;
	
	/// `ECONNABORTED` (usize).
	pub const ECONNABORTED_usize: usize = Self::ECONNABORTED.0;
	
	/// `ECONNREFUSED` (usize).
	pub const ECONNREFUSED_usize: usize = Self::ECONNREFUSED.0;
	
	/// `ECONNRESET` (usize).
	pub const ECONNRESET_usize: usize = Self::ECONNRESET.0;
	
	/// `EDEADLK` (usize).
	pub const EDEADLK_usize: usize = Self::EDEADLK.0;
	
	/// `EDEADLOCK` (usize).
	pub const EDEADLOCK_usize: usize = Self::EDEADLOCK.0;
	
	/// `EDESTADDRREQ` (usize).
	pub const EDESTADDRREQ_usize: usize = Self::EDESTADDRREQ.0;
	
	/// `EDOM` (usize).
	pub const EDOM_usize: usize = Self::EDOM.0;
	
	/// `EDOTDOT` (usize).
	pub const EDOTDOT_usize: usize = Self::EDOTDOT.0;
	
	/// `EDQUOT` (usize).
	pub const EDQUOT_usize: usize = Self::EDQUOT.0;
	
	/// `EEXIST` (usize).
	pub const EEXIST_usize: usize = Self::EEXIST.0;
	
	/// `EFAULT` (usize).
	pub const EFAULT_usize: usize = Self::EFAULT.0;
	
	/// `EFBIG` (usize).
	pub const EFBIG_usize: usize = Self::EFBIG.0;
	
	/// `EHOSTDOWN` (usize).
	pub const EHOSTDOWN_usize: usize = Self::EHOSTDOWN.0;
	
	/// `EHOSTUNREACH` (usize).
	pub const EHOSTUNREACH_usize: usize = Self::EHOSTUNREACH.0;
	
	/// `EHWPOISON` (usize).
	pub const EHWPOISON_usize: usize = Self::EHWPOISON.0;
	
	/// `EIDRM` (usize).
	pub const EIDRM_usize: usize = Self::EIDRM.0;
	
	/// `EILSEQ` (usize).
	pub const EILSEQ_usize: usize = Self::EILSEQ.0;
	
	/// `EINPROGRESS` (usize).
	pub const EINPROGRESS_usize: usize = Self::EINPROGRESS.0;
	
	/// `EINTR` (usize).
	pub const EINTR_usize: usize = Self::EINTR.0;
	
	/// `EINVAL` (usize).
	pub const EINVAL_usize: usize = Self::EINVAL.0;
	
	/// `EIO` (usize).
	pub const EIO_usize: usize = Self::EIO.0;
	
	/// `EISCONN` (usize).
	pub const EISCONN_usize: usize = Self::EISCONN.0;
	
	/// `EISDIR` (usize).
	pub const EISDIR_usize: usize = Self::EISDIR.0;
	
	/// `EISNAM` (usize).
	pub const EISNAM_usize: usize = Self::EISNAM.0;
	
	/// `EKEYEXPIRED` (usize).
	pub const EKEYEXPIRED_usize: usize = Self::EKEYEXPIRED.0;
	
	/// `EKEYREJECTED` (usize).
	pub const EKEYREJECTED_usize: usize = Self::EKEYREJECTED.0;
	
	/// `EKEYREVOKED` (usize).
	pub const EKEYREVOKED_usize: usize = Self::EKEYREVOKED.0;
	
	/// `EL2HLT` (usize).
	pub const EL2HLT_usize: usize = Self::EL2HLT.0;
	
	/// `EL2NSYNC` (usize).
	pub const EL2NSYNC_usize: usize = Self::EL2NSYNC.0;
	
	/// `EL3HLT` (usize).
	pub const EL3HLT_usize: usize = Self::EL3HLT.0;
	
	/// `EL3RST` (usize).
	pub const EL3RST_usize: usize = Self::EL3RST.0;
	
	/// `ELIBACC` (usize).
	pub const ELIBACC_usize: usize = Self::ELIBACC.0;
	
	/// `ELIBBAD` (usize).
	pub const ELIBBAD_usize: usize = Self::ELIBBAD.0;
	
	/// `ELIBEXEC` (usize).
	pub const ELIBEXEC_usize: usize = Self::ELIBEXEC.0;
	
	/// `ELIBMAX` (usize).
	pub const ELIBMAX_usize: usize = Self::ELIBMAX.0;
	
	/// `ELIBSCN` (usize).
	pub const ELIBSCN_usize: usize = Self::ELIBSCN.0;
	
	/// `ELNRNG` (usize).
	pub const ELNRNG_usize: usize = Self::ELNRNG.0;
	
	/// `ELOOP` (usize).
	pub const ELOOP_usize: usize = Self::ELOOP.0;
	
	/// `EMEDIUMTYPE` (usize).
	pub const EMEDIUMTYPE_usize: usize = Self::EMEDIUMTYPE.0;
	
	/// `EMFILE` (usize).
	pub const EMFILE_usize: usize = Self::EMFILE.0;
	
	/// `EMLINK` (usize).
	pub const EMLINK_usize: usize = Self::EMLINK.0;
	
	/// `EMSGSIZE` (usize).
	pub const EMSGSIZE_usize: usize = Self::EMSGSIZE.0;
	
	/// `EMULTIHOP` (usize).
	pub const EMULTIHOP_usize: usize = Self::EMULTIHOP.0;
	
	/// `ENAMETOOLONG` (usize).
	pub const ENAMETOOLONG_usize: usize = Self::ENAMETOOLONG.0;
	
	/// `ENAVAIL` (usize).
	pub const ENAVAIL_usize: usize = Self::ENAVAIL.0;
	
	/// `ENETDOWN` (usize).
	pub const ENETDOWN_usize: usize = Self::ENETDOWN.0;
	
	/// `ENETRESET` (usize).
	pub const ENETRESET_usize: usize = Self::ENETRESET.0;
	
	/// `ENETUNREACH` (usize).
	pub const ENETUNREACH_usize: usize = Self::ENETUNREACH.0;
	
	/// `ENFILE` (usize).
	pub const ENFILE_usize: usize = Self::ENFILE.0;
	
	/// `ENOANO` (usize).
	pub const ENOANO_usize: usize = Self::ENOANO.0;
	
	/// `ENOBUFS` (usize).
	pub const ENOBUFS_usize: usize = Self::ENOBUFS.0;
	
	/// `ENOCSI` (usize).
	pub const ENOCSI_usize: usize = Self::ENOCSI.0;
	
	/// `ENODATA` (usize).
	pub const ENODATA_usize: usize = Self::ENODATA.0;
	
	/// `ENODEV` (usize).
	pub const ENODEV_usize: usize = Self::ENODEV.0;
	
	/// `ENOENT` (usize).
	pub const ENOENT_usize: usize = Self::ENOENT.0;
	
	/// `ENOEXEC` (usize).
	pub const ENOEXEC_usize: usize = Self::ENOEXEC.0;
	
	/// `ENOKEY` (usize).
	pub const ENOKEY_usize: usize = Self::ENOKEY.0;
	
	/// `ENOLCK` (usize).
	pub const ENOLCK_usize: usize = Self::ENOLCK.0;
	
	/// `ENOLINK` (usize).
	pub const ENOLINK_usize: usize = Self::ENOLINK.0;
	
	/// `ENOMEDIUM` (usize).
	pub const ENOMEDIUM_usize: usize = Self::ENOMEDIUM.0;
	
	/// `ENOMEM` (usize).
	pub const ENOMEM_usize: usize = Self::ENOMEM.0;
	
	/// `ENOMSG` (usize).
	pub const ENOMSG_usize: usize = Self::ENOMSG.0;
	
	/// `ENONET` (usize).
	pub const ENONET_usize: usize = Self::ENONET.0;
	
	/// `ENOPKG` (usize).
	pub const ENOPKG_usize: usize = Self::ENOPKG.0;
	
	/// `ENOPROTOOPT` (usize).
	pub const ENOPROTOOPT_usize: usize = Self::ENOPROTOOPT.0;
	
	/// `ENOSPC` (usize).
	pub const ENOSPC_usize: usize = Self::ENOSPC.0;
	
	/// `ENOSR` (usize).
	pub const ENOSR_usize: usize = Self::ENOSR.0;
	
	/// `ENOSTR` (usize).
	pub const ENOSTR_usize: usize = Self::ENOSTR.0;
	
	/// `ENOSYS` (usize).
	pub const ENOSYS_usize: usize = Self::ENOSYS.0;
	
	/// `ENOTBLK` (usize).
	pub const ENOTBLK_usize: usize = Self::ENOTBLK.0;
	
	/// `ENOTCONN` (usize).
	pub const ENOTCONN_usize: usize = Self::ENOTCONN.0;
	
	/// `ENOTDIR` (usize).
	pub const ENOTDIR_usize: usize = Self::ENOTDIR.0;
	
	/// `ENOTEMPTY` (usize).
	pub const ENOTEMPTY_usize: usize = Self::ENOTEMPTY.0;
	
	/// `ENOTNAM` (usize).
	pub const ENOTNAM_usize: usize = Self::ENOTNAM.0;
	
	/// `ENOTRECOVERABLE` (usize).
	pub const ENOTRECOVERABLE_usize: usize = Self::ENOTRECOVERABLE.0;
	
	/// `ENOTSOCK` (usize).
	pub const ENOTSOCK_usize: usize = Self::ENOTSOCK.0;
	
	/// `ENOTSUPP` (usize).
	pub const ENOTSUPP_usize: usize = Self::ENOTSUPP.0;
	
	/// `ENOTTY` (usize).
	pub const ENOTTY_usize: usize = Self::ENOTTY.0;
	
	/// `ENOTUNIQ` (usize).
	pub const ENOTUNIQ_usize: usize = Self::ENOTUNIQ.0;
	
	/// `ENXIO` (usize).
	pub const ENXIO_usize: usize = Self::ENXIO.0;
	
	/// `EOPNOTSUPP` (usize).
	pub const EOPNOTSUPP_usize: usize = Self::EOPNOTSUPP.0;
	
	/// `EOVERFLOW` (usize).
	pub const EOVERFLOW_usize: usize = Self::EOVERFLOW.0;
	
	/// `EOWNERDEAD` (usize).
	pub const EOWNERDEAD_usize: usize = Self::EOWNERDEAD.0;
	
	/// `EPERM` (usize).
	pub const EPERM_usize: usize = Self::EPERM.0;
	
	/// `EPFNOSUPPORT` (usize).
	pub const EPFNOSUPPORT_usize: usize = Self::EPFNOSUPPORT.0;
	
	/// `EPIPE` (usize).
	pub const EPIPE_usize: usize = Self::EPIPE.0;
	
	/// `EPROTO` (usize).
	pub const EPROTO_usize: usize = Self::EPROTO.0;
	
	/// `EPROTONOSUPPORT` (usize).
	pub const EPROTONOSUPPORT_usize: usize = Self::EPROTONOSUPPORT.0;
	
	/// `EPROTOTYPE` (usize).
	pub const EPROTOTYPE_usize: usize = Self::EPROTOTYPE.0;
	
	/// `ERANGE` (usize).
	pub const ERANGE_usize: usize = Self::ERANGE.0;
	
	/// `EREMCHG` (usize).
	pub const EREMCHG_usize: usize = Self::EREMCHG.0;
	
	/// `EREMOTE` (usize).
	pub const EREMOTE_usize: usize = Self::EREMOTE.0;
	
	/// `EREMOTEIO` (usize).
	pub const EREMOTEIO_usize: usize = Self::EREMOTEIO.0;
	
	/// `ERESTART` (usize).
	pub const ERESTART_usize: usize = Self::ERESTART.0;
	
	/// `ERFKILL` (usize).
	pub const ERFKILL_usize: usize = Self::ERFKILL.0;
	
	/// `EROFS` (usize).
	pub const EROFS_usize: usize = Self::EROFS.0;
	
	/// `ESHUTDOWN` (usize).
	pub const ESHUTDOWN_usize: usize = Self::ESHUTDOWN.0;
	
	/// `ESOCKTNOSUPPORT` (usize).
	pub const ESOCKTNOSUPPORT_usize: usize = Self::ESOCKTNOSUPPORT.0;
	
	/// `ESPIPE` (usize).
	pub const ESPIPE_usize: usize = Self::ESPIPE.0;
	
	/// `ESRCH` (usize).
	pub const ESRCH_usize: usize = Self::ESRCH.0;
	
	/// `ESRMNT` (usize).
	pub const ESRMNT_usize: usize = Self::ESRMNT.0;
	
	/// `ESTALE` (usize).
	pub const ESTALE_usize: usize = Self::ESTALE.0;
	
	/// `ESTRPIPE` (usize).
	pub const ESTRPIPE_usize: usize = Self::ESTRPIPE.0;
	
	/// `ETIME` (usize).
	pub const ETIME_usize: usize = Self::ETIME.0;
	
	/// `ETIMEDOUT` (usize).
	pub const ETIMEDOUT_usize: usize = Self::ETIMEDOUT.0;
	
	/// `ETOOMANYREFS` (usize).
	pub const ETOOMANYREFS_usize: usize = Self::ETOOMANYREFS.0;
	
	/// `ETXTBSY` (usize).
	pub const ETXTBSY_usize: usize = Self::ETXTBSY.0;
	
	/// `EUCLEAN` (usize).
	pub const EUCLEAN_usize: usize = Self::EUCLEAN.0;
	
	/// `EUNATCH` (usize).
	pub const EUNATCH_usize: usize = Self::EUNATCH.0;
	
	/// `EUSERS` (usize).
	pub const EUSERS_usize: usize = Self::EUSERS.0;
	
	/// `EXDEV` (usize).
	pub const EXDEV_usize: usize = Self::EXDEV.0;
	
	/// `EXFULL` (usize).
	pub const EXFULL_usize: usize = Self::EXFULL.0;
	
	/// Is this OK?
	#[inline(always)]
	pub const fn is_ok(self) -> bool
	{
		self.0 < Self::InclusiveErrorRangeStartsFrom.0
	}
	
	/// Is this an error?
	#[inline(always)]
	pub const fn is_error(self) -> bool
	{
		// There is no need to check for the minimum (`-1`) as it is `0xFFFF_FFFF_FFFF_FFFF`.
		self.0 >= Self::InclusiveErrorRangeStartsFrom.0
	}
	
	/// Sets the `errno` "global static" (usually a value in the `pthread` struct pointed to be `pthread_self()`).
	///
	/// Result of the function is one of:-
	///
	/// * `-1` error number is set.
	/// * `0 ..= isize::MAX`: Valid result.
	/// * `-4096 ..= isize::MIN`: Valid result.
	///
	/// The result of the function can never be `-2 to -4095`.
	///
	/// This horrible approach is because Linux stuffs an error code into an usize register, unlike, say, the BSDs, which return a value in one register and a flag in another indicating if the value is an error or not.
	#[inline(always)]
	pub fn set_errno_to_be_compatible_with_libc(self) -> isize
	{
		if unlikely!(self.is_error())
		{
			self.system_call_error_number().set_errno();
			-1
		}
		else
		{
			self.0 as isize
		}
	}
	
	/// Horrible code to cater for 'unwrapped' SystemCallResult used in `match` statements as its inner `usize` value.
	#[inline(always)]
	pub fn usize_to_io_error(error: usize) -> io::Error
	{
		Self::usize_to_system_call_error_number(error).into()
	}
	
	/// Horrible code to cater for 'unwrapped' SystemCallResult used in `match` statements as its inner `usize` value.
	#[inline(always)]
	pub fn usize_to_system_call_error_number(error: usize) -> SystemCallErrorNumber
	{
		let result = Self(error);
		debug_assert!(result.is_error());
		result.system_call_error_number()
	}
	
	#[inline(always)]
	const fn into_result(self) -> Result<SystemCallResultOkValue, SystemCallErrorNumber>
	{
		if unlikely!(self.is_error())
		{
			Err(self.system_call_error_number())
		}
		else
		{
			Ok(SystemCallResultOkValue(self.0))
		}
	}
	
	#[inline(always)]
	const fn system_call_error_number(self) -> SystemCallErrorNumber
	{
		SystemCallErrorNumber::from_valid_u16((-(self.0 as isize)) as u16)
	}
}
