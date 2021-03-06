use crate::grinder::{domain::Domain, revise_result::ReviseResult, var_des::VarDes};

pub type EnTy = i32;
pub type VarId = usize;

pub struct Variable<T> {
    // partial: Cell<Option<EnTy>>,
    state: T,
    domain: Domain,
    id: VarId,
}



impl<T: VarDes> Variable<T> {
    pub fn new(t: T, d: Vec<EnTy>, i: usize) -> Variable<T> {
        Variable {
            state: t,
            id: i,
            domain: Domain::new(d),
        }
    }

    pub fn get_state(&self) -> &T {
        &self.state
    }
    pub fn get_domain(&self) -> &Domain {
        &self.domain
    }

    // pub fn set_partial(&self, partial: EnTy) {
    //     self.partial.set(Some(partial));
    // }

    // pub fn reset_partial(&self) {
    //     self.partial.set(None);
    // }

    // pub fn get_partial(&self) -> Option<EnTy> {
    //     self.partial.get()
    // }

    // pub fn remove(&self, _value: EnTy) -> Option<usize> {
    //     None
    // }

    pub fn get_id(&self) -> VarId {
        self.id
    }

    // pub fn set_domain_value(&mut self, val: EnTy){
    //     self.domain = vec![val];
    // }
    pub fn revise(&self, other: &Self) -> ReviseResult {
        self.domain.revise(other.get_domain(), |x, y| {
            self.state.is_valid(&other.state, *x, *y)
        })
    }

    pub fn restore(&self, rng: usize) {
        self.domain.restore(rng);
    }
}
