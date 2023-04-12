use crate::{var_des::VarDes, variable::Variable};

#[derive(Copy, Clone)]
pub struct SudokuVar {
    x: usize,
    y: usize,
}

impl SudokuVar {
    pub fn new(x: usize, y: usize) -> SudokuVar {
        SudokuVar { x, y }
    }
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
}

impl VarDes for SudokuVar {
    fn is_valid(&self, other: &Self, current: &Self::VarVal, checked: &Self::VarVal) -> bool {
        !((self.get_x() == other.get_x() || self.get_y() == other.get_y()) && (current == checked))
    }

    type VarVal = i32;
}

fn unary_constrainsts(config: &mut Vec<Variable<SudokuVar>>) {
    let v = vec![
        (1, 0, 6),
        (2, 0, 1),
        (5, 0, 7),
        (8, 0, 3),
        (1, 1, 9),
        (2, 1, 2),
        (5, 1, 3),
        (2, 3, 8),
        (3, 3, 5),
        (4, 3, 3),
        (6, 4, 5),
        (8, 4, 4),
        (0, 5, 5),
        (5, 5, 8),
        (1, 6, 4),
        (8, 6, 1),
        (3, 7, 1),
        (4, 7, 6),
        (5, 7, 8),
        (0, 8, 6),
    ];

    for i in v {
        let j = i.0 + i.1 * 9;
        config[j].set_domain_value(i.2 - 1);
    }
}

pub fn build_sudoku() -> Vec<Variable<SudokuVar>> {
    let mut config: Vec<Variable<SudokuVar>> = (0..81)
        .map(|i| Variable::new(SudokuVar::new(i / 9, i % 9), (0..9).map(|j| j).collect(), i))
        .collect();
    unary_constrainsts(&mut config);
    config
}

// pub fn pritn_variables1(ct: &ConfigTank<SudokuVar>) {
//     for var in ct.get_variables() {
//         println!("{} , {} ", var.get_state().get_x(), var.get_state().get_y());
//         var.get_domain().print_vals();
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{sudoku::SudokuVar, var_des::VarDes};
    // use crate::sudoku::VarDes::is_valid;

    fn is_valid_h(x: (usize, usize, i32), y: (usize, usize, i32)) -> bool {
        SudokuVar::new(x.0, x.1).is_valid(&SudokuVar::new(y.0, y.1), &x.2, &y.2)
    }

    #[test]
    fn dupa() {
        assert!(is_valid_h((0, 0, 0), (1, 2, 1)));
        assert!(is_valid_h((0, 0, 0), (1, 2, 0)));

        assert!(is_valid_h((0, 0, 0), (1, 0, 1)));
        assert!(!is_valid_h((0, 0, 0), (1, 0, 0)));

        assert!(is_valid_h((0, 0, 0), (0, 1, 1)));
        assert!(!is_valid_h((0, 0, 0), (0, 1, 0)));

        assert!(is_valid_h((0, 0, 0), (0, 0, 1)));
        assert!(!is_valid_h((0, 0, 0), (0, 0, 0)));
    }
}
