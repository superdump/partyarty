use rand::{Rng, thread_rng};
use specs::Entity;
use specs::prelude::*;

use aabb::{AABB, hit_aabb, surrounding_box};
use components::Position;
use hitable::{hit, Hitable, HitRecord};
use ray::Ray;

use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Default)]
pub struct BVHNode {
    pub aabb: AABB,
    pub entity: Option<Entity>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

pub fn bvh_node(aabb: AABB, entity: Option<Entity>) -> BVHNode {
    BVHNode::new(aabb, entity)
}

impl BVHNode {
    pub fn new(aabb: AABB, entity: Option<Entity>) -> BVHNode {
        BVHNode {
            aabb,
            entity: entity,
            left: None,
            right: None,
        }
    }
}

#[derive(Default)]
pub struct BVHTree {
    pub nodes: Vec<BVHNode>,
}

impl fmt::Display for BVHTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut q = VecDeque::new();
        q.push_back((self.nodes.len() - 1, self.nodes[self.nodes.len() - 1].clone()));
        while !q.is_empty() {
            let front = q.pop_front().unwrap();
            writeln!(
                f, "{} : {} - left: {}, right: {}",
                front.0,
                front.1.aabb,
                front.1.left.unwrap_or(12345),
                front.1.right.unwrap_or(12345),
            )?;
            if let Some(left) = front.1.left {
                q.push_back((left, self.nodes[left].clone()));
            }
            if let Some(right) = front.1.right {
                q.push_back((right, self.nodes[right].clone()));
            }
        }
        Ok(())
    }
}

pub fn build_bvh(
    tree: &mut BVHTree,
    base: usize,
    n: usize,
    t0: f32,
    t1: f32
) -> Option<usize> {
    match thread_rng().gen_range(0, 3) {
        0 => {
            tree.nodes[base..(base + n)]
                .sort_unstable_by(|a, b| a.aabb.min.x.partial_cmp(&b.aabb.min.x).unwrap());
        },
        1 => {
            tree.nodes[base..(base + n)]
                .sort_unstable_by(|a, b| a.aabb.min.y.partial_cmp(&b.aabb.min.y).unwrap());
        },
        _ => {
            tree.nodes[base..(base + n)]
                .sort_unstable_by(|a, b| a.aabb.min.z.partial_cmp(&b.aabb.min.z).unwrap());
        },
    }
    let mut node = BVHNode::default();
    match n {
        1 => {
            node.left = Some(base);
            node.right = node.left;
        },
        2 => {
            node.left = Some(base);
            node.right = Some(base + 1);
        },
        _ => {
            node.left = build_bvh(tree, base, n / 2, t0, t1);
            node.right = build_bvh(tree, base + n / 2, n - n / 2, t0, t1);
        },
    }
    if let (Some(left), Some(right)) = (node.left, node.right) {
        node.aabb = surrounding_box(&tree.nodes[left].aabb, &tree.nodes[right].aabb);
        tree.nodes.push(node);
    }
    Some(tree.nodes.len() - 1)
}

pub fn hit_bvh_node(
    tree: &BVHTree,
    positions: &ReadStorage<Position>,
    hitables: &ReadStorage<Hitable>,
    node: &BVHNode,
    r: &Ray,
    t_min: f32,
    t_max: f32,
) -> Option<HitRecord> {
    if hit_aabb(&node.aabb, r, t_min, t_max) {
        let left_hit = node.left.map_or(None, |left| {
            let mut rec: Option<HitRecord>;
            if let Some(left_entity) = tree.nodes[left].entity {
                rec = hit(
                    tree,
                    positions,
                    hitables,
                    positions.get(left_entity),
                    hitables.get(left_entity).unwrap(),
                    r,
                    t_min,
                    t_max
                );
                if let Some(ref mut rec) = rec {
                    rec.entity = Some(left_entity);
                }
            } else {
                rec = hit_bvh_node(
                    tree,
                    positions,
                    hitables,
                    &tree.nodes[left],
                    r,
                    t_min,
                    t_max,
                );
            }
            rec
        });
        let right_hit = node.right.map_or(None, |right| {
            let mut rec: Option<HitRecord>;
            if let Some(right_entity) = tree.nodes[right].entity {
                rec = hit(
                    tree,
                    positions,
                    hitables,
                    positions.get(right_entity),
                    hitables.get(right_entity).unwrap(),
                    r,
                    t_min,
                    t_max
                );
                if let Some(ref mut rec) = rec {
                    rec.entity = Some(right_entity);
                }
            } else {
                rec = hit_bvh_node(
                    tree,
                    positions,
                    hitables,
                    &tree.nodes[right],
                    r,
                    t_min,
                    t_max,
                );
            }
            rec
        });
        if let (Some(left_hit), Some(right_hit)) = (left_hit, right_hit) {
            if left_hit.t < right_hit.t {
                return Some(left_hit);
            }
            return Some(right_hit);
        } else if let Some(left_hit) = left_hit {
            return Some(left_hit);
        } else if let Some(right_hit) = right_hit {
            return Some(right_hit);
        }
    }
    None
}
