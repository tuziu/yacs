use std::collections::{HashMap, VecDeque};

use crate::grinder::variable::{VarId, Variable};
use crate::grinder::{config_tank::ConfigTank, domain::Domain, var_des::VarDes, variable::EnTy};

// fn check<T: VarDes>(ct: &ConfigTank<T>, val: EnTy, pos: usize) -> bool {
fn check<T: VarDes>(
    ct: &ConfigTank<T>,
    val: EnTy,
    pos: usize,
    partials: &mut HashMap<VarId, EnTy>,
) -> bool {
    let current = ct.get_variable(pos);
    for i in 0..pos {
        // printState(mill, pos);
        let local = ct.get_variable(i);
        if let Some(v) = partials.get(&i) {
            let l = current.get_state().is_valid(local.get_state(), val, *v);
            if l {
                return false;
            }
        }
    }
    true
}

pub fn allocate<T: VarDes>(_ct: &ConfigTank<T>) -> bool {
    // allocate_at(ct, 0)
    false
}

// fn allocate_at<T: VarDes>(
//     ct: &ConfigTank<T>,
//     partials: &mut HashMap<VarId, EnTy>,
//     current: (VarId, EnTy)
// ) -> bool {
//     if current == ct.get_variables().len() {
//         return true;
//     }

//     partials.insert(current_id, *i);
//     if check(ct, *i, current_id, partials) && allocate_at(ct, partials, current)
//     {
//         return true;
//     }
//     partials.remove(&current_id);
//     false

// }

// fn allocate_at<T: VarDes>(
//         ct: &ConfigTank<T>,
//         partials: &mut HashMap<VarId, EnTy>,
//         current_id: VarId,
//         domains: &Vec<Domain>,
//     ) -> bool {
//         if current_id == ct.get_variables().len() {
//             return true;
//         }
//         domains[current_id].iter().any(|i| {
//             partials.insert(current_id, *i);
//             if check(ct, *i, current_id, partials) && allocate_at(ct, partials, current_id + 1, domains)
//             {
//                 return true;
//             }
//             partials.remove(&current_id);
//             false
//         })
//         false
//     }
fn generate(from: usize, to: usize) -> VecDeque<(usize, usize)> {
    let mut tmp = VecDeque::with_capacity(2 * (to - from));
    for i in (from + 1)..to {
        tmp.push_back((from, i));
        tmp.push_back((i, from));
    }
    tmp
}

pub struct AllocResult{

}

fn restore<T: VarDes>(ct: &ConfigTank<T> ,state: &HashMap<VarId, usize>){
    for (id , rng) in state {
        ct.get_variable(*id).restore(*rng);
    }
}

pub fn arc_consistency<T: VarDes>(ct: &ConfigTank<T>, pos: usize) -> bool {
    let mut state: HashMap<VarId, usize> = HashMap::new();
    let mut q = generate(pos, ct.get_variables().len());
    while let Some((vk, vm)) = q.pop_front() {
        let  r = ct.get_variable(vk).revise(ct.get_variable(vm)) ;
        if r.result() {
            if ct.get_domain(vk).is_empty() {
                restore(&ct,&state);
                return false;
            } else {
                state.insert(vm, r.value());
                q.append(&mut generate(vk, ct.get_variables().len()));
            }
        }
    }
    true
}

pub fn solve<T: VarDes>(cu: &ConfigTank<T>) -> Option<Vec<(VarId,EnTy)>> {

    Option::None
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, fmt::Debug};

    use crate::grinder::grind::generate;

    fn assert_same_Values<Ty>(left: &[Ty], right: &[Ty]) -> bool
    where
        Ty: Clone + PartialEq + Debug,
    {
        if left.len() != right.len()
            || !left
                .iter()
                .filter(|&x| !right.contains(x))
                .cloned()
                .collect::<Vec<Ty>>()
                .is_empty()
        {
            assert_eq!(left, right);
        }
        true
    }

    #[test]
    fn arc_cons() {
        let a = vec![2, 220, 50, 17];
    }

    #[test]
    fn arc_cons1() {
        let mut q = generate(0, 5);
        assert_eq!(
            q,
            [
                (0, 1),
                (1, 0),
                (0, 2),
                (2, 0),
                (0, 3),
                (3, 0),
                (0, 4),
                (4, 0)
            ]
        );
    }

    fn arc_cons2() {
        let mut q = generate(2, 5);
        assert_eq!(q, [(2, 3), (3, 2), (2, 4), (4, 2)]);
    }

    fn arc_cons3() {
        let mut q = generate(4, 5);
        assert_eq!(q, [(2, 3), (3, 2), (2, 4), (4, 2)]);
    }
}
