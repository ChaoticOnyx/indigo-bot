use app_macros::global;
use std::ops::{Deref, DerefMut};
use tera::Tera;

#[derive(Debug, Clone)]
#[global(set, lock)]
pub struct Templates(pub Tera);

impl Deref for Templates {
    type Target = Tera;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Templates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
