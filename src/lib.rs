mod whois_parser;

use std::collections::HashMap;

use futures::future::join_all;
use lazy_static::lazy_static;
use regex::Regex;
use whois_parser::WhoIsInfo;
use whois_rust::{WhoIs, WhoIsError, WhoIsLookupOptions};

lazy_static! {
    static ref DOMAIN_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.[a-zA-Z]{2,}$").unwrap();
    static ref WHO_IS: WhoIs = WhoIs::from_string(include_str!("../servers.json")).unwrap();
}

pub fn is_domain(domain: &str) -> bool {
    DOMAIN_REGEX.is_match(domain)
}

pub async fn single_domain(domain: &str) -> Result<String, WhoIsError> {
    let domain_info = check_domain(domain).await?;

    if !domain_info.tld_supported {
        let (_, tld) = domain.split_once(".").unwrap();
        return Ok(format!("the .{} TLD isn't currently supported.", tld));
    }

    if domain_info.taken {
        return Ok(format!(
            "{} is registered at {} and will expire on {}",
            domain, domain_info.registrar_name, domain_info.expiration_date
        ));
    }

    Ok(format!("{} may be available!", domain))
}

async fn check_domain(domain: &str) -> Result<WhoIsInfo, WhoIsError> {
    let raw = WHO_IS
        .lookup_async(WhoIsLookupOptions::from_str(domain)?)
        .await?;

    Ok(WhoIsInfo::parse(&raw))
}

pub async fn all_domains(s: &str) -> Result<String, WhoIsError> {
    let domains = find_available_domains(s).await?;

    if domains.len() == 1 {
        return Ok(format!("{} may be available!", domains[0]));
    }

    if domains.len() > 1 {
        return Ok(format!(
            "{} and {} may be available!",
            &domains[..domains.len() - 1].join(", "),
            domains[domains.len() - 1]
        ));
    }

    Ok(format!(
        "none of the common TLDs are available for '{}'.",
        s
    ))
}

pub async fn find_available_domains(domain: &str) -> Result<Vec<String>, WhoIsError> {
    let tlds = ["com", "org", "net", "co", "io", "dev", "xyz", "tech"];
    let available_domains =
        join_all(tlds.map(|tld| domain_available(format!("{}.{}", domain, tld))))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, WhoIsError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

    Ok(available_domains)
}

async fn domain_available(domain: impl AsRef<str>) -> Result<Option<String>, WhoIsError> {
    let domain_info = check_domain(domain.as_ref()).await?;

    let domain = if !domain_info.taken {
        Some(domain.as_ref().to_owned())
    } else {
        None
    };

    Ok(domain)
}
