use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("File read error");
    let mut paths = input.split("\n");
    let path_1 = path_to_intermediate_points(paths.next().unwrap());
    let path_2 = path_to_intermediate_points(paths.next().unwrap());



     let crossing = closest_crossing(path_1, path_2);
     println!("The closest crossing distance: {}", crossing);
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64
}

fn closest_crossing(path_1: Vec<Point>, path_2: Vec<Point>) -> i64 {
    let origin = Point{x: 0, y: 0};
    let mut min_dist: i64 = std::i64::MAX;

    println!("{}, {}", path_1.len(), path_2.len());
    for line1 in path_1.windows(2) {
        for line2 in path_2.windows(2) {
            match check_intersect(&line1[0], &line1[1], &line2[0], &line2[1]) {
                Some(dist) => {
                    if dist < min_dist {
                        min_dist = dist
                    }
                },
                None => (),
            }
        }
    }
    min_dist
}

fn closest_crossing_naive(path_1: Vec<Point>, path_2: Vec<Point>) -> i64 {
    let origin = Point{x: 0, y: 0};
    let mut min_dist: i64 = std::i64::MAX;

    println!("{}, {}", path_1.len(), path_2.len());
    for p1 in path_1.iter() {
        for p2 in path_2.iter() {
            if p1 == p2 {
                println!("Equal");
                let dist = distance(p1, &origin);
                if dist < min_dist {
                    println!("New min dist: {}", dist);
                    min_dist = dist;
                }
            }
        }
    }
    min_dist
}

fn path_to_intermediate_points(path: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let mut ptr = Point{x: 0, y: 0};
    for inst in path.split(",") {
        let (dir, len) = inst.split_at(1);
        let len = len.parse::<usize>().unwrap();
        let dif = match dir {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => panic!("Unexpected direction!"),
        };
        ptr.x += (len as i64) * dif.0;
        ptr.y += (len as i64) * dif.1;
        points.push(ptr);
    }
    points
}

fn check_intersect(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Option<i64> {
    let origin = Point{x: 0, y: 0};
    if p1.x <= p3.x && p2.x >= p4.x || p1.x >= p3.x && p2.x <= p4.x {
        if p1.y <= p3.y && p2.y >= p4.y || p1.y >= p3.y && p2.y <= p4.y {
            if p1.y == p2.y && p3.x == p4.x {
                return Some(distance(&origin, &Point{x: p3.x, y: p4.y}));
            }
            else if p1.x == p2.x && p3.y == p4.y {
                return Some(distance(&origin, &Point{x: p1.x, y: p3.y}));
            }
            //println!("Found intersection: {:?}, {:?}, {:?}, {:?}", p1, p2, p3, p4);
            return Some(0)
        }
    }
    None
}

fn path_to_points(path: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let mut ptr = Point{x: 0, y: 0};
    for inst in path.split(",") {
        let (dir, len) = inst.split_at(1);
        let len = len.parse::<usize>().unwrap();
        let dif = match dir {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => panic!("Unexpected direction!"),
        };
        for _ in 1..len+1 {
            ptr.x += dif.0;
            ptr.y += dif.1;
            points.push(ptr);
        }
    }
    println!("done");
    points
}

fn distance(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Point{x: -2, y: 4};
        let p2 = Point{x: 10, y: -3};

        assert_eq!(distance(&p1, &p2), 19);
    }

    #[test]
    fn test_crossing() {
        let path_1 = path_to_points("R8,U5,L5,D3");
        let path_2 = path_to_points("U7,R6,D4,L4");

        assert_eq!(closest_crossing_naive(path_1, path_2), 6); 
    }

    #[test]
    fn test_crossing_equivalency() {
        let path_1 = path_to_points("R8,U5,L5,D3");
        let path_2 = path_to_points("U7,R6,D4,L4");

        assert_eq!(closest_crossing_naive(path_1.clone(), path_2.clone()), closest_crossing(path_1, path_2)); 
    }

    #[test]
    fn test_intersect_neg() {
        let p1 = Point{x:1, y:1};
        let p2 = Point{x:1, y:20};
        let p3 = Point{x:-10, y:30};
        let p4 = Point{x:10, y:30};

        assert_eq!(check_intersect(&p1, &p2, &p3, &p4), None); 
    }

    #[test]
    fn test_intersect_pos() {
        let p1 = Point{x:1, y:1};
        let p2 = Point{x:1, y:20};
        let p3 = Point{x:-10, y:10};
        let p4 = Point{x:10, y:10};

        assert_ne!(check_intersect(&p1, &p2, &p3, &p4), None); 
    }

    #[test]
    fn test_intersect_pos_edge() {
        let p1 = Point{x:1, y:1};
        let p2 = Point{x:1, y:10};
        let p3 = Point{x:1, y:10};
        let p4 = Point{x:10, y:10};

        assert_ne!(check_intersect(&p1, &p2, &p3, &p4), None); 
    }
}