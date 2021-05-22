use std::cell::RefCell;
use std::rc::Rc;

/// May analysis
/// forward analysis(前向分析)

/// .0: the index of ir
/// .1: the updated char. char is stored as `u8`
struct IR(usize, u8);

/// basic block
struct BB {
    irs: Vec<IR>,
    /// static
    gen: u64,
    /// static
    kill: u64,
    /// dynamic
    output: u64,
    /// forward analysis. So we need all previous BBs
    prev: Vec<Rc<RefCell<BB>>>,
}

impl BB {
    fn new(irs: Vec<IR>) -> Self {
        let gen: u64 = irs.iter().map(|a| a.0).fold(0u64, |a, b| a | (1 << b));
        let bb = BB {
            irs,
            gen,
            kill: 0,
            output: 0,
            prev: vec![],
        };
        bb
    }
}

/// control flow graph
struct CFG {
    bbs: Vec<Rc<RefCell<BB>>>,
}

impl CFG {
    fn new(mut bbs: Vec<Rc<RefCell<BB>>>) -> Self {
        let mut bitmaps: [u64; 26] = [0; 26];

        for bb in &bbs {
            for IR(i, c) in &(*bb).borrow().irs {
                bitmaps[(c - b'a') as usize] |= (1 << i);
            }
        }
        for bb in &mut bbs {
            let mut x: u64 = (*bb)
                .borrow()
                .irs
                .iter()
                .map(|a| bitmaps[(a.1 - b'a') as usize])
                .fold(0u64, |a, b| a | b);
            x &= (!(*bb).borrow().gen);
            (*bb).borrow_mut().kill = x;
        }
        CFG { bbs }
    }

    /// OUT=GEN U (IN - KILL )
    /// when no BB update, algorithm stops
    /// the count of 1 in `output` is monotonic increasing, so there is an upper limit of 1 in
    /// `output`. When reaching the limit, the algorithm will stop.
    fn solve(&mut self) {
        loop {
            let mut changed = false;
            for bb in &self.bbs {
                let input: u64 = bb
                    .borrow()
                    .prev
                    .iter()
                    .map(|x| (*x).borrow().output)
                    .fold(0u64, |a, b| a | b);
                let mut bb = (*bb).borrow_mut();
                let output: u64 = bb.gen | (input & (input ^ bb.kill));
                if output != bb.output {
                    bb.output = output;
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
    fn it_works() {
        let b1 = Rc::new(RefCell::new(BB::new(vec![IR(1, b'x'), IR(2, b'y')])));
        let b2 = Rc::new(RefCell::new(BB::new(vec![IR(3, b'm'), IR(4, b'y')])));
        let b3 = Rc::new(RefCell::new(BB::new(vec![IR(7, b'x')])));
        let b4 = Rc::new(RefCell::new(BB::new(vec![IR(5, b'x'), IR(6, b'z')])));
        let b5 = Rc::new(RefCell::new(BB::new(vec![IR(8, b'z')])));
        (*b2).borrow_mut().prev.push(b1.clone());
        (*b2).borrow_mut().prev.push(b4.clone());
        (*b4).borrow_mut().prev.push(b2.clone());
        (*b3).borrow_mut().prev.push(b2.clone());
        (*b5).borrow_mut().prev.push(b3.clone());
        (*b5).borrow_mut().prev.push(b4.clone());
        let bbs = vec![b1.clone(), b2.clone(), b3.clone(), b4.clone(), b5.clone()];
        let mut cfg = CFG::new(bbs);
        cfg.solve();
        assert_eq!((*b1).borrow().output, 0b0000_0011_0u64);
        assert_eq!((*b2).borrow().output, 0b0011_1101_0u64);
        assert_eq!((*b3).borrow().output, 0b0110_1100_0u64);
        assert_eq!((*b4).borrow().output, 0b0011_1100_0u64);
        assert_eq!((*b5).borrow().output, 0b1101_1100_0u64);
    }
}
