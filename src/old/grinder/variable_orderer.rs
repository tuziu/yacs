use std::collections::HashMap;

use crate::grinder::{config_tank::{ConfigTank, self}, grind::arc_consistency, var_des::VarDes, variable::{EnTy, VarId}};


type Ty = i32;


// pub struct  VariableOrder {
//     partials: Vec<Option<(usize,Ty)>>,
// }

// fn allocate_at<T: VarDes>(
//     ct: &ConfigTank<T>,
//     order: Vec<VarId>,
//     partials: &mut HashMap<VarId, EnTy>,
//     current: (VarId, EnTy)
// ) -> bool {
//     let next_id = current.0 + 1 ;
//     if next_id == order.len() {
//         return true;
//     }
//     for i in  ct.get_domain(next_id).get_domain(){
//         let result = arc_consistency(ct,next_id);
//         if check(ct, *i, current_id, partials) && allocate_at(ct, order,partials, current)
//         {
//             return true;
//         }
//     }
//     partials.remove(&current_id);
//     false

// }

// fn allocate_at(order: &Vec<VarId>, current: usize) -> bool {
//     let next_id = current + 1;
//     if next_id == order.len() {
//         return true;
//     }
//     for i in  ct.get_domain(next_id).get_domain(){
//         let result = arc_consistency(ct,next_id);
//         if check(ct, *i, current_id, partials) && allocate_at(ct, order,partials, current)
//         {
//             return true;
//         }
//     }
//     partials.remove(&current_id);
//     false

// }

fn reject() -> bool {
    false
}
fn accept<T: VarDes>(ct: &ConfigTank<T>) -> bool {
    false
}

// fn backtrack<T: VarDes>(ct: &ConfigTank<T>, current: (VarId,EnTy)) {
//     // if reject(){
//     //     return;
//     // }
//     if accept(ct){
//         output();
//     }
//     let s = first(c):
//     while s.is_oK() {
//         backtrack();
//         next();
//     }
// }