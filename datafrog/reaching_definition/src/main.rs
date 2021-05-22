mod facts;
use datafrog::{Iteration, Relation, RelationLeaper, Variable};
use facts::{Definition, Point};

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

    // definition at Point
    let def: Relation<(Point, Definition)> = vec![
        (Point::from(1), Definition::from(1)),
        (Point::from(2), Definition::from(2)),
        (Point::from(3), Definition::from(3)),
        (Point::from(4), Definition::from(4)),
        (Point::from(5), Definition::from(5)),
        (Point::from(6), Definition::from(6)),
        (Point::from(7), Definition::from(7)),
        (Point::from(8), Definition::from(8)),
    ]
    .iter()
    .collect();

    let mut v = vec![
        // x
        (Definition::from(1), Definition::from(5)),
        (Definition::from(1), Definition::from(7)),
        (Definition::from(5), Definition::from(7)),
        // y
        (Definition::from(2), Definition::from(4)),
        // z
        (Definition::from(6), Definition::from(8)),
    ];
    let mut v_rev = v.iter().map(|&(u, v)| (v, u)).collect();
    v.append(&mut v_rev);
    let conflict: Relation<(Definition, Definition)> = v.iter().collect();

    let reach = iteration.variable::<(Point, Definition)>("reach");
    reach.insert(def.clone());

    // kill(d2,p):-
    //  def(p,d1),
    //  conflict(d1,d2).
    let kill: Relation<(Definition, Point)> = Relation::from_leapjoin(
        &def,
        conflict.extend_with(|&(_p, d1)| d1),
        |&(p, _d1), &d2| (d2, p),
    );

    while iteration.changed() {
        // reach(point2, definition) :-
        //   reach(point1, definition),
        //   edge(point1, point2),
        //   not kill(definition, point2).
        //

        reach.from_leapjoin(
            &reach,
            (
                edge.extend_with(|&(point1, definition)| point1),
                kill.extend_anti(|&(point1, definition)| definition),
            ),
            |&(point1, definition), &point2| (point2, definition),
        );
    }

    let x = reach.complete();
    assert_eq!(x.elements.len(),31);
    println!("{:?}", x.elements);
}
