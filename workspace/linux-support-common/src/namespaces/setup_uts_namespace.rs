/// UTS stands for Unix Time-Sharing.
#[inline(always)]
pub fn setup_uts_namespace()
{
	// NOTE: Unusually, this C function does not require a string with a terminating NUL byte.
	static FixedHostName: &'static [u8] = b"stormmq";
	unsafe { sethostname(FixedHostName.as_ptr() as *const c_char, FixedHostName.len()) };

	// NOTE: Unusually, this C function does not require a string with a terminating NUL byte.
	static FixedNetworkInformationServiceDomainName: &'static [u8] = b"com";
	unsafe { setdomainname(FixedNetworkInformationServiceDomainName.as_ptr() as *const c_char, FixedNetworkInformationServiceDomainName.len()) };
}
