use std::{cmp::Ordering, fmt::Debug};

use crate::models::Place;
pub trait Convex<K>: Ord + Clone + Debug
where
    K: PartialOrd,
{
    fn get_angle(a: &Self, b: &Self) -> K;
    fn get_turn(a: &Self, b: &Self, c: &Self) -> Turn;
}

#[derive(PartialEq, PartialOrd)]
pub enum Turn {
    Clockwise,
    CounterClockwise,
}

impl Convex<f64> for Place {
    fn get_angle(a: &Self, b: &Self) -> f64 {
        let dx = a.eastings - b.eastings;
        let dy = a.northings - b.northings;

        let dist = (dx * dx + dy * dy).sqrt();

        dist / 1000.0
    }

    fn get_turn(a: &Self, b: &Self, c: &Self) -> Turn {
        let crossprod = (b.eastings - a.eastings) * (c.northings - b.northings)
            - (b.northings - a.northings) * (c.eastings - b.eastings);
        if crossprod < 0.0 {
            return Turn::Clockwise;
        }
        Turn::CounterClockwise
    }
}
impl PartialOrd for Place {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order = self.northings.partial_cmp(&other.northings).unwrap();

        if order != Ordering::Equal {
            Some(order)
        } else {
            Some(self.eastings.partial_cmp(&other.eastings).unwrap())
        }
    }
}

impl Eq for Place {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Place {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn convex_hull<T, K>(source: &[T]) -> Vec<T>
where
    T: Convex<K>,
    K: PartialOrd,
{
    if source.len() <= 3 {
        panic!("Cannot calculate convex hull of collection with less than 4 items")
    }

    let min = source.iter().min().unwrap();

    let sorted = sort_by_angle(source, min);

    let mut stack = vec![sorted[0].clone(), sorted[1].clone()];

    for p in sorted.iter().skip(2) {
        while stack.len() > 1
            && Turn::Clockwise
                == Convex::get_turn(&stack[stack.len() - 2], &stack[stack.len() - 1], p)
        {
            stack.pop();
        }
        stack.push(p.clone());
    }

    stack
}

fn sort_by_angle<T, K>(source: &[T], corner: &T) -> Vec<T>
where
    T: Convex<K>,
    K: PartialOrd,
{
    let mut angles: Vec<MinScored<K, T>> = source
        .iter()
        .map(|p| {
            let angle = Convex::get_angle(corner, p);

            MinScored {
                key: angle,
                value: p.clone(),
            }
        })
        .collect();

    //Sort values by angle to corner.
    angles.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
    return angles.iter().map(|ms| ms.value.clone()).collect();
}

#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>
where
    T: Convex<K>,
    K: PartialOrd,
{
    pub key: K,
    pub value: T,
}
