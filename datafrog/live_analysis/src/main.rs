mod facts;
use datafrog::{Iteration, Relation, RelationLeaper};
use facts::{Variable, Point};

fn main() {
    let mut iteration = Iteration::new();

    let edge: Relation<(Point, Point)> = vec![
        (Point::from(1), Point::from(2)),
        (Point::from(2), Point::from(3)),
        (Point::from(6), Point::from(3)),
        (Point::from(3), Point::from(4)),
        (Point::from(4), Point::from(5)),
        (Point::from(4), Point::from(7)),
        (Point::from(5), Point::from(6)),
        (Point::from(6), Point::from(8)),
        (Point::from(7), Point::from(8)),
    ]
    .iter()
    .collect();

    let rev_edge:Relation<(Point,Point)> = Relation::from_iter(edge.elements.iter().map(|&(u,v)|(v,u)));

    // definition at Point
    let def: Relation<(Variable,Point)> = vec![
        (Variable::from('x' as usize),Point::from(1) ),
        (Variable::from('y' as usize),Point::from(2) ),
        (Variable::from('m' as usize),Point::from(3) ),
        (Variable::from('y' as usize),Point::from(4) ),
        (Variable::from('x' as usize),Point::from(5) ),
        (Variable::from('q' as usize),Point::from(6) ),
        (Variable::from('x' as usize),Point::from(7) ),
        (Variable::from('z' as usize),Point::from(8) ),
    ]
    .iter()
    .collect();

    let live= iteration.variable::<(Point, Variable)>("live");
    live.extend(vec![
                (Point::from(1),Variable::from('p' as usize)),
                (Point::from(2),Variable::from('q' as usize)),
                (Point::from(2),Variable::from('z' as usize)),
                (Point::from(3),Variable::from('k' as usize)),
                (Point::from(4),Variable::from('m' as usize)),
                (Point::from(6),Variable::from('y' as usize)),
                (Point::from(7),Variable::from('x' as usize)),
                (Point::from(8),Variable::from('p' as usize)),
    ].iter());

    while iteration.changed() {
        // live(point1, variable) :-
        //   live(point2, variable),
        //   rev_edge(point2, point1),
        //   not def(variable, point1).
        //

        live.from_leapjoin(
            &live,
            (
                rev_edge.extend_with(|&(point2, variable)| point2),
                def.extend_anti(|&(point2, variable)| variable),
            ),
            |&(point1, variable), &point2| (point2, variable),
        );
    }

    let x = live.complete();
    println!("{:?}", x.elements.len());
    println!("{:?}", x.elements);
}

//                |-----------|
//                | p1        |
//                |   x=p+1;  |
//                | p2        |
//                |   y=q+z;  |
//                |-----------|
//                     |
//                     |
//                |-----------|
//                | p3        |
//     |----------|   m=k;    |
//     |          | p4        |
//     |          |   y=m-1;  |
//     |          |-----------|
//     |             /      \
//     |            /        \
//     |           /          \
//     |          /            \
//     |      |-----------|    |-----------| 
//     |      | p5        |    | p7        |
//     |      |   x=4;    |    |   x=x-3;  |
//     |------| p6        |    |-----------|
//            |   q=y;    |          /
//            |-----------|         / 
//                   \             /
//                    \           /
//                     \         /
//                      \       /
//                    |-----------| 
//                    | p8        |
//                    |   z=2p;   |
//                    |-----------|
//
