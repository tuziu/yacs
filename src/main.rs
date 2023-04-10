// pub mod backtrack;

// // use std::env::current_dir;
// use mylib::{queens::*, sudoku::*, var_des::VarDes, variable::Variable};

// type VarId = usize;
// type EnTy = i32;

// struct Context {
//     var_id: VarId,
//     domain_ele_id: usize,
//     // domain_to_check: Vec<EnTy>,
//     partial: Option<EnTy>,
// }

// fn print_context(context: &[Context]) {
//     let d: Vec<String> = context
//         .iter()
//         .map(|c| {
//             format!(
//                 "vi: {} di: {} p: {:?}",
//                 c.var_id, c.domain_ele_id, c.partial
//             )
//         })
//         .collect();
//     println!(" {} ", d.concat());
// }

// fn reject<T: VarDes>(config: &[Variable<T>], context: &[Context]) -> bool {
//     context.last().map_or(false, |t| {
//         let var_id = t.var_id;
//         for i in 0..var_id {
//             if !config[var_id].state().is_valid(
//                 config[i].state(),
//                 context[var_id].partial.unwrap(),

//             ) {
//                 return true;
//             }
//         }
//         false
//     })
// }

// fn accept<T: VarDes>(config: &Vec<Variable<T>>, context: &Vec<Context>) -> bool {
//     context.len() == config.len()
// }

// fn first<T: VarDes>(config: &[Variable<T>], context: &mut Vec<Context>) -> bool {
//     let v = config[context.len()].get_domain().first().unwrap();
//     context.push(Context {
//         var_id: context.len(),
//         domain_ele_id: 0,
//         partial: Some(*v),
//     });
//     true
// }

// fn next<T: VarDes>(config: &[Variable<T>], context: &mut [Context]) -> bool {
//     let tmp = context.last_mut().unwrap();
//     let next_dom_id = tmp.domain_ele_id + 1;
//     if next_dom_id >= config[tmp.var_id].get_domain().len() {
//         return false;
//     }
//     tmp.domain_ele_id = next_dom_id;
//     tmp.partial = Some(config[tmp.var_id].get_domain()[next_dom_id]);
//     true
// }

// fn backtrack_int<T: VarDes>(config: &Vec<Variable<T>>, context: &mut Vec<Context>) -> bool {
//     // print_context(context);
//     if reject(config, context) {
//         return false;
//     }
//     if accept(config, context) {
//         return true;
//     }
//     let mut s = first(config, context);
//     while s {
//         if backtrack_int(config, context) {
//             return true;
//         }
//         s = next(config, context);
//     }
//     context.pop();
//     false
// }
// fn print_solution(partial: &[Context]) {
//     let mut s = String::new();
//     partial
//         .iter()
//         .for_each(|i| s.push_str(&ele_to_str(i)));
//     println!("{}", s);
// }
// fn ele_to_str(e: &Context) -> String {
//     let a = e.partial.unwrap();
//     format!("| {} - {} - {} |", e.var_id, e.domain_ele_id, a)
// }

// fn backtrack<T: VarDes>(config: &Vec<Variable<T>>) -> bool {
//     let mut context = Vec::with_capacity(config.len());
//     backtrack_int(config, &mut context);
//     print_solution(&context);
//     true
// }

// fn main() {
//     let n = 4;
//     // let config = buildQueens(n);
//     // let config = build_sudoku();
//     let config = build_queens(39);

//     backtrack(&config);
// }
