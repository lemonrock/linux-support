
# Outstanding parser issues

/*
    TODO: Additional weird arpa zones
        e164.arpa ?
        uri.arpa (RFC 3405)
        urn.arpa (RFC 3405) Dynamic Delegation Discovery System (DDDS)
        See https://www.iana.org/domains/arpa?ref=hackernoon.com

    TODO: Fix canonical name chain for not just parent, but any ancestor (grand parent, great-great granparent, etc)
        dig NAPTR 4.3.2.1.6.7.9.8.6.4.e164.arpa
        6.4.e164.arpa.		90	IN	SOA	nic.gota.net. enum-registry.gotanet.se. 2020022701 10800 1800 3600000 3600
        
        dig NAPTR 3.2.1.0.5.5.5.9.9.9.1.e164.arpa. 
        e164.arpa.		90	IN	SOA	pri.authdns.ripe.net. dns.ripe.net. 1581072354 3600 600 864000 3600 

	TODO: Sort out CertificateAuthorityAuthorization into enums with values.
	TODO: Add Into<> functions a la StartOfAuthority
	
	TODO: DNS extended errors: https://tools.ietf.org/html/rfc8914
	TODO: PTR - should these be solitary?
	
	TODO: TXT records.
	
	TODO DNS Names that are underscored
	    - are they enumservices https://www.iana.org/assignments/enum-services/enum-services.xhtml#enum-services-1 ?  RFC 6117 and RFC 6118 https://tools.ietf.org/html/rfc6118
	    - https://tools.ietf.org/html/rfc8552 Scoped Interpretation of DNS Resource Records through  "Underscored" Naming of Attribute Leaves
	
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
