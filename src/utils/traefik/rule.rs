use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Host(String),
    HostRegexp(String),
    Path(String),
    PathPrefix(String),
    Method(String),
    Header(String, String),
    Query(String, String),
    ClientIP(String),
    And(Box<Rule>, Box<Rule>),
    Or(Box<Rule>, Box<Rule>),
    Not(Box<Rule>),
}

impl Rule {
    pub fn host(domain: impl Into<String>) -> Self {
        Rule::Host(domain.into())
    }

    pub fn host_regexp(pattern: impl Into<String>) -> Self {
        Rule::HostRegexp(pattern.into())
    }

    pub fn path(path: impl Into<String>) -> Self {
        Rule::Path(path.into())
    }

    pub fn path_prefix(prefix: impl Into<String>) -> Self {
        Rule::PathPrefix(prefix.into())
    }

    pub fn method(method: impl Into<String>) -> Self {
        Rule::Method(method.into())
    }

    pub fn header(key: impl Into<String>, value: impl Into<String>) -> Self {
        Rule::Header(key.into(), value.into())
    }

    pub fn query(key: impl Into<String>, value: impl Into<String>) -> Self {
        Rule::Query(key.into(), value.into())
    }

    pub fn client_ip(cidr: impl Into<String>) -> Self {
        Rule::ClientIP(cidr.into())
    }

    pub fn and(self, other: Rule) -> Self {
        Rule::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: Rule) -> Self {
        Rule::Or(Box::new(self), Box::new(other))
    }

    pub fn negate(self) -> Self {
        Rule::Not(Box::new(self))
    }

    fn needs_parens_within(&self, parent_is_and: bool) -> bool {
        match self {
            Rule::Or(_, _) => true,
            Rule::And(_, _) => !parent_is_and,
            _ => false,
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Host(v) => write!(f, "Host(`{v}`)"),
            Rule::HostRegexp(v) => write!(f, "HostRegexp(`{v}`)"),
            Rule::Path(v) => write!(f, "Path(`{v}`)"),
            Rule::PathPrefix(v) => write!(f, "PathPrefix(`{v}`)"),
            Rule::Method(v) => write!(f, "Method(`{v}`)"),
            Rule::Header(k, v) => write!(f, "Header(`{k}`, `{v}`)"),
            Rule::Query(k, v) => write!(f, "Query(`{k}`, `{v}`)"),
            Rule::ClientIP(v) => write!(f, "ClientIP(`{v}`)"),
            Rule::Not(inner) => {
                if matches!(inner.as_ref(), Rule::And(_, _) | Rule::Or(_, _)) {
                    write!(f, "!({inner})")
                } else {
                    write!(f, "!{inner}")
                }
            }
            Rule::And(l, r) => {
                write_side(f, l, true)?;
                write!(f, " && ")?;
                write_side(f, r, true)
            }
            Rule::Or(l, r) => {
                write_side(f, l, false)?;
                write!(f, " || ")?;
                write_side(f, r, false)
            }
        }
    }
}

fn write_side(f: &mut fmt::Formatter<'_>, side: &Rule, parent_is_and: bool) -> fmt::Result {
    if side.needs_parens_within(parent_is_and) {
        write!(f, "({side})")
    } else {
        write!(f, "{side}")
    }
}

impl std::ops::Not for Rule {
    type Output = Rule;
    fn not(self) -> Self::Output {
        self.negate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_host() {
        assert_eq!(Rule::host("example.com").to_string(), "Host(`example.com`)");
    }

    #[test]
    fn and_combinator() {
        let r = Rule::host("example.com").and(Rule::path_prefix("/api"));
        assert_eq!(r.to_string(), "Host(`example.com`) && PathPrefix(`/api`)");
    }

    #[test]
    fn or_nested_in_and_gets_parens() {
        let r = Rule::host("a.com")
            .or(Rule::host("b.com"))
            .and(Rule::path_prefix("/api"));
        assert_eq!(
            r.to_string(),
            "(Host(`a.com`) || Host(`b.com`)) && PathPrefix(`/api`)"
        );
    }

    #[test]
    fn and_nested_in_and_no_parens() {
        let r = Rule::host("a.com")
            .and(Rule::path_prefix("/api"))
            .and(Rule::method("GET"));
        assert_eq!(
            r.to_string(),
            "Host(`a.com`) && PathPrefix(`/api`) && Method(`GET`)"
        );
    }

    #[test]
    fn negate() {
        let r = !Rule::method("POST");
        assert_eq!(r.to_string(), "!Method(`POST`)");
    }

    #[test]
    fn negate_compound() {
        let r = (Rule::host("a.com").and(Rule::path_prefix("/x"))).negate();
        assert_eq!(r.to_string(), "!(Host(`a.com`) && PathPrefix(`/x`))");
    }
}
