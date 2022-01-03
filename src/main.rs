// use std::env::current_dir;
use mylib::{queens::QueensVar, var_des::VarDes, variable::Variable};

type VarId = usize;
type EnTy = i32;

struct Context {
    var_id: VarId,
    domain_ele_id: usize,
    // domain_to_check: Vec<EnTy>,
    partial: Option<EnTy>,
}

fn print_context(context: &Vec<Context>) {
    let d: Vec<String> = context
        .iter()
        .map(|c| {
            format!(
                "vi: {} di: {} p: {:?}",
                c.var_id, c.domain_ele_id, c.partial
            )
        })
        .collect();
    println!(" {} ", d.concat());
}

fn reject(config: &Vec<Variable<QueensVar>>, context: &Vec<Context>) -> bool {
    context.last().map_or(false, |t| {
        let var_id = t.var_id;
        for i in 0..var_id {
            if ! config[var_id].state().is_valid(
                config[i].state(),
                context[var_id].partial.unwrap(),
                context[i].partial.unwrap(),
            ) {
                return true;
            }
        }
        false
    })
}

fn accept(config: &Vec<Variable<QueensVar>>, context: &Vec<Context>) -> bool {
    config.len() <= context.len()
}

fn first(config: &Vec<Variable<QueensVar>>, context: &mut Vec<Context>) -> bool {
    let v = config[0].get_domain().first().unwrap();
    context.push(Context {
        var_id: context.len(),
        domain_ele_id: 0,
        partial: Some(*v),
    });
    true
}

fn next(config: &Vec<Variable<QueensVar>>, context: &mut Vec<Context>) -> bool {
    let tmp = context.last_mut().unwrap();
    let next_dom_id = tmp.domain_ele_id + 1;
    if next_dom_id >= config[tmp.var_id].get_domain().len() {
        return false;
    }
    tmp.domain_ele_id = next_dom_id;
    tmp.partial = Some(config[tmp.var_id].get_domain()[next_dom_id]);
    true
}

fn backtrack_int(config: &Vec<Variable<QueensVar>>, context: &mut Vec<Context>) -> bool {
    // print_context(context);
    if reject(config, context) {
        return false;
    } else if accept(config, context) {
        return true;
    }
    let mut s = first(config, context);
    while s {
        if backtrack_int(config, context) {
            return true;
        }
        s = next(config, context);
    }
    context.pop();
    false
}
fn print_solution(partial: &Vec<Context>) {
    let mut s = String::new();
    partial
        .iter()
        .enumerate()
        .for_each(|i| s.push_str(&ele_to_str(i)));
    println!("{}", s);
}
fn ele_to_str(e: (usize, &Context)) -> String {
    let a = e.1.partial.unwrap();
    format!("| {} - {} |", e.0, a)
    // String::new()
}

fn backtrack(config: &Vec<Variable<QueensVar>>) -> bool {
    let mut context = Vec::with_capacity(config.len());
    backtrack_int(config, &mut context);
    print_solution(&context);
    true
}

fn main() {
    let n = 31;
    let config: Vec<Variable<QueensVar>> = (0..n)
        .map(|i| Variable::new(QueensVar::new(i), (0..n).map(|j| j as EnTy).collect(), i))
        .collect();

    backtrack(&config);
    // let mut b = ConfigTankBuilder::new();
    // add_variable(QueensVar::new(0))
    //     .with_domain((1..4).map(|i| i as EnTy).collect())
    //     .to(&mut b);
    // for i in 1..4 {
    //     add_variable(QueensVar::new(i))
    //         .with_domain((0..4).map(|i| i as EnTy).collect())
    //         .to(&mut b);
    // }
    // b.finalize()
}
