use crate::models::{ScopedServiceRights, TokenRights, UserRights};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::BitOr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rights {
    pub user: UserRights,
    pub token: TokenRights,
    pub service: ScopedServiceRights,
}

impl Rights {
    pub fn full() -> Self {
        Self {
            user: UserRights::all(),
            token: TokenRights::all(),
            service: ScopedServiceRights::all(),
        }
    }

    pub fn none() -> Self {
        Self {
            user: UserRights::empty(),
            token: TokenRights::empty(),
            service: ScopedServiceRights::empty(),
        }
    }

    pub fn bits(&self) -> u64 {
        self.user.bits() + self.token.bits() + self.service.sum().bits()
    }
}

impl BitOr for Rights {
    type Output = Rights;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            user: self.user | rhs.user,
            service: self.service | rhs.service,
            token: self.token | rhs.token,
        }
    }
}

impl PartialEq for Rights {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token && self.user == other.user && self.service == other.service
    }
}

impl PartialOrd for Rights {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self > other {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }

    fn lt(&self, other: &Self) -> bool {
        !self.gt(other)
    }

    fn le(&self, other: &Self) -> bool {
        !self.ge(other)
    }

    fn gt(&self, other: &Self) -> bool {
        self.token > other.token && self.service > other.service && self.user > other.user
    }

    fn ge(&self, other: &Self) -> bool {
        self.token >= other.token && self.service >= other.service && self.user >= other.user
    }
}
