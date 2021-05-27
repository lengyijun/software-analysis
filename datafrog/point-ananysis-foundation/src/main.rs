/// https://www.bilibili.com/video/BV1rK4y1p79j/?spm_id_from=333.788.b_7265636f5f6c697374.2
mod facts;
use datafrog::{Iteration, Relation, RelationLeaper};
use facts::{Field, Point, Variable, Variable_or_field};

fn main() {
    let mut iteration = Iteration::new();

    // line 1: b= new C();
    // line 3: c= new C();
    let new: Relation<(Variable, Point)> = vec![
        (Variable::from('b' as usize), Point::from(1)),
        (Variable::from('c' as usize), Point::from(3)),
    ]
    .iter()
    .collect();

    // line 2: a=b;
    // line 5: d=c;
    let assign: Relation<(Variable, Variable)> = vec![
        (Variable::from('b' as usize), Variable::from('a' as usize)),
        (Variable::from('c' as usize), Variable::from('d' as usize)),
    ]
    .iter()
    .collect();

    // line 4: c.f=a;
    // line 6: c.f=d;
    let store: Relation<(Field, Variable)> = vec![
        (
            Field {
                v: Variable::from('c' as usize),
                f: 'f',
            },
            Variable::from('a' as usize),
        ),
        (
            Field {
                v: Variable::from('c' as usize),
                f: 'f',
            },
            Variable::from('d' as usize),
        ),
    ]
    .iter()
    .collect();

    // line 7: e=d.f;
    let load: Relation<(Variable, Field)> = vec![(
        Variable::from('e' as usize),
        Field {
            v: Variable::from('d' as usize),
            f: 'f',
        },
    )]
    .iter()
    .collect();

    let pt = iteration.variable::<(Variable, Point)>("pt");
    pt.extend(new.iter());
    // todo
    let edge = iteration.variable::<(Variable, Point)>("edge");

    while iteration.changed() {
        pt.from_leapjoin(&pt, assign.extend_with(|&(v, p)| v), |&(_, p), &v| (v, p));
    }
    let x = pt.complete();
    println!("{}", x.elements.len());
    println!("{:?}", x.elements);
}
