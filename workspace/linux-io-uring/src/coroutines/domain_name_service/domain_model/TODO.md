
# Outstanding parser issues

/*
    
    

	TODO: Sort out CertificateAuthorityAuthorization into enums with values.
	
	TODO: DNS extended errors: https://tools.ietf.org/html/rfc8914
	TODO: PTR - should these be solitary?
	
	?https://www.rfc-editor.org/rfc/rfc8499.html#? TTLs in a RRSet must match?

    TODO: TXT strings

 */

# Other

* Encode and decode UTF-8 strings to punycode/IDNs.


# Resolvers

* Use `/etc/hostname` and `/etc/dnsdomainname`.
* Use `/etc/hosts`
* Use `/proc/sys` `LinuxKernelHostName` and `LinuxKernelDomainName`
* Use `/etc/resolv.conf`
    * <http://man7.org/linux/man-pages/man5/resolv.conf.5.html>
    * LOCALDOMAIN environment variable takes precedence over `search` entry
    * RES_OPTIONS takes precedence generally.
    * If not present, default is to use 127.0.0.1
* Use `/etc/nsswitch.conf`
    * <http://man7.org/linux/man-pages/man5/nsswitch.conf.5.html>
        * Use the `hosts` database for host name lookup
        * Could use the `services` database (via `/etc/services`) for a back-up for `SRV` / `URI` look up.
* Special handling of Wildcards
    * See <https://tools.ietf.org/html/rfc4592> for examples like `*.live.com`
