use app::data::GALAXY;
use app::{
    parse, Board,
    Expr::{self, *},
    Interpreter, Sender,
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

const DEFAULT_URL: &str = "https://icfpc2020-api.testkontur.ru";
const API_KEY: &str = "41ff8e29e5fa4596928186fcfe5bfee2";

fn main() {
    let exps = parse(GALAXY);
    let interpreter = Interpreter::new(
        exps.clone(),
        Sender::new(DEFAULT_URL.to_string(), API_KEY.to_string()),
    );
    let galaxy_exp = exps.get(&-1).unwrap().clone();
    let galaxy = interpreter.apply_cons(galaxy_exp);
    let mut state = Nil;
    println!("{}", galaxy);
    loop {
        println!("State: {}", state);
        let (new_state, boards) =
            interpreter.interact(galaxy.clone(), state.clone(), Expr::vector(0, 0));
        state = new_state;
        boards.iter().for_each(Board::draw);
    }
}
