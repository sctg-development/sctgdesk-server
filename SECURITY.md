# Security policy

Please **do not open a public issue** for a vulnerability.

Report it privately with GitHub's "Report a vulnerability" button on the Security tab of this repository
(private vulnerability reporting). Include the version, what you did, what happened, and, if you can, a minimal
reproduction.

We will acknowledge the report, work on a fix, and credit you if you wish. Vulnerabilities that also affect upstream
RustDesk should be reported to upstream as well.

Supported versions: the latest release and the default branch. This project has not been through an independent
security review; the TCP key exchange in particular is our own implementation of a protocol designed by upstream.
