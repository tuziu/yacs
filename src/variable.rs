use crate::var_des::VarDes;
// use std::cell::Cell;

pub type EnTy = i32;
pub type VarId = usize;

pub struct Variable<T> {
    // partial: Cell<Option<EnTy>>,
    state: T,
    domain: Vec<EnTy>,
    id: VarId,
}



impl<T: VarDes> Variable<T> {
    pub fn new(t: T, d: Vec<EnTy>, i: usize) -> Variable<T> {
        Variable {
            state: t,
            id: i,
            domain: d,
            // partial: Cell::new(Option::None),
        }
    }

    pub fn state(&self) -> &T {
        &self.state
    }
    pub fn get_domain(&self) -> &Vec<EnTy> {
        &self.domain
    }

    // pub fn remove(&self, _value: EnTy) -> Option<usize> {
    //     None
    // }

    pub fn get_id(&self) -> VarId {
        self.id
    }

    pub fn set_domain_value(&mut self, val: EnTy){
        self.domain = vec![val];
    }
    // pub fn revise(&self, other: &Self) -> ReviseResult {
    //     self.domain.revise(other.get_domain(), |x, y| {
    //         self.state.is_valid(&other.state, *x, *y)
    //     })
    // }

    // pub fn restore(&self, rng: usize) {
    //     self.domain.restore(rng);
    // }
}