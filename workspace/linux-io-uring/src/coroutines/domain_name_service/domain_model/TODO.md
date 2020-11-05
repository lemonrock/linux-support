
# Outstanding parser issues

## Need to validate the data for DS-Delete when using CDS and CDNSKEY records

```
The contents of the CDS or CDNSKEY RRset MUST contain one RR and only
contain the exact fields as shown below.

CDS 0 0 0 0

CDNSKEY 0 3 0 0

The keying material payload is represented by a single 0.  This
record is signed in the same way as regular CDS/CDNSKEY RRsets are
signed.

Strictly speaking, the CDS record could be "CDS X 0 X 0" as only the
DNSKEY algorithm is what signals the DELETE operation, but for
clarity, the "0 0 0 0" notation is mandated -- this is not a
definition of DS digest algorithm 0.  The same argument applies to
"CDNSKEY 0 3 0 0"; the value 3 in the second field is mandated by
[RFC4034], Section 2.1.2."
```


/*

	TODO: Sort out CertificateAuthorityAuthorization into enums with values.
	TODO: Add Into<> functions a la StartOfAuthority
	
	TODO: DNS extended errors: https://tools.ietf.org/html/rfc8914
	TODO: PTR - should these be solitary?
	
	TODO: S-NAPTR (Straightforward-NAPTR) Parameters
	    https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml
	    Seem to have a new flag "D"
	    Seem to have both domain name and regexp: https://tools.ietf.org/html/rfc7553#section-5.2
	
	?https://www.rfc-editor.org/rfc/rfc8499.html#? TTLs in a RRSet must match?
	
	Revisit PresentSolitary, PresentMultiple and turn into:-
	
	Solitary
	
	MultipleSorted (A, AAAA, etc)
	    * RFC 6724, Section 6 Destination Address Selection for sorting IPv4 / IPv6 address lists for A and AAAA.
	    * Others needing a sort order:-
		    PublicKeyFingerprint
		    DnsBasedAuthenticationOfNamedEntities
	
	MultipleOrderedThenPrioritized (NAPTR)
	    * ?Need to preserve the sort key (Order) for future insertions?
	
	MultiplePrioritized (MX, KX, NID, L32, L64, LP, IPSECKEY [u8])
	
	MultiplePrioritizedThenWeighted (SRV, URI)

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
