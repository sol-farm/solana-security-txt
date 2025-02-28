use anchor_lang::prelude::*;

use solana_security_txt::security_txt;

security_txt! {
    name: "Example",
    project_url: "http://example.com",
    source_code: "https://github.com/example/example",
    expiry: "2042-01-01",
    preferred_languages: "en,de",
    contacts: "email:example@example.com,discord:example#1234",
    encryption: "
-----BEGIN PGP PUBLIC KEY BLOCK-----
Comment: Alice's OpenPGP certificate
Comment: https://www.ietf.org/id/draft-bre-openpgp-samples-01.html

mDMEXEcE6RYJKwYBBAHaRw8BAQdArjWwk3FAqyiFbFBKT4TzXcVBqPTB3gmzlC/U
b7O1u120JkFsaWNlIExvdmVsYWNlIDxhbGljZUBvcGVucGdwLmV4YW1wbGU+iJAE
ExYIADgCGwMFCwkIBwIGFQoJCAsCBBYCAwECHgECF4AWIQTrhbtfozp14V6UTmPy
MVUMT0fjjgUCXaWfOgAKCRDyMVUMT0fjjukrAPoDnHBSogOmsHOsd9qGsiZpgRnO
dypvbm+QtXZqth9rvwD9HcDC0tC+PHAsO7OTh1S1TC9RiJsvawAfCPaQZoed8gK4
OARcRwTpEgorBgEEAZdVAQUBAQdAQv8GIa2rSTzgqbXCpDDYMiKRVitCsy203x3s
E9+eviIDAQgHiHgEGBYIACAWIQTrhbtfozp14V6UTmPyMVUMT0fjjgUCXEcE6QIb
DAAKCRDyMVUMT0fjjlnQAQDFHUs6TIcxrNTtEZFjUFm1M0PJ1Dng/cDW4xN80fsn
0QEA22Kr7VkCjeAEC08VSTeV+QFsmz55/lntWkwYWhmvOgE=
=iIGO
-----END PGP PUBLIC KEY BLOCK-----
",
    acknowledgements: "
The following hackers could've stolen all our money but didn't:
- Neodyme
",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md"
}

declare_id!("H6EoGSe3xgMEsnjg8ADcXb5B8GF4gJXEesQhyMFz4ZxR");

#[program]
pub mod example_contract {
    use super::*;

    pub fn hello_world(_ctx: Context<HelloWorld>) -> Result<()> {
        msg!("Hello World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct HelloWorld {}
