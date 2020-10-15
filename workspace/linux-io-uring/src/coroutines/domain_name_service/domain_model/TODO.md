
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

## We need to actually occasionally accept the security algos for NSEC3

## We need to fix the parsing of HIP resource server names.

## We need to consider supporting compressed names in ALL resource records if we want to work with multicast-DNS.


# Handlers

* in practice, we'll implement dedicated handlers for the various resource records of interest.
* ie, A, AAAA, NS (no CNAMEs allowed), SOA, PTR (mostly useless on the modern internet), SRV, MX, ?KX, IPSECKEY, LOC, URI, CAA, TXT


# Bugs

* NS, SOA, PTR, MX, ?KX, SRV are not allowed to be aliases.
* Looking up A can give a CNAME.
* Looking up AAAA can give a CNAME.
* Looking up PTR can give a CNAME.


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
* Special handling of exceptions
    * eg `example.com`, `example.org`, `localhost` and names in the `.local` TLD.
    * See RFC 6761.
* RFC 6762 recommends the following 'internal' domain names:-
    * .intranet.
    * .internal.
    * .private.
    * .corp.
    * .home.
    * .lan.
* RFC 3484 IPv4 sort list
* RFC 6724 IPv6 sort list
* RFC 3484 / 6764 destination address selection.
    * Rules 3, 4, and 7 should be omitted for having excessive runtime and code size cost and dubious benefit.
* Special validations
    - PTR queries are invalid for link-local domains such as "254.169.in-addr.arpa." and the IPv6 link-local reverse mapping domains "8.e.f.ip6.arpa.", "9.e.f.ip6.arpa.", "a.e.f.ip6.arpa.", and "b.e.f.ip6.arpa.".
    - PTR queries which are not for *.in-addr.arpa. or *.ip6.arpa. or do not fit the format (4 labels or 32 labels). Think also about multicast addresses, etc.
    - RFC 6761:-
             10.in-addr.arpa.      21.172.in-addr.arpa.  26.172.in-addr.arpa.
             16.172.in-addr.arpa.  22.172.in-addr.arpa.  27.172.in-addr.arpa.
             17.172.in-addr.arpa.  30.172.in-addr.arpa.  28.172.in-addr.arpa.
             18.172.in-addr.arpa.  23.172.in-addr.arpa.  29.172.in-addr.arpa.
             19.172.in-addr.arpa.  24.172.in-addr.arpa.  31.172.in-addr.arpa.
             20.172.in-addr.arpa.  25.172.in-addr.arpa.  168.192.in-addr.arpa.
        test. - supposed to be passed to resolver, but supposed to eventually by NXDOMAIN
        example. example.com. example.net. example.org. - supposed to be passed to resolver, but supposed to eventually by NXDOMAIN
        localhost.
        invalid. - always returns NXDOMAIN
     - https://www.iana.org/domains/reserved
     - https://www.iana.org/assignments/special-use-domain-names/special-use-domain-names.xhtml
