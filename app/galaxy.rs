use app::data::GALAXY;
use app::{parse, Interpreter};

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

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
    let mut refer_to: HashMap<i32, HashSet<i32>> = HashMap::from_iter(
        exps.clone()
            .into_iter()
            .map(|(id, exp)| (id, exp.travarse_defs())),
    );
    for (id, vec) in refer_to.iter_mut() {
        if vec.remove(&id) {
            println!("{}", id);
        }
    }
    //    println!("{:?}", refer_to);
    let reachability = calc_reachability(&refer_to);
    let in_loop: Vec<i32> = refer_to
        .keys()
        .cloned()
        .filter(|id| *reachability.get(&(*id, *id)).unwrap())
        .collect();
    println!("{:?}", in_loop);
    for id in &in_loop {
        //        println!("{}: {:?}\n", id, exps.get(&id).unwrap())
        println!(
            "{}: {:?}\n",
            id,
            refer_to
                .get(id)
                .unwrap()
                .iter()
                .filter(|x| in_loop.contains(x))
                .collect::<Vec<_>>()
        )
    }
    let to_loop: Vec<i32> = refer_to
        .keys()
        .cloned()
        .filter(|id| {
            in_loop
                .iter()
                .any(|lp| *reachability.get(&(*id, *lp)).unwrap())
        })
        .collect();
    println!("{:?}", to_loop);
    //    let galaxy = exps.get(&-1).unwrap().clone();
    let galaxy = exps.get(&1208).unwrap().clone();
    println!("{:?}", galaxy);
    let interpreter = Interpreter::new(exps);
    println!("{:?}", interpreter.apply(galaxy));
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
