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
        x3: i64,
        x4: i64,
        x5: i64,
    },
}

impl Command {
    pub fn accelerate(ship_id: i64, vector: (i64, i64)) -> Command {
        Command::Accelerate { ship_id, vector }
    }
    pub fn denotate(ship_id: i64) -> Command {
        Command::Denotate { ship_id }
    }
    pub fn shoot(ship_id: i64, target: (i64, i64)) -> Command {
        Command::Shoot {
            ship_id,
            target,
            x3: 1,
            x4: 0,
            x5: 4,
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
                x4,
                x5,
            } => {
                let (x, y) = target;
                Expr::from_vector(vec![
                    Int(2),
                    Int(ship_id),
                    Expr::vector(x, y),
                    Expr::from_vector(vec![Int(x3), Int(x4), Int(x5)]),
                ])
            }
        }
    }
}
