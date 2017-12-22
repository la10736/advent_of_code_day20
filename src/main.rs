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

    let particles = content
        .lines()
        .map(|l| l.parse::<Particle>().unwrap()).collect::<Vec<_>>();

    println!("Len = {}", particles.len());

    let collide_count = particles
        .iter().take(particles.len()-1).filter(
        |&p0|
                    particles.iter()
                        .filter(|p1| p1 != &p0)
                        .filter(|p1| p0.collide(p1).len()>0)
                        .nth(0)
                        .is_some()
    ).count();

    println!("Collidet = {}", collide_count);
    println!("Result = {}", particles.len() - collide_count);
}

type CoordType = i32;

#[derive(Eq, PartialEq, Debug, Clone)]
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

impl<'a> std::ops::AddAssign<&'a Coord> for Coord {
    fn add_assign(&mut self, rhs: &'a Coord) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<'a> std::ops::SubAssign<&'a Coord> for Coord {
    fn sub_assign(&mut self, rhs: &'a Coord) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
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
        Ok(Coord(values[0], values[1], values[2]))
    }
    type Err = String;
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord, Clone)]
struct Particle {
    a: Coord,
    v: Coord,
    p: Coord,
}

fn isqrt(n: i32) -> Option<i32> {
    if n < 0 {
        return None;
    }
    let candidate = (n as f64).sqrt() as i32;

    if n == candidate * candidate {
        Some(candidate)
    } else {
        None
    }
}

fn is_zero_crossing(t:i32, p: i32, v: i32, a: i32) -> bool {
    //     p(t) = p0 + tv0 + a0 ((t+1) * t)/2

    p + t * v + a *(t * (t+1))/2 == 0

}

fn resolve_zero_crossing(p: i32, v: i32, a: i32) -> Vec<i32> {
    if a == 0{
        return if v == 0 {
            if p == 0 {
                vec![0]
            } else {
                vec![]
            }
        } else {
            if p % v == 0 {
                vec![-p/v]
            } else {
                vec![]
            }
        }
    }
    // Trovo il candidato risolvendo l'equazione per la prima coordinata
    // A = a0
    // B = 2v0 + a0
    // C = 2p0
    let (a, b, c) = (a, 2*v + a, 2*p);

    let det = b * b - (4 * a * c);

    let det = match isqrt(det) {
        Some(d) => d,
        None => return vec![]
    };

    [(-b + det) / ( 2 * a), (-b - det) / ( 2 * a)].into_iter().
        filter(|&r| r >= &0 ).map(|r| r.clone()).collect()
}

impl Particle {
    fn collide(&self, other: &Particle) -> Vec<i32> {
        let mut equiv = self.clone();
        equiv.p -= &other.p;
        equiv.v -= &other.v;
        equiv.a -= &other.a;

        let (a, v, p) = (equiv.a.0, equiv.v.0, equiv.p.0);

        resolve_zero_crossing(p, v, a).into_iter().filter(
            |&t|
            is_zero_crossing(t, equiv.p.1, equiv.v.1, equiv.a.1)
        ).filter(
            |&t|
                is_zero_crossing(t, equiv.p.2, equiv.v.2, equiv.a.2)
        ).collect()
    }

    fn evolve(mut self) -> Self {
        self.v += &self.a;
        self.p += &self.v;
        self
    }

    fn evolve_back(mut self) -> Self {
        self.p -= &self.v;
        self.v -= &self.a;
        self
    }
}

impl std::str::FromStr for Particle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sub_parts = s[3..]
            .split(|c| "<>".contains(c))
            .filter(|token| token.len() > 0 && "-0123456789".contains(&token[0..1]))
            .collect::<Vec<_>>();
        Ok(Particle {
            p: sub_parts[0].parse()?,
            v: sub_parts[1].parse()?,
            a: sub_parts[2].parse()?,
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
            p: Coord(1791, 622, -2528),
            v: Coord(258, 87, -359),
            a: Coord(-17, -1, 24),
        }, line.parse().unwrap());
    }

    #[test]
    fn distance() {
        let particle: Particle = "p=<1791,622,-2528>, v=<258,87,-359>, a=<-17,-1,24>"
            .parse().unwrap();

        assert_eq!(42, particle.a.distance())
    }

    fn p(p0: CoordType, p1: CoordType, p2: CoordType, v0: CoordType, v1: CoordType, v2: CoordType, a0: CoordType, a1: CoordType, a2: CoordType) -> Particle {
        Particle {
            p: Coord(p0, p1, p2),
            v: Coord(v0, v1, v2),
            a: Coord(a0, a1, a2),
        }
    }

    #[test]
    fn compare_particle() {
        assert!(p(0, 0, 0, 0, 0, 0, 0, 0, 0) <
            p(0, 0, 0, 0, 0, 0, 1, 0, 0));

        assert!(p(0, 0, 0, 0, 0, 0, 0, 0, 0) <
            p(0, 0, 0, 0, 0, 0, -11, 0, 0));

        assert!(p(1, 0, 0, 0, 0, 0, 0, 0, 0) <
            p(0, 0, 0, 0, 0, 0, -11, 0, 0));

        assert!(p(0, 0, 0, 0, 1, 5, 0, 2, 1) <
            p(0, 0, 0, 0, 7, 0, -1, 0, 2));

        assert!(p(0, 0, 0, 0, 1, 5, 0, 2, 1) <
            p(0, 0, 1, 0, 6, 0, -1, 0, 2));
    }

    #[test]
    fn same_position_particle_should_collide_at_0() {
        let p0 = p(1,2,3, 5,22, -1, 2, -1, 0);
        let p1 = p(1,2,3, 0,1, 0, 0, 0, 0);

        assert_eq!(vec![0], p0.collide(&p1))
    }

    #[test]
    fn evolve() {
        let p0 = p(1, 2, 3, 5, 22, -1, 2, -1, 0).evolve();

        assert_eq!(p(8, 23, 2, 7, 21, -1, 2, -1, 0), p0);
    }

    #[test]
    fn evolve_back() {
        let p0 = p(1, 2, 3, 5, 22, -1, 2, -1, 0).evolve().evolve_back();

        assert_eq!(p(1, 2, 3, 5, 22, -1, 2, -1, 0), p0);
    }

    #[test]
    fn evolve_dance() {
        let p0 = p(1, 2, 3, 5, 22, -1, 2, -1, 0)
            .evolve()
            .evolve()
            .evolve()
            .evolve_back()
            .evolve()
            .evolve_back()
            .evolve_back()
            .evolve()
            .evolve_back()
            .evolve_back();

        assert_eq!(p(1, 2, 3, 5, 22, -1, 2, -1, 0), p0);
    }

    #[test]
    fn should_collide_after_some_time() {

        let p0 = p(1, 2, 3, 2, -1, 5, 1, -1, 2)
            .evolve_back()
            .evolve_back()
            .evolve_back()
            .evolve_back();
        let p1 = p(1, 2, 3, 5, 22, -1, 2, -1, 0)
            .evolve_back()
            .evolve_back()
            .evolve_back()
            .evolve_back();

        assert_eq!(vec!(4), p0.collide(&p1));

    }

    #[test]
    fn should_not_collide_after_some_time() {

        let p0 = p(1, 2, 3, 2, -1, 5, 1, -1, 2)
            .evolve_back()
            .evolve_back()
            .evolve_back()
            .evolve_back();
        let p_stamp = p(1, 2, 3, 5, 22, -1, 2, -1, 0)
            .evolve_back()
            .evolve_back()
            .evolve_back()
            .evolve_back();

        let mut p1 = p_stamp.clone();
        p1.v.1 = 3;

        let mut p2 = p_stamp.clone();
        p2.v.2 = -64;

        let mut p3 = p_stamp.clone();
        p3.a.1 = 2;

        let mut p4 = p_stamp.clone();
        p4.a.2 = -12;

        assert_eq!(Vec::<i32>::new(), p0.collide(&p1));
        assert_eq!(Vec::<i32>::new(), p0.collide(&p2));
        assert_eq!(Vec::<i32>::new(), p0.collide(&p3));
        assert_eq!(Vec::<i32>::new(), p0.collide(&p4));

    }

    // ```
    // p = p0 + tv + a(t^2 /2 )
    // v(t) = v0 + t*a0
    // p(t) = p(t-1) + v0 + t*a0 =
    //        p(t-2) + 2v0 + [t + (t-1)]a0 =
    //        p0 + tv0 + a0 * Sum(i:1..t) i =
    //        p0 + tv0 + a0 ((t+1) * t)/2 =
    //        p0 + (v0 + a0/2) t + a0/2 t^2
    //
    // A = a0/2
    // B = (v0 + a0/2)
    // C = p0
    //
    //
    // p(2) = 3 + 2 * 2  + ( -1 (2 * 3)/2 )
    // p(3) = 3 + 2 * 3  + ( -1 (3 * 4)/2 ) = 3 + (2 - 1/2) * 3 + 1/2 * 9
    // ```
}
