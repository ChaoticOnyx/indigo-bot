use crate::models::Rights;
use hex_color::HexColor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(transparent)]
pub struct RoleId(pub i64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub color: HexColor,
    pub rights: Rights,
}

impl Role {
    pub fn sum_roles_rights(roles: Vec<Role>) -> Rights {
        roles
            .into_iter()
            .map(|role| role.rights)
            .fold(Rights::none(), |acc, rights| acc | rights)
    }
}

impl Default for Role {
    fn default() -> Self {
        Self {
            id: RoleId(-1),
            name: String::from("Гость"),
            color: HexColor::from(u32::MAX),
            rights: Rights::none(),
        }
    }
}
