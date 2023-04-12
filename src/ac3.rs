use std::collections::{HashMap, VecDeque};

use crate::{
    var_des::VarDes,
    variable::{VarId, Variable},
};

fn generate(from: usize, to: usize) -> VecDeque<(usize, usize)> {
    let mut tmp = VecDeque::with_capacity(2 * (to - from));
    for i in (from + 1)..to {
        tmp.push_back((from, i));
        tmp.push_back((i, from));
    }
    tmp
}

pub struct AllocResult {}

fn restore<T: VarDes>(ct: &Vec<Variable<T>>, state: &HashMap<VarId, usize>) {
    for (id, rng) in state {
        ct[*id].restore(*rng);
    }
}

pub fn arc_consistency<T: VarDes>(ct: &Vec<Variable<T>>, pos: usize) -> bool {
    let mut state: HashMap<VarId, usize> = HashMap::new();
    let mut q = generate(pos, ct.len());
    while let Some((vk, vm)) = q.pop_front() {
        let r = ct[vk].revise(&ct[vm]);
        if r.result() {
            if ct[vk].get_domain().is_empty() {
                restore(ct, &state);
                return false;
            } else {
                state.insert(vm, r.value());
                q.append(&mut generate(vk, ct.len()));
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::{var_des::VarDes, variable::{Variable}};

    pub struct OddVar {
        i: usize,
    }

    impl OddVar{
        fn new(i: usize) -> OddVar{
            OddVar{i}
        }
    }

    impl VarDes for OddVar {
        fn is_valid(&self, other: &Self, current: Self::VarVal, checked: Self::VarVal) -> bool {
            current != checked
        }

        type VarVal = i32;
    }

    // type Type = OddVar::VarVal;

    pub fn build_odd(n: usize) -> Vec<Variable<OddVar>>{
        let d: Vec<i32> = (0..n).map(|j| j as i32).collect();
        let config: Vec<Variable<OddVar>> = (0..n)
            .map(|i| Variable::new(OddVar::new(i), d, i))
            .collect();
        config
    }

    // #[test]
    // fn queens_d() {
    //     let t = vec![vec![1,2,3],vec![4,5,6],vec![6,7,8]];
    //     let i :usize = 8;
    //     let mut b = build_odd(n);
    //     for i in 0..7 {
    //         add_variable(QueensVar::new(i))
    //             .with_domain((0..7).map(|i| i as EnTy).collect())
    //             .to(&mut b);
    //     }

    //     let mut mill = b.finalize();

    //     let t = arc_consistency(&mill, 0);
    //     pritn_variables(&mill);
    // }

    #[test]
    fn queens_d() {
        let i :usize = 8;
        let mut b = build_odd(i);


        // let mut mill = b.finalize();

        // let t = arc_consistency(&mill, 0);
        // pritn_variables(&mill);
    }
}
