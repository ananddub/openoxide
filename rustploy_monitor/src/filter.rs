/// Decides which containers this agent collects metrics for.
///
/// At high container counts most of what runs on a host is noise — sidecars,
/// build containers, other tenants. Collecting everything wastes storage and
/// buries the containers that matter, so the set is narrowed here.
#[derive(Debug, Clone, Default)]
pub struct ContainerFilter {
    include: Vec<String>,
    exclude: Vec<String>,
}

impl ContainerFilter {
    pub fn new(include: Vec<String>, exclude: Vec<String>) -> Self {
        Self { include, exclude }
    }

    /// True when this container should be collected.
    ///
    /// Exclude wins over include, and an empty include list means "everything
    /// not excluded" — so a filter that is entirely unset collects all
    /// containers, which is the sensible default for a small host.
    pub fn should_monitor(&self, name: &str) -> bool {
        if self.exclude.iter().any(|p| glob_match(p, name)) {
            return false;
        }

        if self.include.is_empty() {
            return true;
        }

        self.include.iter().any(|p| glob_match(p, name))
    }

    pub fn is_unset(&self) -> bool {
        self.include.is_empty() && self.exclude.is_empty()
    }

    pub fn describe(&self) -> String {
        if self.is_unset() {
            return "all containers".to_string();
        }
        format!(
            "include={:?} exclude={:?}",
            self.include, self.exclude
        )
    }
}

/// Matches `text` against a pattern where `*` stands for any run of characters.
///
/// Patterns are deliberately explicit rather than implicit substring matches.
/// A bare `api` matches only a container actually named `api`; to catch
/// `myproject-api-1` the pattern must say `*api*`. Substring-by-default is what
/// makes filters silently over-match — an `exclude: ["api"]` that also drops
/// `my-api-gateway` is very hard to notice.
fn glob_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();

    let mut pi = 0;
    let mut ti = 0;
    // Position of the last `*` seen, and how much of `text` it had consumed, so
    // a failed match can backtrack and let the star swallow one more character.
    let mut star: Option<usize> = None;
    let mut star_consumed = 0;

    while ti < t.len() {
        if pi < p.len() && p[pi] == t[ti] {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star = Some(pi);
            star_consumed = ti;
            pi += 1;
        } else if let Some(star_pi) = star {
            pi = star_pi + 1;
            star_consumed += 1;
            ti = star_consumed;
        } else {
            return false;
        }
    }

    // Trailing stars can match the empty remainder.
    while pi < p.len() && p[pi] == '*' {
        pi += 1;
    }

    pi == p.len()
}

/// Splits a comma-separated env var into patterns, dropping blanks.
pub fn parse_patterns(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unset_filter_accepts_everything() {
        let filter = ContainerFilter::default();
        assert!(filter.should_monitor("web"));
        assert!(filter.should_monitor("anything-at-all"));
        assert!(filter.is_unset());
    }

    #[test]
    fn include_restricts_to_matches() {
        let filter = ContainerFilter::new(vec!["web".into(), "db".into()], vec![]);
        assert!(filter.should_monitor("web"));
        assert!(filter.should_monitor("db"));
        assert!(!filter.should_monitor("redis"));
    }

    #[test]
    fn exclude_wins_over_include() {
        let filter = ContainerFilter::new(vec!["*".into()], vec!["buildkit".into()]);
        assert!(filter.should_monitor("web"));
        assert!(!filter.should_monitor("buildkit"));
    }

    #[test]
    fn a_bare_pattern_does_not_match_a_substring() {
        // The bug this avoids: excluding "api" must not also drop "my-api-gateway".
        let filter = ContainerFilter::new(vec![], vec!["api".into()]);
        assert!(!filter.should_monitor("api"));
        assert!(filter.should_monitor("my-api-gateway"));
    }

    #[test]
    fn wildcards_match_compose_and_swarm_names() {
        let filter = ContainerFilter::new(vec!["*web*".into()], vec![]);
        assert!(filter.should_monitor("myproject-web-1"));
        assert!(filter.should_monitor("web"));
        assert!(filter.should_monitor("web.1.abc123xyz"));
        assert!(!filter.should_monitor("database"));
    }

    #[test]
    fn prefix_and_suffix_patterns_work() {
        assert!(glob_match("web*", "web-1"));
        assert!(!glob_match("web*", "myweb"));
        assert!(glob_match("*-1", "web-1"));
        assert!(!glob_match("*-1", "web-2"));
    }

    #[test]
    fn a_lone_star_matches_anything_including_empty() {
        assert!(glob_match("*", "anything"));
        assert!(glob_match("*", ""));
    }

    #[test]
    fn multiple_stars_backtrack_correctly() {
        assert!(glob_match("*a*b*", "xxayybzz"));
        assert!(!glob_match("*a*b*", "xxbyyazz"));
        assert!(glob_match("a*b*c", "abc"));
    }

    #[test]
    fn exact_patterns_are_exact() {
        assert!(glob_match("web", "web"));
        assert!(!glob_match("web", "web2"));
        assert!(!glob_match("web", "aweb"));
        assert!(!glob_match("web", ""));
    }

    #[test]
    fn parses_comma_separated_patterns() {
        assert_eq!(parse_patterns("web, db ,redis"), vec!["web", "db", "redis"]);
        assert!(parse_patterns("").is_empty());
        assert!(parse_patterns("  ,  , ").is_empty());
    }
}
