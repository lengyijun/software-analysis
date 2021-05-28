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
    let load: Relation<( Variable_or_field,(char,Variable_or_field))> = vec![
        (Variable_or_field::variable('d'),('f',Variable_or_field::variable('e')))
    ]
    .iter()
    .collect();

    let pt = iteration.variable::<(Variable_or_field, Point)>("pt");
    pt.extend(new.iter());

    let edge = iteration.variable::<(Variable_or_field, Variable_or_field)>("edge");
    edge.extend(assign.iter());

    // temp1
    let t1=iteration.variable::<(Variable_or_field,(Point,char))>("t1");
    // temp2
    let t2=iteration.variable::<(Variable_or_field,Variable_or_field)>("t2");

    while iteration.changed() {

        // pt(x,oi):- pt(y,oi), edge(y,x)).
        pt.from_join(&pt,&edge,|&_y,&o,&x|(x,o));

        // t1(p1,f,y) :- pt(pi,x) , x=f.y.
        t1.from_leapjoin(&pt,store.extend_with(|&(x,p)| x),|&(x,p),&(c,y)|(y,(p,c)) );

        // pt(oi.f,oj) :- t1(y,(oi,f)), pt(y,oj).
        pt.from_join(&t1,&pt,|&_y,&(oi,f),&oj| (Variable_or_field::field(oi,f),oj));

        edge.from_join(&pt,&store,|&_x,&o,&(f,y)| (y,Variable_or_field::field(o,f)));

        // t2((oi,f),y) :- pt(oi,x),y=x.f.
        t2.from_leapjoin(&pt,load.extend_with(|&(x,_p)|x),|&(_x,p),&(f,y)| (Variable_or_field::field(p,f),y));

        // pt(y,oj):- t2((oi,f),y),pt((oi,f),oj).
        pt.from_join(&t2,&pt,|&_z,&y,&oj|(y,oj));

        edge.from_join(&pt,&load,|&_x,&o,&(f,y)|(Variable_or_field::field(o,f),y));
    }

    let x = pt.complete();
    println!("{}", x.elements.len());
    println!("{:?}", x.elements);

    let e=edge.complete();
    println!("{}", e.elements.len());
    println!("{:?}", e.elements);
}

// b=new C();
// a=b;
// c=new C();
// c.f=a;
// d=c;
// c.f=d;
// e=d.f
//
//                          {o1}     {o1}
//                     |-----a <------b
// {o1,o3}  {o1,o3}    |
// e  <----- o3.f <----|
//                     |
//                     |-----d <------c
//                          {o3}     {o3}
// 

