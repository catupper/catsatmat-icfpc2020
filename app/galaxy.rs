use app::data::GALAXY;
use app::{
    parse,
    Expr::{self, *},
    Interpreter,
};

use std::collections::{HashMap, HashSet};

#[allow(unused)]
fn calc_reachability(edge: &HashMap<i32, HashSet<i32>>) -> HashMap<(i32, i32), bool> {
    let mut reachability = HashMap::new();
    for from in edge.keys() {
        for to in edge.keys() {
            reachability.insert((*from, *to), false);
        }
    }

    fn dfs(
        from: i32,
        now: i32,
        reachability: &mut HashMap<(i32, i32), bool>,
        edge: &HashMap<i32, HashSet<i32>>,
    ) {
        let come = reachability.get_mut(&(from, now)).unwrap();
        if *come {
            return;
        }
        *come = true;
        for to in edge.get(&now).unwrap() {
            dfs(from, *to, reachability, edge);
        }
    }

    for (id, neighbs) in edge.iter() {
        for to in neighbs {
            dfs(*id, *to, &mut reachability, &edge);
        }
    }
    reachability
}

fn main() {
    let exps = parse(GALAXY);
    let interpreter = Interpreter::new(exps.clone());
    let galaxy_exp = exps.get(&-1).unwrap().clone();
    let galaxy = interpreter.apply(galaxy_exp);
    let new_expr = Expr::ap(Expr::ap(galaxy, Nil), Expr::vector(0, 0));
    let hoge = interpreter.apply(new_expr);
    println!("{:?}", hoge);
    /*
    for (id, exp) in exps {
        if id != 1342 {
            continue;
        }
        println!(":{} =", id);
        println!("{:?}", interpreter.apply(exp));
        println!();
        println!();
    }*/
}
/*
fn hoge() {
    let piyo = Ap(
        Ap(
            Cst("b"),
            Ap(
                Cst("s"),
                Ap(
                    Ap(Cst("c"), Ap(Cst("eq"), Int(0))), Cst("nil")
                ),
            ),
        ),
        Ap(
            Ap(
                Cst("s"),
                Ap(
                    Ap(Cst("b"), Cst("b")),
                    Ap(
                        Ap(Cst("b"), Def(1115)),
                        Ap(
                            Ap(
                                Cst("c"),
                                Ap(
                                    Ap(
                                        Cst("s"),
                                        Ap(
                                            Ap(Cst("b"), Cst("b")),
                                            Ap(
                                                Ap(
                                                    Cst("c"),
                                                    Ap(Ap(Cst("b"), Cst("b")), Cst("add")),
                                                ),
                                                Cst("neg"),
                                            ),
                                        ),
                                    ),
                                    Ap(Ap(Cst("b"), Ap(Cst("s"), Cst("mul"))), Cst("div")),
                                ),
                            ),
                            Int(2),
                        ),
                    ),
                ),
            ),
            Ap(
                Ap(
                    Cst("c"),
                    Ap(
                        Ap(Cst("b"), Cst("b")),
                        Ap(
                            Ap(Cst("b"), Def(1208)),
                            Ap(Ap(Cst("c"), Cst("div")), Int(2)),
                        ),
                    ),
                ),
                Ap(Cst("add"), Int(-1)),
            ),
        ),
    );
}
*/