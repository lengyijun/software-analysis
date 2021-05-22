use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy)]
enum Element {
    Constant(u32),
    Var(char),
}

#[derive(Clone)]
struct Expression {
    /// +,-,*,/
    op: char,
    /// u32
    /// char
    elements: Vec<Element>,
    index: u32,
}

struct IR {
    def: char,
    expression: Expression,
}

struct BB {
    irs: Vec<IR>,
    /// from CFG
    prev: Vec<Rc<RefCell<BB>>>,
    /// generate from irs
    kill: u32,
    /// generate from irs
    gen: u32,
    /// change in every iteration
    out: u32,
}

impl BB {
    fn new(irs: Vec<IR>) -> Self {
        let gen: u32 = irs
            .iter()
            .fold(0u32, |res, ir| res | (1 << ir.expression.index));
        BB {
            irs,
            prev: vec![],
            kill: 0u32,
            gen,
            out: (1 << 10) - 1,
        }
    }

    fn entry() -> Self {
        BB {
            irs: vec![],
            prev: vec![],
            kill: 0,
            gen: 0,
            out: 0,
        }
    }
}

/// control flow graph
struct CFG {
    bbs: Vec<Rc<RefCell<BB>>>,
}

impl CFG {
    fn new(bbs: Vec<Rc<RefCell<BB>>>) -> Self {
        let mut dp: Vec<Vec<u32>> = vec![vec![]; 26];
        for bb in &bbs {
            for ir in &(*bb).borrow().irs {
                for e in &ir.expression.elements {
                    if let Element::Var(x) = e {
                        dp[(x.clone() as i32 - 'a' as i32) as usize].push(ir.expression.index);
                    }
                }
            }
        }

        for bb in &bbs {
            let mut kill = 0;
            for ir in &(*bb).borrow().irs {
                kill |= dp[(ir.def as i32 - 'a' as i32) as usize]
                    .iter()
                    .fold(0u32, |res, x| res | 1 << x);
            }
            (*bb).borrow_mut().kill = kill;
        }

        CFG { bbs }
    }

    fn solve(&mut self) {
        loop {
            let mut changed = false;
            for bb in &self.bbs {
                let mut bb = (*bb).borrow_mut();
                let input: u32 = bb
                    .prev
                    .iter()
                    .fold((1 << 10) - 1, |res, prev| res & (*prev).borrow().out);
                let out = bb.gen | (input & (input ^ bb.kill));
                if out != bb.out {
                    bb.out = out;
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
    use crate::Element::Constant;
    use crate::Element::Var;
    use crate::Expression;
    use crate::BB;
    use crate::CFG;
    use crate::IR;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn it_works() {
        //p-1
        let ex1 = Expression {
            op: '-',
            elements: vec![Var('p'), Constant(1)],
            index: 4,
        };

        // z/5
        let ex2 = Expression {
            op: '/',
            elements: vec![Var('z'), Constant(5)],
            index: 3,
        };
        // 2*y
        let ex3 = Expression {
            op: '*',
            elements: vec![Constant(2), Var('y')],
            index: 2,
        };
        // 10^7 * x
        let ex4 = Expression {
            op: '*',
            elements: vec![Constant(10_000_000), Var('x')],
            index: 1,
        };
        // y+3
        let ex5 = Expression {
            op: '+',
            elements: vec![Var('y'), Constant(3)],
            index: 0,
        };

        // y=p-1
        let ir1 = IR {
            def: 'y',
            expression: ex1.clone(),
        };

        // k=z/5
        let ir2 = IR {
            def: 'k',
            expression: ex2.clone(),
        };

        // p=10^7 * x
        let ir3 = IR {
            def: 'p',
            expression: ex4.clone(),
        };

        // x=2*y
        let ir4 = IR {
            def: 'x',
            expression: ex3.clone(),
        };

        // q=10^7 * x
        let ir5 = IR {
            def: 'q',
            expression: ex4.clone(),
        };

        // z = y+3
        let ir6 = IR {
            def: 'z',
            expression: ex5.clone(),
        };

        // m=10^7*x
        let ir7 = IR {
            def: 'm',
            expression: ex4.clone(),
        };

        // y=z/5
        let ir8 = IR {
            def: 'y',
            expression: ex2.clone(),
        };

        let mut entry = Rc::new(RefCell::new(BB::entry()));
        let mut b1 = Rc::new(RefCell::new(BB::new(vec![ir1])));
        let mut b2 = Rc::new(RefCell::new(BB::new(vec![ir2, ir3])));
        let mut b3 = Rc::new(RefCell::new(BB::new(vec![ir6])));
        let mut b4 = Rc::new(RefCell::new(BB::new(vec![ir4, ir5])));
        let mut b5 = Rc::new(RefCell::new(BB::new(vec![ir7, ir8])));

        (*b1).borrow_mut().prev.push(entry.clone());
        (*b2).borrow_mut().prev.push(b1.clone());
        (*b2).borrow_mut().prev.push(b4.clone());
        (*b3).borrow_mut().prev.push(b2.clone());
        (*b4).borrow_mut().prev.push(b2.clone());
        (*b5).borrow_mut().prev.push(b3.clone());
        (*b5).borrow_mut().prev.push(b4.clone());

        // don't put entry into
        // otherwise will modify entry's out
        let mut cfg = CFG::new(vec![
            b1.clone(),
            b2.clone(),
            b3.clone(),
            b4.clone(),
            b5.clone(),
        ]);
        cfg.solve();
        assert_eq!(16, (*b1).borrow().out);
        assert_eq!(10, (*b2).borrow().out);
        assert_eq!(3, (*b3).borrow().out);
        assert_eq!(14, (*b4).borrow().out);
        assert_eq!(10, (*b5).borrow().out);
    }
}
