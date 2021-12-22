use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::ops::{Add, Deref, Neg, Sub};
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");
const MIN_COMMON_BEACONS: usize = 12;

lazy_static::lazy_static! {
    static ref ROTATIONS: HashSet<Rotation> = {
        // This is neded to reduce the size of the problem, because
        // we would expect 64 different rotations (4 * 4 * 4) but in
        // reality we would have less rotations (24)
        [0, 90, 180, 270].iter().flat_map(|x| {
            [0, 90, 180, 270].iter().flat_map(move |y| {
                [0, 90, 180, 270].iter().map(move |z| {
                    let rotation_degrees = Rotation(Point3D {x:*x,y:*y,z:*z});
                    let rotated_axes = (
                        Point3D{x: 1, y: 0, z: 0}.rotate(&rotation_degrees).clone(),
                        Point3D{x: 0, y: 1, z: 0}.rotate(&rotation_degrees).clone(),
                        Point3D{x: 0, y: 0, z: 1}.rotate(&rotation_degrees).clone(),
                    );
                    (rotated_axes, rotation_degrees)
                })
            })
        }).collect::<HashMap<_, _>>().values().cloned().collect()
    };
}

/// Retrieve all the unordered pair of values within a given set.
///
/// NOTES:
/// * The following pairs are considered as the same: `(A, B)` is equivalent to `(B, A)`
/// * Having values in a set ensures that the pair `(A, A)` is not possible
fn pair_of_values<T: PartialOrd>(values: &HashSet<T>) -> impl Iterator<Item = (&T, &T)> {
    values.iter().flat_map(move |value1| {
        values.iter().filter_map(move |value2| {
            if value1 > value2 {
                Some((value1, value2))
            } else {
                None
            }
        })
    })
}

// Considering the input we should need around 10 bits to represent the number
// Even assuming translations we should be able to make it work within 16 bits
// Doing this reduces the memory of each point from
// 24 bytes (with isize) to 6 bytes (with i16)
type PointInt = i16;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
struct Point3D {
    x: PointInt,
    y: PointInt,
    z: PointInt,
}

impl FromStr for Point3D {
    type Err = anyhow::Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = scan_fmt!(line, "{},{},{}", PointInt, PointInt, PointInt)?;
        Ok(Self { x, y, z })
    }
}

impl Add for &Point3D {
    type Output = Point3D;
    fn add(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Neg for &Point3D {
    type Output = Point3D;
    fn neg(self) -> Self::Output {
        Point3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for &Point3D {
    type Output = Point3D;
    fn sub(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Point3D {
    fn rotate_x_clockwise_by_90_degrees(&mut self) -> &mut Self {
        /*
        90 degrees rotation around x
            ⎡ 1 0  0 ⎤⎡ x ⎤   ⎡  x ⎤
            ⎢ 0 0 -1 ⎥⎢ y ⎥ = ⎢ -z ⎥
            ⎣ 0 1  0 ⎦⎣ z ⎦   ⎣  y ⎦
        */
        self.z *= -1;
        std::mem::swap(&mut self.y, &mut self.z);
        self
    }

    fn rotate_y_clockwise_by_90_degrees(&mut self) -> &mut Self {
        /*
        90 degrees rotation around y
            ⎡  0 0 1 ⎤⎡ x ⎤   ⎡  z ⎤
            ⎢  0 1 0 ⎥⎢ y ⎥ = ⎢  y ⎥
            ⎣ -1 0 0 ⎦⎣ z ⎦   ⎣ -x ⎦
        */
        self.x *= -1;
        std::mem::swap(&mut self.x, &mut self.z);
        self
    }

    fn rotate_z_clockwise_by_90_degrees(&mut self) -> &mut Self {
        /*
        90 degrees rotation around z
            ⎡ 0 -1 0 ⎤⎡ x ⎤   ⎡ -y ⎤
            ⎢ 1  0 0 ⎥⎢ y ⎥ = ⎢  x ⎥
            ⎣ 0  0 1 ⎦⎣ z ⎦   ⎣  z ⎦
        */
        self.y *= -1;
        std::mem::swap(&mut self.x, &mut self.y);
        self
    }

    fn rotate(&mut self, rotation_degrees: &Rotation) -> &mut Self {
        // Rotation rules in https://en.wikipedia.org/wiki/Rotation_matrix#In_three_dimensions

        debug_assert_eq!(rotation_degrees.x % 90, 0);
        debug_assert_eq!(rotation_degrees.y % 90, 0);
        debug_assert_eq!(rotation_degrees.z % 90, 0);

        for _ in 0..(rotation_degrees.x / 90) {
            self.rotate_x_clockwise_by_90_degrees();
        }

        for _ in 0..(rotation_degrees.y / 90) {
            self.rotate_y_clockwise_by_90_degrees();
        }

        for _ in 0..(rotation_degrees.z / 90) {
            self.rotate_z_clockwise_by_90_degrees();
        }

        self
    }

    fn manhattan_distance(&self, other: &Self) -> usize {
        let diff = self - other;
        diff.x.abs() as usize + diff.y.abs() as usize + diff.z.abs() as usize
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Rotation(Point3D);

impl Deref for Rotation {
    type Target = Point3D;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ScannerId(u8);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scanner {
    id: ScannerId,
    beacons: HashSet<Point3D>,
    point_distances: HashMap<(Point3D, Point3D), Point3D>,
    known_distances: HashSet<Point3D>,
}

impl TryFrom<&[String]> for Scanner {
    type Error = anyhow::Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        anyhow::ensure!(lines.len() > 1);
        let id = ScannerId(scan_fmt!(&lines[0], "--- scanner {} ---", u8)?);
        let beacons: HashSet<Point3D> = lines[1..]
            .iter()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self::new(id, beacons))
    }
}

impl Scanner {
    /// Add new beacons into the scanner
    ///
    /// The method is useful to avoid to fully recompute all the cached values
    fn add_beacons(&mut self, mut new_beacons: HashSet<Point3D>) {
        // Remove already known points
        new_beacons.retain(|beacon| !self.beacons.contains(beacon));

        // All the new pairs that are present due to the new beacons
        let new_pairs: HashSet<(Point3D, Point3D)> = pair_of_values(&new_beacons)
            .chain(self.beacons.iter().flat_map(|beacon1| {
                new_beacons.iter().filter_map(move |beacon2| {
                    if beacon1 > beacon2 {
                        Some((beacon1, beacon2))
                    } else {
                        None
                    }
                })
            }))
            .chain(new_beacons.iter().flat_map(|beacon1| {
                self.beacons.iter().filter_map(move |beacon2| {
                    if beacon1 > beacon2 {
                        Some((beacon1, beacon2))
                    } else {
                        None
                    }
                })
            }))
            .map(|(beacon1, beacon2)| (beacon1.clone(), beacon2.clone()))
            .collect();

        // Record new point distances and update known distances
        self.point_distances.extend(
            new_pairs
                .iter()
                .map(|(beacon1, beacon2)| ((beacon1.clone(), beacon2.clone()), beacon1 - beacon2)),
        );
        self.known_distances
            .extend(new_pairs.iter().map(|(beacon1, beacon2)| beacon1 - beacon2));

        // Add the new beacons into the known beacons
        self.beacons.extend(new_beacons);
    }

    fn new(id: ScannerId, beacons: HashSet<Point3D>) -> Self {
        let mut result = Self {
            id,
            beacons: HashSet::with_capacity(0),
            point_distances: HashMap::with_capacity(0),
            known_distances: HashSet::with_capacity(0),
        };
        result.add_beacons(beacons);
        result
    }

    /// Rotate the beacons according to `rotation`.
    ///
    /// The method will return a new Scanner instance
    fn rotate(&self, rotation: &Rotation) -> Self {
        Self::new(
            self.id,
            self.beacons
                .iter()
                .map(|beacon| {
                    let mut cloned_beacon = beacon.clone();
                    cloned_beacon.rotate(rotation);
                    cloned_beacon
                })
                .collect(),
        )
    }

    /// Try to merge `scanner` into `self` with the assumption that
    /// points in `scanner` might be translated.
    ///
    /// The method will then be responsible for:
    /// * ensuring that at least `min_common_beacons` beacons are in common
    /// * translating `scanner`'s beacons to properly overlap with `self`
    /// * add the translated `scanner`'s beacons into `self`
    /// * returning a boolean value representing the success or failure of the merge
    fn try_merge(&mut self, scanner: &Self, min_common_beacons: usize) -> Option<Point3D> {
        debug_assert!(min_common_beacons > 0);

        let points_with_matching_distances: HashSet<(&Point3D, &Point3D)> = scanner
            .point_distances
            .iter()
            .filter_map(|((beacon1, beacon2), distance)| {
                if self.known_distances.contains(distance) {
                    Some((beacon1, beacon2))
                } else {
                    None
                }
            })
            .take(min_common_beacons)
            .collect();

        if points_with_matching_distances.len() >= min_common_beacons {
            let beacons_with_known_distance_pair: (Point3D, Point3D) = points_with_matching_distances.iter().next().map(|(beacon1, beacon2)| {
                let beacon1 : &Point3D = beacon1;
                let beacon2 : &Point3D = beacon2;
                (beacon1.clone(), beacon2.clone())
            }).expect("At least one pair of beacons is present as min_common_beacons is greather than 0");
            let known_distance = scanner
                .point_distances
                .get(&beacons_with_known_distance_pair)
                .expect("The pair of points exists, hence it is already evaluated");
            let reference_beacons_with_known_distance = self
                .point_distances
                .iter()
                .find_map(|(beacons_with_same_distance, self_distance)| {
                    if self_distance == known_distance {
                        Some(beacons_with_same_distance)
                    } else {
                        None
                    }
                })
                .expect(
                    "By construction the distance we're looking for exists on the current instance",
                );
            let offset =
                &beacons_with_known_distance_pair.0 - &reference_beacons_with_known_distance.0;
            self.add_beacons(
                scanner
                    .beacons
                    .iter()
                    .map(|beacon| beacon - &offset)
                    .collect(),
            );

            Some(offset)
        } else {
            None
        }
    }

    /// Return the number of visible beacons in the scanner
    fn visible_beacons(&self) -> usize {
        self.beacons.len()
    }
}

#[derive(Debug)]
struct Input {
    scanner_id_to_rotation_to_scanner: HashMap<ScannerId, HashMap<Rotation, Scanner>>,
}

impl TryFrom<Vec<String>> for Input {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let read_scanners: Vec<Scanner> = std::iter::once(0)
            .chain(lines.iter().enumerate().filter_map(|(line_number, line)| {
                if line.is_empty() {
                    Some(line_number)
                } else {
                    None
                }
            }))
            .chain(std::iter::once(lines.len()))
            .collect::<Vec<_>>()
            .windows(2)
            .map(|pair| {
                debug_assert_eq!(pair.len(), 2);
                let first_line = if pair[0] == 0 { 0 } else { pair[0] + 1 };
                let last_line = pair[1] - 1;
                first_line..=last_line
            })
            .map(|range| Scanner::try_from(&lines[range]))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            scanner_id_to_rotation_to_scanner: read_scanners
                .iter()
                .map(|scanner| {
                    (
                        scanner.id,
                        ROTATIONS
                            .iter()
                            .map(|rotation| (rotation.clone(), scanner.rotate(rotation)))
                            .collect(),
                    )
                })
                .collect(),
        })
    }
}

impl Input {
    fn try_merge_all_scanners(&self) -> Option<(Scanner, HashMap<ScannerId, Point3D>)> {
        let mut reference_scanner: Scanner = if let Some(scanner) = self
            .scanner_id_to_rotation_to_scanner
            .values()
            .find_map(|rotation_to_scanner| rotation_to_scanner.values().next())
        {
            scanner.clone()
        } else {
            return None;
        };

        let mut scanner_ids_to_merge: HashSet<_> =
            self.scanner_id_to_rotation_to_scanner.keys().collect();
        scanner_ids_to_merge.remove(&reference_scanner.id);

        let mut scanner_id_to_offset: HashMap<ScannerId, Point3D> = HashMap::new();

        loop {
            let maybe_merged_scanner_and_offset: Option<(ScannerId, Point3D)> =
                scanner_ids_to_merge.iter().find_map(|scanner_id| {
                    if let Some(rotation_to_scanners) =
                        self.scanner_id_to_rotation_to_scanner.get(scanner_id)
                    {
                        rotation_to_scanners.values().find_map(|scanner| {
                            reference_scanner
                                .try_merge(scanner, MIN_COMMON_BEACONS)
                                .map(|offset| (scanner.id, offset))
                        })
                    } else {
                        None
                    }
                });

            if let Some((merged_scanner_id, offset)) = maybe_merged_scanner_and_offset {
                scanner_ids_to_merge.remove(&merged_scanner_id);
                scanner_id_to_offset.insert(merged_scanner_id, offset);
            } else {
                break;
            }
        }

        Some((reference_scanner, scanner_id_to_offset))
    }
}

fn part01(input: &Input) -> usize {
    input
        .try_merge_all_scanners()
        .map_or(0, |(merged_scanner, _)| merged_scanner.visible_beacons())
}

fn part02(input: &Input) -> usize {
    input
        .try_merge_all_scanners()
        .map_or(0, |(_, scanner_id_to_offset)| {
            let offsets: HashSet<Point3D> = scanner_id_to_offset.values().cloned().collect();
            pair_of_values(&offsets)
                .map(|(offset1, offset2)| offset1.manhattan_distance(offset2))
                .max()
                .unwrap_or(0)
        })
}

fn main() -> anyhow::Result<()> {
    let input = Input::try_from(input_lines(INPUT)?)?;

    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));

    Ok(())
}
