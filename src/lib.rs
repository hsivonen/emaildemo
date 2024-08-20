/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

use icu_normalizer::ComposingNormalizer;
use idna::uts46::verify_dns_length;
use idna::uts46::AsciiDenyList;
use idna::uts46::ErrorPolicy;
use idna::uts46::Hyphens;
use idna::uts46::ProcessingError;
use idna::uts46::ProcessingSuccess;
use idna::uts46::Uts46;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn email(input: &str) -> Option<String> {
    let (local, domain) = input.rsplit_once('@')?;

    if local.is_empty() {
        return None;
    }

    for b in local.as_bytes() {
        match b {
            0..=0x20
            | 0x7F
            | b'"'
            | b'('
            | b')'
            | b','
            | b':'
            | b';'
            | b'<'
            | b'>'
            | b'@'
            | b'['
            | b'\\'
            | b']' => {
                return None;
            }
            _ => {}
        }
    }

    let mut ascii_domain = String::new();
    let mut unicode_domain = String::new();

    match Uts46::new().process(
        domain.as_bytes(),
        AsciiDenyList::STD3,
        Hyphens::Check,
        ErrorPolicy::FailFast,
        |_, _, _| true,
        &mut unicode_domain,
        Some(&mut ascii_domain),
    ) {
        Ok(ProcessingSuccess::Passthrough) => {
            ascii_domain.push_str(domain);
            unicode_domain.push_str(domain);
        }
        Ok(ProcessingSuccess::WroteToSink) => {
            if ascii_domain.is_empty() {
                ascii_domain.push_str(&unicode_domain);
            }
        }
        Err(ProcessingError::ValidityError) => {
            return None;
        }
        Err(ProcessingError::SinkError) => unreachable!(),
    }
    if !verify_dns_length(&ascii_domain, false) {
        return None;
    }

    assert_ne!(ascii_domain.as_bytes().last(), Some(&b'.'));

    let tld = if let Some((_, tld)) = ascii_domain.rsplit_once('.') {
        tld
    } else {
        // XXX do we want to support TLDs with MX records or require
        // at least one more level in the domain name?
        domain
    };

    if tld.as_bytes().iter().all(|&b| b.is_ascii_digit()) {
        return None;
    }

    let normalized_local = ComposingNormalizer::new_nfc().normalize(local);

    let mut output = String::with_capacity(
        normalized_local.len()
            + 1
            + unicode_domain.len()
            + 1
            + normalized_local.len()
            + 1
            + ascii_domain.len(),
    );
    output.push_str(&normalized_local);
    output.push('@');
    output.push_str(&unicode_domain);
    output.push('\u{0}');
    output.push_str(&normalized_local);
    output.push('@');
    output.push_str(&ascii_domain);

    Some(output)
}
