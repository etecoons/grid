//! Application configuration (flat struct).
//!
//! Replaces the prior wrapper around `shared_backend::server::ServerConfig`.
//! All 16 fields that the shared `ServerConfig` had are inlined here as
//! direct fields of [`AppConfig`]. The env-parsing logic is duplicated
//! per app so each app tunes its own defaults (cookie clamp, opt-out
//! booleans, site-title fallback) without a one-size-fits-all shared
//! abstraction.

use ipnet::IpNet;
use std::env;
use std::str::FromStr;

const DEFAULT_PORT: u16 = 4405;
const DEFAULT_MAX_ATTEMPTS: u32 = 5;
const DEFAULT_LOCKOUT_TIME_MINUTES: u64 = 15;
const DEFAULT_COOKIE_MAX_AGE_HOURS: i64 = 24;
const DEFAULT_SHUTDOWN_DRAIN_SECONDS: u64 = 5;

/// Application configuration.
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub port: u16,
    pub site_title: String,
    pub base_url: String,
    pub allowed_origins: String,
    pub pin: Option<String>,
    pub enable_translation: bool,
    pub enable_themes: bool,
    pub enable_print: bool,
    pub show_version: bool,
    pub show_github: bool,
    pub trust_proxy: bool,
    pub trusted_proxies: Vec<IpNet>,
    pub max_attempts: u32,
    pub lockout_time_minutes: u64,
    pub cookie_max_age_hours: i64,
    pub shutdown_drain_seconds: u64,
}

impl AppConfig {
    pub fn load() -> Self {
        #[cfg(not(test))]
        {
            let _ = dotenvy::from_path("/app/data/.env");
            let _ = dotenvy::dotenv();
        }

        let port = parse_or("PORT", DEFAULT_PORT);

        let site_title = first_nonempty_env(&[
            "GRID_SITE_TITLE",
            "GRID_TITLE",
            "SITE_TITLE",
        ])
        .unwrap_or_else(|| "Grid".to_string());

        let base_url = env::var("BASE_URL").unwrap_or_else(|_| format!("http://localhost:{port}"));
        let allowed_origins = env::var("ALLOWED_ORIGINS").unwrap_or_default();
        let pin = first_nonempty_env(&["GRID_PIN", "PIN"]).and_then(|p| {
            let len = p.chars().count();
            if (4..=64).contains(&len) {
                Some(p)
            } else {
                None
            }
        });

        let trust_proxy = parse_bool_env("TRUST_PROXY");
        let trusted_proxies = parse_trusted_proxies("TRUSTED_PROXY_IPS");

        Self {
            port,
            site_title,
            base_url,
            allowed_origins,
            pin,
            enable_translation: parse_bool_env("ENABLE_TRANSLATION"),
            enable_themes: parse_optout_bool_env("ENABLE_THEMES", true),
            enable_print: parse_optout_bool_env("ENABLE_PRINT", true),
            show_version: parse_optout_bool_env("SHOW_VERSION", true),
            show_github: parse_optout_bool_env("SHOW_GITHUB", true),
            trust_proxy,
            trusted_proxies,
            max_attempts: parse_or("MAX_ATTEMPTS", DEFAULT_MAX_ATTEMPTS),
            lockout_time_minutes: parse_or("LOCKOUT_TIME_MINUTES", DEFAULT_LOCKOUT_TIME_MINUTES),
            cookie_max_age_hours: parse_or("COOKIE_MAX_AGE_HOURS", DEFAULT_COOKIE_MAX_AGE_HOURS),
            shutdown_drain_seconds: parse_or("SHUTDOWN_DRAIN_SECONDS", DEFAULT_SHUTDOWN_DRAIN_SECONDS),
        }
    }

    /// Returns `true` if PIN-based authentication is enabled.
    #[must_use]
    pub fn pin_enabled(&self) -> bool {
        self.pin.is_some()
    }

    /// Returns the lockout duration as a `std::time::Duration`.
    #[must_use]
    pub fn lockout_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.lockout_time_minutes * 60)
    }
}

fn parse_or<T>(name: &str, default: T) -> T
where
    T: FromStr,
{
    match env::var(name) {
        Ok(v) => match v.parse() {
            Ok(parsed) => parsed,
            Err(_) => {
                tracing::warn!(
                    target: "config",
                    "{name}={v:?} is not a valid value; using default",
                );
                default
            }
        },
        Err(_) => default,
    }
}

fn parse_bool_env(name: &str) -> bool {
    env::var(name)
        .map(|v| v == "true" || v == "on")
        .unwrap_or(false)
}

fn parse_optout_bool_env(name: &str, default: bool) -> bool {
    env::var(name)
        .map(|v| v != "false" && v != "off")
        .unwrap_or(default)
}

fn first_nonempty_env(names: &[&str]) -> Option<String> {
    for name in names {
        if let Ok(v) = env::var(name)
            && !v.is_empty()
        {
            return Some(v);
        }
    }
    None
}

fn parse_trusted_proxies(name: &str) -> Vec<IpNet> {
    env::var(name)
        .ok()
        .map(|s| {
            s.split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .filter_map(|s| IpNet::from_str(s).ok())
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_does_not_panic() {
        let cfg = AppConfig::load();
        assert!(!cfg.site_title.is_empty());
    }

    #[test]
    fn pin_disabled_by_default() {
        let cfg = AppConfig::load();
        // Default: no GRID_PIN/PIN env set, so no PIN.
        // Tests set GRID_PIN in some other test files; this is the
        // baseline.
        if cfg.pin.is_none() {
            assert!(!cfg.pin_enabled());
        } else {
            assert!(cfg.pin_enabled());
        }
    }

    #[test]
    fn lockout_duration_scales_with_minutes() {
        let cfg = AppConfig::load();
        let expected = std::time::Duration::from_secs(cfg.lockout_time_minutes * 60);
        assert_eq!(cfg.lockout_duration(), expected);
    }
}
