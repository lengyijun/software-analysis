/// https://www.bilibili.com/video/BV1rK4y1p79j/?spm_id_from=333.788.b_7265636f5f6c697374.2
mod facts;
use datafrog::{Iteration, Relation, RelationLeaper};
use facts::{ Point,  Variable_or_field};

fn main() {
    let mut iteration = Iteration::new();

    // line 1: b= new C();
    // line 3: c= new C();
    let new: Relation<(Variable_or_field, Point)> = vec![
        (Variable_or_field::variable('b' ), Point::from(1)),
        (Variable_or_field::variable('c' ), Point::from(3)),
    ]
    .iter()
    .collect();

    // line 2: a=b;
    // line 5: d=c;
    let assign: Relation<(Variable_or_field, Variable_or_field)> = vec![
        (Variable_or_field::variable('b' ), Variable_or_field::variable('a' )),
        (Variable_or_field::variable('c' ), Variable_or_field::variable('d' )),
    ]
    .iter()
    .collect();

    // line 4: c.f=a;
    // line 6: c.f=d;
    let store: Relation<(Variable_or_field,(char,Variable_or_field))> = vec![
        (Variable_or_field::variable('c'),('f',Variable_or_field::variable('a'))),
        (Variable_or_field::variable('c'),('f',Variable_or_field::variable('d'))),
    ]
    .iter()
    .collect();

    // line 7: e=d.f;
    let load: Relation<(Variable_or_field, Variable_or_field,char)> = vec![
        (Variable_or_field::variable('e'),Variable_or_field::variable('d'),'f')
    ]
    .iter()
    .collect();

    let pt = iteration.variable::<(Variable_or_field, Point)>("pt");
    pt.extend(new.iter());

    let edge = iteration.variable::<(Variable_or_field, Point)>("edge");
    // temp1
    let t1=iteration.variable::<(Variable_or_field,(Point,char))>("t1");

    while iteration.changed() {

        // pt(x,oi):- pt(y,oi), x=y.
        pt.from_leapjoin(&pt, assign.extend_with(|&(v, p)| v), |&(_, p), &v| (v, p));

        // t1(p1,f,y) :- pt(pi,x) , x=f.y.
        t1.from_leapjoin(&pt,store.extend_with(|&(x,p)| x),|&(x,p),&(c,y)|(y,(p,c)) );

        // pt(oi.f,oj) :- t1(y,(oi,f)), pt(y,oj).
        pt.from_join(&t1,&pt,|&_y,&(oi,f),&oj| (Variable_or_field::field(oi,f),oj));
    }

    let x = pt.complete();
    println!("{}", x.elements.len());
    println!("{:?}", x.elements);
}
