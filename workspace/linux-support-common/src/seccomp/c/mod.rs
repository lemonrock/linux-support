use libc::*;


include!("__NR_SCMP_ERROR.rs");
include!("SCMP_ACT.rs");
include!("SCMP_ACT_ERRNO.rs");
include!("SCMP_ACT_TRACE.rs");
include!("scmp_arch.rs");
include!("scmp_arg_cmp.rs");
include!("scmp_compare.rs");
include!("scmp_datum_t.rs");
include!("scmp_filter_attr.rs");
include!("scmp_filter_ctx.rs");
include!("seccomp.functions.rs");
