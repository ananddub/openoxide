/// Which organizations' notification channels a dispatch may reach.
///
/// Notifications belong to an organization, so anything raised on behalf of a
/// tenant must name that tenant. `AllOrganizations` exists only for panel-wide
/// events that genuinely concern every operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationScope {
    Organization(i64),
    AllOrganizations,
}

impl NotificationScope {
    pub fn organization_id(&self) -> Option<i64> {
        match self {
            Self::Organization(id) => Some(*id),
            Self::AllOrganizations => None,
        }
    }

    pub fn is_tenant_scoped(&self) -> bool {
        matches!(self, Self::Organization(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tenant_scope_carries_its_id() {
        let scope = NotificationScope::Organization(7);
        assert_eq!(scope.organization_id(), Some(7));
        assert!(scope.is_tenant_scoped());
    }

    #[test]
    fn the_panel_wide_scope_names_no_organization() {
        let scope = NotificationScope::AllOrganizations;
        assert_eq!(scope.organization_id(), None);
        assert!(!scope.is_tenant_scoped());
    }
}
