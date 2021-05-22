use std::cell::RefCell;
use std::rc::Rc;

/// a=b+3 IR{def:a, used:[b]}
/// a=b+c IR{def:a, used:[b,c]}
/// a=a+1 IR{def:a, used:[a]}
struct IR {
    def: i32,
    /// every element is in 0-25
    used: Vec<usize>,
}

impl IR {
    fn new(s: &str) -> Self {
        let s = s.trim_start().to_string();
        let mut ir = IR {
            def: 0,
            used: vec![],
        };
        let mut it = s.chars();
        ir.def = it.next().unwrap() as i32 - 'a' as i32;
        for c in it {
            let a: i32 = c as i32 - 'a' as i32;
            if a >= 0 {
                ir.used.push(a as usize);
            }
        }
        ir
    }
}

/// basic block
struct BB {
    irs: Vec<IR>,
    /// static
    def: u64,
    /// static
    used: u64,
    /// change in every iteration
    /// if all input don't change, the loop finished
    input: u64,
    /// backward analysis. So we need all previous BBs
    next: Vec<Rc<RefCell<BB>>>,
}

impl BB {
    fn new(irs: Vec<IR>) -> Self {
        let mut bb = BB {
            irs,
            def: 0,
            used: 0,
            input: 0,
            next: vec![],
        };
        /// 先def,再use    -> 相当于def
        /// 先use,再def    -> 相当于use
        /// 同一行 def+use -> 相当于use
        for ir in &bb.irs {
            for u in &ir.used {
                // not defined
                if (bb.def & (1 << u)) == 0 {
                    bb.used |= (1 << u);
                }
            }
            bb.def |= 1 << (ir.def);
        }
        bb
    }
}

/// control flow graph
struct CFG {
    bbs: Vec<Rc<RefCell<BB>>>,
}

impl CFG {
    fn solve(&mut self) {
        loop {
            let mut changed = false;
            for bb in &self.bbs {
                let out_b: u64 = bb
                    .borrow()
                    .next
                    .iter()
                    .map(|x| x.borrow().input)
                    .fold(0u64, |a, b| a | b);
                let mut bb = bb.borrow_mut();
                let input: u64 = bb.used | (out_b & (out_b ^ bb.def));
                if input != bb.input {
                    bb.input = input;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BB;
    use crate::CFG;
    use crate::IR;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_if() {
        let ir = IR::new("x=p+1");
        assert_eq!(ir.def, (b'x' - b'a') as i32);
        //assert_eq!(ir.used,);
    }

    #[test]
    fn test_cfg() {
        let b1 = Rc::new(RefCell::new(BB::new(vec![
            IR::new("x=p+1"),
            IR::new("y=q+z"),
        ])));
        let b2 = Rc::new(RefCell::new(BB::new(vec![
            IR::new("m=k"),
            IR::new("y=m-1"),
        ])));
        let b3 = Rc::new(RefCell::new(BB::new(vec![IR::new("x=x-3")])));
        let b4 = Rc::new(RefCell::new(BB::new(vec![IR::new("x=4"), IR::new("q=y")])));
        let b5 = Rc::new(RefCell::new(BB::new(vec![IR::new("z=2p")])));

        (*b1).borrow_mut().next.push(b2.clone());
        (*b2).borrow_mut().next.push(b3.clone());
        (*b2).borrow_mut().next.push(b4.clone());
        (*b3).borrow_mut().next.push(b5.clone());
        (*b4).borrow_mut().next.push(b5.clone());
        (*b4).borrow_mut().next.push(b2.clone());

        let mut cfg = CFG {
            bbs: vec![b1.clone(), b2.clone(), b3.clone(), b4.clone(), b5.clone()],
        };
        cfg.solve();
        assert_eq!((*b5).borrow().input, vec_to_res(vec![b'p']));
        assert_eq!((*b3).borrow().input, vec_to_res(vec![b'x', b'p']));
        assert_eq!((*b4).borrow().input, vec_to_res(vec![b'y', b'p', b'k']));
        assert_eq!((*b2).borrow().input, vec_to_res(vec![b'x', b'p', b'k']));
        assert_eq!(
            (*b1).borrow().input,
            vec_to_res(vec![b'p', b'q', b'z', b'k'])
        );
    }

    fn vec_to_res(input: Vec<u8>) -> u64 {
        input
            .iter()
            .map(|x| 1 << (x - b'a'))
            .fold(0u64, |a, b| a | b)
    }
}
