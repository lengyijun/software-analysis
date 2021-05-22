definition(d1).
definition(d2).
definition(d3).
definition(d4).
definition(d5).
definition(d6).
definition(d7).
definition(d8).

point(p1).
point(p2).
point(p3).
point(p4).
point(p5).
point(p6).
point(p7).
point(p8).

/* x */
conflict(d1,d5).
conflict(d1,d7).
conflict(d5,d7).

/* y */
conflict(d2,d4).

/* z */
conflict(d6,d8).

conflict(X,Y):-conflict(Y,X).

edge(p1,p2).
edge(p2,p3).
edge(p6,p3).
edge(p3,p4).
edge(p4,p5).
edge(p4,p7).
edge(p5,p6).
edge(p6,p8).
edge(p7,p8).

generate(p1,d1).
generate(p2,d2).
generate(p3,d3).
generate(p4,d4).
generate(p5,d5).
generate(p6,d6).
generate(p7,d7).
generate(p8,d8).

:- dynamic kill_memory/2.

% kill(P,D2):-kill_memory(P,D2),!.
% kill(P,D2):-generate(P,D1),conflict(D1,D2),assert(kill_memory(P,D2)).

% kill(p1,X).
% can't stop
kill(P,D2):-generate(P,D1),conflict(D1,D2).

reach(X,Y):-generate(X,Y).
reach(P2,D):- definition(D),point(P1),point(P2),reach(P1,D),edge(P1,P2),\+ kill(P2,D).

