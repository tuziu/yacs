use crate::var_des::VarDes;
use crate::variable::Variable;

type VarId = usize;

pub struct Context<T: VarDes> {
    partials: Vec<Option<T::VarVal>>,
    order: Vec<VarId>,
    // p: PhantomData<T>,s
    // constraints: HashMap<(VarId, VarId),>,
}

impl<T: VarDes> Context<T> {
    pub fn new(config: &Vec<Variable<T>>) -> Context<T>
    where
        <T as VarDes>::VarVal: Copy,
    {
        Context {
            partials: vec![None; config.len()],
            order: vec![],
        }
    }

    pub fn assigned_partials(&self) -> usize {
        self.partials.iter().filter(|x| x.is_some()).count()
    }

    pub fn get_unassigend_partials(&self) -> Vec<VarId> {
        self.partials
            .iter()
            .enumerate()
            .filter(|x| x.1.is_none())
            .map(|x| x.0)
            .collect()
    }

    fn get_partials_with_enumeration(&self) -> Vec<(VarId, T::VarVal)>
    where
        <T as VarDes>::VarVal: Copy,
    {
        self.partials
            .iter()
            .enumerate()
            .filter(|x| x.1.is_some())
            .map(|x| (x.0, x.1.unwrap()))
            .collect()
    }

    fn set_partial(&mut self, var_id: VarId, value: T::VarVal) {
        self.partials[var_id] = Some(value);
    }

    fn unset_partial(&mut self, var_id: VarId) {
        self.partials[var_id] = None;
    }
}

pub fn backtrack<T: VarDes>(config: &Vec<Variable<T>>) -> Vec<T::VarVal>
where
    <T as VarDes>::VarVal: Copy,
{
    let mut contex = Context::new(config);
    backtrack_int(config, &mut contex);
    contex.partials.iter().map(|x| x.unwrap()).collect()
}

fn backtrack_int<T: VarDes>(config: &Vec<Variable<T>>, context: &mut Context<T>) -> bool
where
    <T as VarDes>::VarVal: Copy,
{
    if accept(config, context) {
        return true;
    }

    let var_id = get_new_unnassigned_variable(config, context);
    for value in get_domain(config, var_id) {
        if is_consistent(config, context, var_id, value) {
            context.set_partial(var_id, *value);
            if backtrack_int(config, context) {
                return true;
            }
            context.unset_partial(var_id);
        }
    }
    false
}

fn is_consistent<T: VarDes>(
    config: &[Variable<T>],
    context: &Context<T>,
    var_id: VarId,
    value: &T::VarVal,
) -> bool
where
    <T as VarDes>::VarVal: Copy,
{
    let partials = context.get_partials_with_enumeration();
    for (id, val) in partials {
        if !config[var_id]
            .state()
            .is_valid(config[id].state(), value, &val)
        {
            return false;
        }
    }
    true
}

fn get_domain<T: VarDes>(config: &[Variable<T>], var_id: VarId) -> &Vec<T::VarVal> {
    config[var_id].get_domain()
}

fn get_new_unnassigned_variable<T: VarDes>(config: &[Variable<T>], context: &Context<T>) -> VarId {
    let mut unassigned = context.get_unassigend_partials();
    unassigned.sort();
    *unassigned.first().unwrap()
}

fn accept<T: VarDes>(config: &Vec<Variable<T>>, context: &Context<T>) -> bool {
    context.assigned_partials() == config.len()
}
