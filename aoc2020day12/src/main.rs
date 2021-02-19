use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Move(Direction, isize),
    Rotate(AngleDelta),
    Forward(isize),
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| {
        let command = line.as_bytes()[0];
        let number: isize = (&line[1..]).parse().unwrap();

        match command {
            b'N' => Instruction::Move(Direction::North, number),
            b'S' => Instruction::Move(Direction::South, number),
            b'E' => Instruction::Move(Direction::East, number),
            b'W' => Instruction::Move(Direction::West, number),
            b'L' => Instruction::Rotate(AngleDelta(-number / 90)),
            b'R' => Instruction::Rotate(AngleDelta(number / 90)),
            b'F' => Instruction::Forward(number),
            c => panic!("unknown instruction {}", c as char),
        }
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Into<isize> for Direction {
    fn into(self) -> isize {
        self as isize
    }
}

impl std::convert::TryFrom<isize> for Direction {
    type Error = &'static str;
    
    fn try_from(val: isize) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Direction::North),
            1 => Ok(Direction::East),
            2 => Ok(Direction::South),
            3 => Ok(Direction::West),
            _ => Err("Direction: Out of bounds"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct AngleDelta(isize);

impl std::ops::Add<AngleDelta> for Direction {
    type Output = Self;

    fn add(self, rhs: AngleDelta) -> Self::Output {
        use std::convert::TryInto;

        let angle: isize = self.into();
        (angle + rhs.0).rem_euclid(4).try_into().unwrap()
    }
}

impl Direction {
    fn gps(self) -> GPS {
        match self {
            Direction::North => GPS {x: 1, y: 0},
            Direction::East => GPS {x: 0, y: 1},
            Direction::South => GPS {x: -1, y: 0},
            Direction::West => GPS {x: 0, y: -1},
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GPS {
    x: isize,
    y: isize,
}

impl std::ops::Add for GPS {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for GPS {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<isize> for GPS {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl GPS {
    fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ShipState {
    pos: GPS,
    dir: Direction,
}

impl std::ops::Add<Instruction> for ShipState {
    type Output = Self;
    fn add(self, rhs: Instruction) -> Self::Output {
        match rhs {
            Instruction::Move(dir, units) => Self {
                pos: self.pos + dir.gps() * units,
                ..self
            },
            Instruction::Rotate(delta) => Self {
                dir: self.dir + delta,
                ..self
            },
            Instruction::Forward(units) => Self {
                pos: self.pos + self.dir.gps() * units,
                ..self
            },
        }
    }
}

fn main() {
    let start = ShipState {
        dir: Direction::East,
        pos: GPS { x: 0, y: 0 },
    };
    let filename = "input";
    let file_contents = fs::read_to_string(filename).expect("failed to read");
    let end = parse_instructions(&file_contents).fold(start, |state, ins| state + ins);
    dbg!(start, end, (end.pos - start.pos).manhattan());
}

#[test]
fn gps_add() {
    let a = GPS { x: 10, y: 9 };
    let b = GPS { x: 7, y: 6 };
    assert_eq!(a+b, GPS { x: 17, y: 15 });
}

#[test]
fn gps_manhattan() {
    let start = GPS { x: 0, y: 0 };
    let end = GPS { x: 18, y: -5 };
    assert_eq!((end - start).manhattan(), 23);
}

#[test]
fn direction_try_from() {
    use std::convert::TryFrom;

    assert_eq!(
        <Direction as TryFrom<isize>>::try_from(0).unwrap(),
        Direction::North
        );
    assert_eq!(
        <Direction as TryFrom<isize>>::try_from(2).unwrap(),
        Direction::South
        );
    assert!(<Direction as TryFrom<isize>>::try_from(-1).is_err(),);
    assert!(<Direction as TryFrom<isize>>::try_from(4).is_err(),);
}

#[test]
fn test_direction_add() {
    assert_eq!(Direction::East + AngleDelta(1), Direction::South);
    assert_eq!(Direction::East + AngleDelta(-1), Direction::North);
    assert_eq!(Direction::East + AngleDelta(4), Direction::East);
}
