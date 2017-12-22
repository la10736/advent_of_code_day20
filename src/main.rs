use std::io::prelude::*;

fn read_all<S: AsRef<std::path::Path>>(path: S) -> String {
    let mut content = String::new();
    let mut f = std::fs::File::open(path).unwrap();
    f.read_to_string(&mut content).unwrap();
    content
}

fn main() {
    let fname = std::env::args().nth(1).unwrap_or(String::from("example"));
    let content = read_all(fname);

    let (pos, particle) = content
        .lines()
        .map(|l| l.parse::<Particle>().unwrap())
        .enumerate()
        .min_by(|&(_, ref p0), &(_, ref p1)|
            p0.cmp(p1)).unwrap();

    println!("[{}, {:?}]", pos, particle);
}

type CoordType = i32;

#[derive(Eq, PartialEq, Debug)]
struct Coord(CoordType, CoordType, CoordType);

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Coord) -> Option<std::cmp::Ordering> {
        self.distance().partial_cmp(&other.distance())
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance().cmp(&other.distance())
    }
}

impl Coord {
    fn distance(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl std::str::FromStr for Coord {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.splitn(3, ',').map(
            |token| token.parse().unwrap()
        ).collect::<Vec<CoordType>>();
        Ok(Coord(values[0],values[1],values[2]))
    }
    type Err = String;
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
struct Particle {
    a: Coord,
    v: Coord,
    p: Coord,
}

impl std::str::FromStr for Particle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sub_parts = s[3..]
            .split(|c| "<>".contains(c))
            .filter(|token| token.len()>0 && "-0123456789".contains(&token[0..1]))
            .collect::<Vec<_>>();
        Ok(Particle {
            p: sub_parts[0].parse()?,
            v: sub_parts[1].parse()?,
            a: sub_parts[2].parse()?
        }
        )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line() {
        let line = "p=<1791,622,-2528>, v=<258,87,-359>, a=<-17,-1,24>";

        assert_eq!(Particle {
            p: Coord(1791,622,-2528),
            v: Coord(258,87,-359),
            a: Coord(-17,-1,24)
        }, line.parse().unwrap());
    }

    #[test]
    fn distance() {
        let particle: Particle = "p=<1791,622,-2528>, v=<258,87,-359>, a=<-17,-1,24>"
            .parse().unwrap();

        assert_eq!(42, particle.a.distance())
    }

    #[test]
    fn compare_particle() {
        fn p(p0: CoordType,p1: CoordType,p2: CoordType,v0: CoordType,v1: CoordType,v2: CoordType,a0: CoordType,a1: CoordType,a2: CoordType) -> Particle {
            Particle {
                p: Coord(p0, p1, p2),
                v: Coord(v0, v1, v2),
                a: Coord(a0, a1, a2),
            }
        }

        assert!(p(0,0,0,0,0,0,0,0,0) <
                p(0,0,0,0,0,0,1,0,0) );

        assert!(p(0,0,0,0,0,0,0,0,0) <
                p(0,0,0,0,0,0,-11,0,0) );

        assert!(p(1,0,0,0,0,0,0,0,0) <
                p(0,0,0,0,0,0,-11,0,0) );

        assert!(p(0,0,0,0,1,5,0,2,1) <
            p(0,0,0,0,7,0,-1,0,2) );

        assert!(p(0,0,0,0,1,5,0,2,1) <
            p(0,0,1,0,6,0,-1,0,2) );
    }


}
