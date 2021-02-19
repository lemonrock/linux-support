// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#![allow(non_camel_case_types)]

use ::libc::c_int;
use ::libc::uint32_t;
use ::libc::uint64_t;


include!("prctl_mm_map.rs");


pub const PR_SET_PDEATHSIG: c_int = 1;
pub const PR_GET_PDEATHSIG: c_int = 2;
pub const PR_GET_DUMPABLE: c_int = 3;
pub const PR_SET_DUMPABLE: c_int = 4;
pub const PR_GET_UNALIGN: c_int = 5;
pub const PR_SET_UNALIGN: c_int = 6;
pub const PR_UNALIGN_NOPRINT: c_int = 1;
pub const PR_UNALIGN_SIGBUS: c_int = 2;
pub const PR_GET_KEEPCAPS: c_int = 7;
pub const PR_SET_KEEPCAPS: c_int = 8;
pub const PR_GET_FPEMU: c_int = 9;
pub const PR_SET_FPEMU: c_int = 10;
pub const PR_GET_FPEXC: c_int = 11;
pub const PR_SET_FPEXC: c_int = 12;
pub const PR_GET_TIMING: c_int = 13;
pub const PR_SET_TIMING: c_int = 14;
pub const PR_SET_NAME: c_int = 15;
pub const PR_GET_NAME: c_int = 16;
pub const PR_GET_ENDIAN: c_int = 19;
pub const PR_SET_ENDIAN: c_int = 20;
pub const PR_ENDIAN_BIG: c_int = 0;
pub const PR_ENDIAN_LITTLE: c_int = 1;
pub const PR_ENDIAN_PPC_LITTLE: c_int = 2;
pub const PR_GET_SECCOMP: c_int = 21;
pub const PR_SET_SECCOMP: c_int = 22;
pub const PR_CAPBSET_READ: c_int = 23;
pub const PR_CAPBSET_DROP: c_int = 24;
pub const PR_GET_TSC: c_int = 25;
pub const PR_SET_TSC: c_int = 26;
pub const PR_GET_SECUREBITS: c_int = 27;
pub const PR_SET_SECUREBITS: c_int = 28;
pub const PR_SET_TIMERSLACK: c_int = 29;
pub const PR_GET_TIMERSLACK: c_int = 30;
pub const PR_TASK_PERF_EVENTS_DISABLE: c_int = 31;
pub const PR_TASK_PERF_EVENTS_ENABLE: c_int = 32;
pub const PR_MCE_KILL: c_int = 33;
pub const PR_MCE_KILL_GET: c_int = 34;
pub const PR_SET_MM: c_int = 35;
pub const PR_SET_CHILD_SUBREAPER: c_int = 36;
pub const PR_GET_CHILD_SUBREAPER: c_int = 37;
pub const PR_SET_NO_NEW_PRIVS: c_int = 38;
pub const PR_GET_NO_NEW_PRIVS: c_int = 39;
pub const PR_GET_TID_ADDRESS: c_int = 40;
pub const PR_SET_THP_DISABLE: c_int = 41;
pub const PR_GET_THP_DISABLE: c_int = 42;
pub const PR_MPX_ENABLE_MANAGEMENT: c_int = 43;
pub const PR_MPX_DISABLE_MANAGEMENT: c_int = 44;
pub const PR_SET_FP_MODE: c_int = 45;
pub const PR_GET_FP_MODE: c_int = 46;
pub const PR_CAP_AMBIENT: c_int = 47;

pub const PR_TSC_ENABLE: c_int = 1;
pub const PR_TSC_SIGSEGV: c_int = 2;

pub const PR_TIMING_STATISTICAL: c_int = 0;
pub const PR_TIMING_TIMESTAMP: c_int = 1;

pub const PR_FPEMU_NOPRINT: c_int = 1;
pub const PR_FPEMU_SIGFPE: c_int = 2;

pub const PR_FP_EXC_SW_ENABLE: c_int = 0x80;

pub const PR_FP_EXC_DIV: c_int = 0x010000;
pub const PR_FP_EXC_OVF: c_int = 0x020000;
pub const PR_FP_EXC_UND: c_int = 0x040000;
pub const PR_FP_EXC_RES: c_int = 0x080000;
pub const PR_FP_EXC_INV: c_int = 0x100000;

pub const PR_FP_EXC_DISABLED: c_int = 0;
pub const PR_FP_EXC_NONRECOV: c_int = 1;
pub const PR_FP_EXC_ASYNC: c_int = 2;
pub const PR_FP_EXC_PRECISE: c_int = 3;

pub const PR_MCE_KILL_CLEAR: c_int = 0;
pub const PR_MCE_KILL_SET: c_int = 1;

pub const PR_MCE_KILL_LATE: c_int = 0;
pub const PR_MCE_KILL_EARLY: c_int = 1;
pub const PR_MCE_KILL_DEFAULT: c_int = 2;

pub const PR_SET_MM_START_CODE: c_int = 1;
pub const PR_SET_MM_END_CODE: c_int = 2;
pub const PR_SET_MM_START_DATA: c_int = 3;
pub const PR_SET_MM_END_DATA: c_int = 4;
pub const PR_SET_MM_START_STACK: c_int = 5;
pub const PR_SET_MM_START_BRK: c_int = 6;
pub const PR_SET_MM_BRK: c_int = 7;
pub const PR_SET_MM_ARG_START: c_int = 8;
pub const PR_SET_MM_ARG_END: c_int = 9;
pub const PR_SET_MM_ENV_START: c_int = 10;
pub const PR_SET_MM_ENV_END: c_int = 11;
pub const PR_SET_MM_AUXV: c_int = 12;
pub const PR_SET_MM_EXE_FILE: c_int = 13;
pub const PR_SET_MM_MAP: c_int = 14;
pub const PR_SET_MM_MAP_SIZE: c_int = 15;

pub const PR_SET_PTRACER: c_int = 0x59616d61;
pub const PR_SET_PTRACER_ANY: c_int = -1;

pub const PR_FP_MODE_FR: c_int = 1 << 0;
pub const PR_FP_MODE_FRE: c_int = 1 << 1;

pub const PR_CAP_AMBIENT_IS_SET: c_int = 1;
pub const PR_CAP_AMBIENT_RAISE: c_int = 2;
pub const PR_CAP_AMBIENT_LOWER: c_int = 3;
pub const PR_CAP_AMBIENT_CLEAR_ALL: c_int = 4;
