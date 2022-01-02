// use std::env::current_dir;
use mylib::{variable::Variable, queens::QueensVar, var_des::VarDes};

type VarId = usize;
type EnTy = i32;

struct Context{
    var_id: VarId,
    domain_ele_id: usize,
    // domain_to_check: Vec<EnTy>,
    partial: Option<EnTy>,
}

fn reject(config: &Vec<Variable<QueensVar>>, context: &Vec<Context>) -> bool {
    let var_id = context.last().unwrap().var_id;
    let current = &config[var_id];
    for i in 0..var_id {
        // printState(mill, pos);
        let local = &config[i];
        let d = current.get_state().is_valid(local.get_state(), context[var_id].partial.unwrap(), context[i].partial.unwrap());
        if !d {
            return true;
        }
    }
    false
}
fn accept(config: &Vec<Variable<QueensVar>>, context: &Vec<Context>) -> bool {
    config.len() <= context.len()
}

fn first(config: &Vec<Variable<QueensVar>> ,context: &mut Vec<Context>) -> bool {
    let  v = config[0].get_domain().first().unwrap();
    context.push(Context{ var_id: context.len(),domain_ele_id: 0 , partial: Some(*v)});
    true
}

fn next(config: &Vec<Variable<QueensVar>> ,context: &mut Vec<Context>) -> bool{
    let tmp = context.last_mut().unwrap();
    let next_dom_id = tmp.domain_ele_id +1;
    if next_dom_id >= config[tmp.var_id].get_domain().len() {
        return false;
    }
    tmp.domain_ele_id = next_dom_id;
    true
}

fn backtrack(config: &Vec<Variable<QueensVar>>, context: &mut Vec<Context>) -> bool {
    if accept(config,context){
        return true;
    } else if reject(config,context){
        return false;
    }
    let mut s = first(config,context);
    while s {
        if backtrack(config,context) {
            return true;
        }
        s = next(config , context);
    }
    false
}

fn main(){

}