use crate::var_des::VarDes;

pub type VarId = usize;

pub struct Variable<T: VarDes> {
    // partial: Cell<Option<EnTy>>,
    state: T,
    domain: Vec<T::VarVal>,
    id: VarId,
    // domain_int: Domain,
}

impl<T: VarDes> Variable<T> {
    pub fn new(t: T, d: Vec<T::VarVal>, i: usize) -> Variable<T> {
        Variable {
            state: t,
            domain: d,
            id: i,
        }
    }

    pub fn state(&self) -> &T {
        &self.state
    }
    pub fn get_domain(&self) -> &Vec<T::VarVal> {
        &self.domain
    }

    // pub fn remove(&self, _value: EnTy) -> Option<usize> {
    //     None
    // }

    pub fn get_id(&self) -> VarId {
        self.id
    }

    pub fn set_domain_value(&mut self, val: T::VarVal) {
        self.domain = vec![val];
    }
    // pub fn revise(&self, other: &Self) -> ReviseResult {
    //     // self.domain_int.revise(&other.domain_int, |x, y| {
    //     //     self.state.is_valid(&other.state, *x)
    //     // })
    //     ReviseResult::new(1)
    // }

    pub fn restore(&self, rng: usize) {
        // self.domain_int.restore(rng);
    }
}
