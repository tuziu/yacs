use crate::{revise_result::ReviseResult, swap_container::SwapContainer};
use std::cell::RefCell;

type Ty = i32;
pub struct Domain {
    swap_container: RefCell<SwapContainer>,
}

impl Domain {
    pub fn new(d: Vec<Ty>) -> Domain {
        Domain {
            swap_container: RefCell::new(SwapContainer::new(d)),
        }
    }

    pub fn save_remove(&self, value: &Ty) -> Option<usize> {
        self.swap_container.borrow_mut().save_remove(value)
    }

    pub fn restore(&self, pos: usize) {
        self.swap_container.borrow_mut().restore(pos)
    }

    pub fn revise<F>(&self, other: &Self, f: F) -> ReviseResult
    where
        F: Fn(&Ty, &Ty) -> bool,
    {
        self.swap_container
            .borrow_mut()
            .revise(&other.swap_container.borrow(), f)
    }

    pub fn remove(&self, pos: usize) -> usize {
        self.swap_container.borrow_mut().remove(pos)
    }

    pub fn is_empty(&self) -> bool {
        self.swap_container.borrow().is_empty()
    }

    pub fn print_vals(&self) {
        self.swap_container.borrow().print();
    }

    pub fn get_domain(&self) -> Vec<Ty> {
        self.swap_container.borrow().iter().cloned().collect()
    }
}
