use crate::Expr::{self, *};

#[derive(Debug, Clone)]
pub enum Command {
    Accelerate {
        ship_id: i64,
        vector: (i64, i64),
    },
    Denotate {
        ship_id: i64,
    },
    Shoot {
        ship_id: i64,
        target: (i64, i64),
        x3: Expr,
    },
}

impl Command {
    pub fn accelerate(ship_id: i64, vector: (i64, i64)) -> Command {
        Command::Accelerate { ship_id, vector }
    }
    pub fn is_accelerate(&self) -> bool {
        if let Command::Accelerate {
            ship_id: _,
            vector: _,
        } = self
        {
            true
        } else {
            false
        }
    }

    pub fn denotate(ship_id: i64) -> Command {
        Command::Denotate { ship_id }
    }
    pub fn is_denotate(&self) -> bool {
        if let Command::Denotate { ship_id: _ } = self {
            true
        } else {
            false
        }
    }
    pub fn shoot(ship_id: i64, target: (i64, i64)) -> Command {
        Command::Shoot {
            ship_id,
            target,
            x3: Nil,
        }
    }
    pub fn is_shoot(&self) -> bool {
        if let Command::Shoot {
            ship_id: _,
            target: _,
            x3: _,
        } = self
        {
            true
        } else {
            false
        }
    }
}

impl From<Command> for Expr {
    fn from(command: Command) -> Self {
        match command {
            Command::Accelerate { ship_id, vector } => {
                let (x, y) = vector;
                Expr::from_vector(vec![Int(0), Int(ship_id), Expr::vector(x, y)])
            }
            Command::Denotate { ship_id } => Expr::from_vector(vec![Int(1), Int(ship_id)]),
            Command::Shoot {
                ship_id,
                target,
                x3,
            } => {
                let (x, y) = target;
                Expr::from_vector(vec![Int(2), Int(ship_id), Expr::vector(x, y), x3])
            }
        }
    }
}

impl From<Expr> for Command {
    fn from(expr: Expr) -> Self {
        if let (Int(command_num), expr) = expr.carcdr() {
            if let (Int(ship_id), expr) = expr.carcdr() {
                if command_num == 1 {
                    return Command::Denotate { ship_id };
                }
                let (vector, expr) = expr.carcdr();
                if let (Int(x), Int(y)) = vector.carcdr() {
                    if command_num == 0 {
                        return Command::Accelerate {
                            ship_id,
                            vector: (x, y),
                        };
                    }
                    let x3 = expr.car();
                    return Command::Shoot {
                        ship_id,
                        target: (x, y),
                        x3,
                    };
                }
            }
        }
        panic!();
    }
}
