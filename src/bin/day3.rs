#[derive(Debug)]
struct Point2 {
    x: usize,
    y: usize,
}

impl Point2 {
    fn new(x: usize, y: usize) -> Self {
        Point2 { x, y }
    }
}

trait TobagganStrategy {
    fn get_new_location(&self, line_len: &usize) -> Point2;
}

#[derive(Debug)]
struct SimpleTobaggan<'a> {
    position: Point2,
    delta: Point2,
    slope: &'a str,
}

impl<'a> SimpleTobaggan<'a> {
    fn new(delta: Point2, slope: &'a str) -> Self {
        SimpleTobaggan {
            position: Point2::new(0, 0),
            delta,
            slope,
        }
    }

    fn get_location_type(&self, x: usize, y: usize, slope: &str) -> LocationType {
        if let Some(row) = slope.lines().nth(y as usize) {
            match row.chars().nth(x as usize) {
                Some(spot) => match spot {
                    '#' => return LocationType::Tree,
                    _ => return LocationType::Empty,
                },
                None => LocationType::NotALocation,
            }
        } else {
            return LocationType::NotALocation;
        }
    }

    fn ski_slope(&mut self) -> usize {
        let line_len = self.slope.lines().nth(0).unwrap().len();
        let mut cur_location = LocationType::Empty;
        let mut trees_hit = 0;
        while cur_location != LocationType::NotALocation {
            let new_pos = self.get_new_location(&line_len);
            cur_location =
                self.get_location_type(self.position.x, self.position.y, self.slope);
            match cur_location {
                LocationType::NotALocation => {}
                LocationType::Tree => {
                    trees_hit += 1;
                    self.position = new_pos;
                }
                LocationType::Empty => {
                    self.position = new_pos;
                }
            }
        }
        return trees_hit
    }
}

impl TobagganStrategy for SimpleTobaggan<'_> {
    fn get_new_location(&self, line_len: &usize) -> Point2 {
        let x = (self.position.x + self.delta.x) % line_len;
        return Point2::new(x, self.position.y + self.delta.y);
    }
}

#[derive(PartialEq, Debug)]
enum LocationType {
    Empty,
    Tree,
    NotALocation,
}

fn main() {
    let contents = include_str!("../../inputs/day3.txt");
    let mut tobaggan = SimpleTobaggan::new(Point2::new(3, 1), contents);
    let trees_hit = tobaggan.ski_slope();
    println!("Answer 1: {}", trees_hit);

    let tobaggans = [
        SimpleTobaggan::new(Point2::new(1, 1), contents),
        SimpleTobaggan::new(Point2::new(3, 1), contents),
        SimpleTobaggan::new(Point2::new(5, 1), contents),
        SimpleTobaggan::new(Point2::new(7, 1), contents),
        SimpleTobaggan::new(Point2::new(1, 2), contents),
    ];
    let mut trees_hit: usize = 1;
    for mut tobaggan in tobaggans {
        trees_hit *= tobaggan.ski_slope();
    }

    println!("Answer 2: {:?}", trees_hit);
}
