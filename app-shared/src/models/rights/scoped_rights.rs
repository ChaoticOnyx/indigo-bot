use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::BitOr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RightsScope<T, R>
where
    T: Sized + Ord,
    R: Sized + Ord,
{
    Everything(R),
    Some(BTreeMap<T, R>),
    None,
}

impl<T, R> RightsScope<T, R>
where
    T: Sized + Ord,
    R: Sized + Ord + Default + Copy + BitOr<Output = R>,
{
    pub fn sum(&self) -> R {
        match &self {
            RightsScope::None => R::default(),
            RightsScope::Everything(rights) => *rights,
            RightsScope::Some(scopes) => scopes.values().fold(R::default(), |acc, x| acc.bitor(*x)),
        }
    }
}

impl<T, R> Default for RightsScope<T, R>
where
    T: Sized + Ord,
    R: Sized + Ord,
{
    fn default() -> Self {
        RightsScope::None
    }
}

impl<T, R> BitOr for RightsScope<T, R>
where
    T: Sized + Ord,
    R: Sized + Ord + BitOr<Output = R>,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        use RightsScope::{Everything, None, Some};

        match (self, rhs) {
            // Оба None
            (None, None) => None,
            // Оба Some
            (Some(scopes), Some(scopes_rhs)) => {
                Some(scopes.into_iter().chain(scopes_rhs.into_iter()).collect())
            }
            // Оба Everything
            (Everything(rights), Everything(rights_rhs)) => Everything(rights | rights_rhs),
            // Комбинация None и Some
            (None, Some(scopes)) | (Some(scopes), None) => Some(scopes),
            // Комбинация None и Everything
            (None, Everything(rights)) | (Everything(rights), None) => Everything(rights),
            // Комбинация Some и Everything
            (Some(_), Everything(rights)) | (Everything(rights), Some(_)) => Everything(rights),
        }
    }
}

impl<T, R> PartialEq for RightsScope<T, R>
where
    T: Sized + Ord,
    R: Sized + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        use RightsScope::{Everything, None, Some};

        match (self, other) {
            (None, None) => true,
            (Some(scope), Some(other_scope)) => scope == other_scope,
            (Everything(rights), Everything(other_rights)) => rights == other_rights,
            (_, _) => false,
        }
    }
}

impl<T, R> PartialOrd for RightsScope<T, R>
where
    T: Sized + Ord,
    R: Sized + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }

    fn lt(&self, other: &Self) -> bool {
        !self.gt(other)
    }

    fn le(&self, other: &Self) -> bool {
        !self.ge(other)
    }

    fn gt(&self, other: &Self) -> bool {
        use RightsScope::{Everything, None, Some};

        match (self, other) {
            (None, None) => false,
            (Everything(flags), Everything(other_flags)) => flags > other_flags,
            (Some(scope), Some(other_scope)) => {
                other_scope
                    .keys()
                    .all(|another_service_id| scope.contains_key(another_service_id))
                    && other_scope.iter().all(|(other_service_id, other_flags)| {
                        let flags = scope.get(other_service_id).unwrap();

                        flags > other_flags
                    })
            }
            (Everything(_), None) => true,
            (Some(_), None) => true,
            (_, _) => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        use RightsScope::{Everything, None, Some};

        match (self, other) {
            (None, None) => true,
            (Everything(flags), Everything(other_flags)) => flags >= other_flags,
            (Some(scope), Some(other_scope)) => {
                other_scope
                    .keys()
                    .all(|another_service_id| scope.contains_key(another_service_id))
                    && other_scope.iter().all(|(other_service_id, other_flags)| {
                        let flags = scope.get(other_service_id).unwrap();

                        flags >= other_flags
                    })
            }
            (Everything(_), None) => true,
            (Some(_), None) => true,
            (_, _) => false,
        }
    }
}
