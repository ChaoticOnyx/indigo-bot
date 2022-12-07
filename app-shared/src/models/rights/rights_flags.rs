use std::ops::BitOr;

pub trait RightsFlags
where
    Self: Sized + BitOr + Copy,
    <Self as BitOr>::Output: PartialEq<Self>,
{
    fn has_more_or_equal_rights_than(&self, another: &Self) -> bool {
        (*self | *another) == *self
    }
}
