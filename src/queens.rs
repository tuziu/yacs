// use crate::grinder::config_tank::ConfigTank;
use crate::var_des::VarDes;
use crate::variable::EnTy;

#[derive(Copy, Clone)]
pub struct QueensVar {
    x: usize,
}

impl QueensVar {
    pub fn new(x: usize) -> QueensVar {
        QueensVar { x }
    }
    pub fn get_x(&self) -> i32 {
        self.x as i32
    }
}

impl VarDes for QueensVar {
    fn is_valid(&self, other: &Self, current: EnTy, checked: EnTy) -> bool {
        ((self.get_x() - other.get_x()).abs() != (current - checked).abs()) && current != checked
    }
}

// fn is_oblique(left: (EnTy, EnTy), right: (EnTy, EnTy)) -> bool {
//     (left.0 - right.0).abs() == (left.1 - right.1).abs()
// }

// fn is_horizontal(left: (EnTy, EnTy), right: (EnTy, EnTy)) -> bool {
//     (left.0 != right.0) && (left.1 == right.1)
// }
// pub fn pritn_variables(ct: &ConfigTank<QueensVar>) {
//     for var in ct.get_variables() {
//         println!("{} ", var.get_state().get_x());
//         var.get_domain().print_vals();
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::fmt::Debug;

//     use crate::{
//         grinder::{
//             config_tank_builder::{add_variable, ConfigTankBuilder},
//             grind::arc_consistency,
//             variable::EnTy,
//         },
//         queens::{pritn_variables, QueensVar},
//     };

//     fn assert_same_Values<Ty>(left: &[Ty], right: &[Ty]) -> bool
//     where
//         Ty: Clone + PartialEq + Debug,
//     {
//         if left.len() != right.len()
//             || !left
//                 .iter()
//                 .filter(|&x| !right.contains(x))
//                 .cloned()
//                 .collect::<Vec<Ty>>()
//                 .is_empty()
//         {
//             assert_eq!(left, right);
//         }
//         true
//     }

//     #[test]
//     fn queens_d() {
//         let mut b = ConfigTankBuilder::new();
//         for i in 0..7 {
//             add_variable(QueensVar::new(i))
//                 .with_domain((0..7).map(|i| i as EnTy).collect())
//                 .to(&mut b);
//         }

//         let mut mill = b.finalize();

//         let t = arc_consistency(&mill, 0);
//         pritn_variables(&mill);
//     }
// }
