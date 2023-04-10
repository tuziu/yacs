pub trait VarDes<Other: ?Sized = Self> {
    type VarVal;
    fn is_valid(&self, other: &Other, current: &Self::VarVal, checked: &Self::VarVal) -> bool;
}

#[cfg(test)]
mod tests {

    #[test]
    fn wanted_api() {}
}
