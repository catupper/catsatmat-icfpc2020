use crate::{
    Command,
    Expr::{self, *},
};

#[derive(Debug, Clone)]
pub struct State {
    pub turn: i64,
    pub x0: Vec<i64>,
    pub ships: Vec<Ship>,
}

impl From<Expr> for State {
    fn from(expr: Expr) -> State {
        if let (Int(turn), expr) = expr.carcdr() {
            let (x0, expr) = expr.carcdr();
            let (ships, _) = expr.carcdr();
            return State {
                turn,
                x0: x0
                    .iter()
                    .map(|x| match x {
                        Int(x) => x,
                        _ => panic!(),
                    })
                    .collect(),
                ships: ships.iter().map(|x| -> Ship { x.into() }).collect(),
            };
        }

        panic!("Failed to parse state, {}", expr)
    }
}

#[derive(Debug, Clone)]
pub struct Ship {
    pub role: i64,
    pub ship_id: i64,
    pub position: (i64, i64),
    pub velocity: (i64, i64),
    pub commands: Vec<Command>,
}

impl From<Expr> for Ship {
    fn from(expr: Expr) -> Ship {
        let (expr, commands) = expr.carcdr();
        if let (Int(role), expr) = expr.carcdr() {
            if let (Int(ship_id), expr) = expr.carcdr() {
                let (position, expr) = expr.carcdr();

                let (velocity, _) = expr.carcdr();

                if let (Int(px), Int(py)) = position.carcdr() {
                    if let (Int(vx), Int(vy)) = velocity.carcdr() {
                        println!("Commands {}", commands);
                        return Ship {
                            role,
                            ship_id,
                            position: (px, py),
                            velocity: (vx, vy),
                            commands: commands.iter().map(Into::into).collect(),
                        };
                    }
                }
            }
        }
        panic!("Failed to parse Ship, {}", expr)
    }
}
