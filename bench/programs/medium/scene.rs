#![no_std]

include!("../prelude.rs");

// Trait-object-heavy scene graph walk plus BFS/DFS over an explicit graph.
//
// Nodes are stored as `Box<dyn Node>`; visitors are `&mut dyn Visitor`, so both
// the tree walk and the per-node work dispatch dynamically.  Transforms and
// geometry use integers only, which keeps both backends bit-identical.

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

fn mix(hash: &mut u64, value: u64) {
    *hash = (*hash ^ value)
        .wrapping_mul(0x100_0000_01b3)
        .rotate_left(7);
}

#[derive(Clone, Copy)]
struct Transform {
    tx: i64,
    ty: i64,
    scale: i64,
}

impl Transform {
    const IDENTITY: Transform = Transform {
        tx: 0,
        ty: 0,
        scale: 1,
    };

    fn then(self, child: Transform) -> Transform {
        Transform {
            tx: self.tx.wrapping_add(child.tx.wrapping_mul(self.scale)),
            ty: self.ty.wrapping_add(child.ty.wrapping_mul(self.scale)),
            scale: self.scale.wrapping_mul(child.scale),
        }
    }

    fn point(self, x: i64, y: i64) -> (i64, i64) {
        (
            self.tx.wrapping_add(x.wrapping_mul(self.scale)),
            self.ty.wrapping_add(y.wrapping_mul(self.scale)),
        )
    }
}

trait Visitor {
    fn enter(&mut self, node: &dyn Node, world: Transform, depth: u32);
    fn leave(&mut self, node: &dyn Node, world: Transform, depth: u32);
}

trait Node {
    fn id(&self) -> u32;
    fn local(&self) -> Transform;
    fn kind(&self) -> u8;
    fn weight(&self) -> i64;
    fn extent(&self) -> i64 {
        0
    }
    fn vertices(&self) -> u32 {
        0
    }
    fn tag(&self) -> u64 {
        0
    }
    fn accept(&self, visitor: &mut dyn Visitor, parent: Transform, depth: u32);
}

struct Group {
    id: u32,
    local: Transform,
    children: Vec<Box<dyn Node>>,
}

impl Node for Group {
    fn id(&self) -> u32 { self.id }

    fn local(&self) -> Transform { self.local }

    fn kind(&self) -> u8 { 0 }

    fn weight(&self) -> i64 { 1 }

    fn accept(&self, visitor: &mut dyn Visitor, parent: Transform, depth: u32) {
        let world = parent.then(self.local());
        visitor.enter(self, world, depth);
        for child in &self.children {
            child.accept(visitor, world, depth + 1);
        }
        visitor.leave(self, world, depth);
    }
}

struct Shape {
    id: u32,
    local: Transform,
    sides: u32,
    radius: i64,
    weight: i64,
}

impl Node for Shape {
    fn id(&self) -> u32 { self.id }

    fn local(&self) -> Transform { self.local }

    fn kind(&self) -> u8 { 1 }

    fn weight(&self) -> i64 { self.weight }

    fn extent(&self) -> i64 { self.radius }

    fn vertices(&self) -> u32 { self.sides }

    fn accept(&self, visitor: &mut dyn Visitor, parent: Transform, depth: u32) {
        let world = parent.then(self.local());
        visitor.enter(self, world, depth);
        visitor.leave(self, world, depth);
    }
}

struct Marker {
    id: u32,
    local: Transform,
    tag: u64,
    weight: i64,
}

impl Node for Marker {
    fn id(&self) -> u32 { self.id }

    fn local(&self) -> Transform { self.local }

    fn kind(&self) -> u8 { 2 }

    fn weight(&self) -> i64 { self.weight }

    fn tag(&self) -> u64 { self.tag }

    fn accept(&self, visitor: &mut dyn Visitor, parent: Transform, depth: u32) {
        let world = parent.then(self.local());
        visitor.enter(self, world, depth);
        visitor.leave(self, world, depth);
    }
}

struct AabbVisitor {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
    enters: u64,
    leaves: u64,
    groups: u64,
    shapes: u64,
    markers: u64,
    weight_sum: i64,
    perimeter: i64,
    max_depth: u32,
    hash: u64,
}

impl AabbVisitor {
    fn new() -> Self {
        AabbVisitor {
            min_x: i64::MAX,
            min_y: i64::MAX,
            max_x: i64::MIN,
            max_y: i64::MIN,
            enters: 0,
            leaves: 0,
            groups: 0,
            shapes: 0,
            markers: 0,
            weight_sum: 0,
            perimeter: 0,
            max_depth: 0,
            hash: 0xcbf2_9ce4_8422_2325,
        }
    }

    fn observe(&mut self, node: &dyn Node, world: Transform, depth: u32) {
        self.enters = self.enters.wrapping_add(1);
        self.weight_sum = self.weight_sum.wrapping_add(node.weight());
        if depth > self.max_depth {
            self.max_depth = depth;
        }
        match node.kind() {
            0 => self.groups = self.groups.wrapping_add(1),
            1 => {
                self.shapes = self.shapes.wrapping_add(1);
                let extent = node.extent();
                let (x0, y0) = world.point(extent.wrapping_neg(), extent.wrapping_neg());
                let (x1, y1) = world.point(extent, extent);
                self.min_x = self.min_x.min(x0);
                self.min_y = self.min_y.min(y0);
                self.max_x = self.max_x.max(x1);
                self.max_y = self.max_y.max(y1);
                let sides = node.vertices().max(3) as i64;
                self.perimeter = self
                    .perimeter
                    .wrapping_add(sides.wrapping_mul(extent).wrapping_mul(4));
            }
            _ => {
                self.markers = self.markers.wrapping_add(1);
                mix(&mut self.hash, node.tag());
            }
        }
        mix(&mut self.hash, node.id() as u64 ^ ((depth as u64) << 32));
    }
}

impl Visitor for AabbVisitor {
    fn enter(&mut self, node: &dyn Node, world: Transform, depth: u32) {
        self.observe(node, world, depth);
    }

    fn leave(&mut self, _node: &dyn Node, world: Transform, _depth: u32) {
        self.leaves = self.leaves.wrapping_add(1);
        let code = (world.scale as u64)
            ^ (world.tx as u64).rotate_left(17)
            ^ (world.ty as u64).rotate_left(31);
        mix(&mut self.hash, code);
    }
}

struct IdVisitor {
    ids: Vec<u32>,
    checksum: u64,
    max_depth: u32,
    histogram: [u32; 8],
    leaves: u64,
}

impl IdVisitor {
    fn new() -> Self {
        IdVisitor {
            ids: Vec::new(),
            checksum: 0x9e37_79b9_7f4a_7c15,
            max_depth: 0,
            histogram: [0; 8],
            leaves: 0,
        }
    }
}

impl Visitor for IdVisitor {
    fn enter(&mut self, node: &dyn Node, _world: Transform, depth: u32) {
        self.ids.push(node.id());
        if depth > self.max_depth {
            self.max_depth = depth;
        }
        if (depth as usize) < self.histogram.len() {
            self.histogram[depth as usize] = self.histogram[depth as usize].wrapping_add(1);
        }
        mix(&mut self.checksum, (node.id() as u64) ^ ((node.kind() as u64) << 24));
    }

    fn leave(&mut self, node: &dyn Node, _world: Transform, depth: u32) {
        self.leaves = self.leaves.wrapping_add(1);
        mix(&mut self.checksum, node.id() as u64 + depth as u64);
    }
}

fn make_node(rng: &mut Rng, next_id: &mut u32, depth: u32) -> Box<dyn Node> {
    let id = *next_id;
    *next_id += 1;
    let local = Transform {
        tx: rng.below(21) as i64 - 10,
        ty: rng.below(21) as i64 - 10,
        scale: 1 + rng.below(3) as i64,
    };
    let roll = rng.below(100);
    if depth < 3 && roll < 42 {
        let child_count = 1 + rng.below(4);
        let mut children: Vec<Box<dyn Node>> = Vec::new();
        let mut child = 0;
        while child < child_count {
            children.push(make_node(rng, next_id, depth + 1));
            child += 1;
        }
        Box::new(Group {
            id,
            local,
            children,
        })
    } else if roll < 76 {
        Box::new(Shape {
            id,
            local,
            sides: 3 + rng.below(6) as u32,
            radius: 1 + rng.below(7) as i64,
            weight: 1 + rng.below(10) as i64,
        })
    } else {
        Box::new(Marker {
            id,
            local,
            tag: rng.next_u64(),
            weight: 1 + rng.below(5) as i64,
        })
    }
}

fn build_scene(seed: u64) -> Vec<Box<dyn Node>> {
    let mut rng = Rng::new(seed);
    let root_count = 2 + rng.below(3);
    let mut next_id = 1u32;
    let mut roots: Vec<Box<dyn Node>> = Vec::new();
    let mut root = 0;
    while root < root_count {
        roots.push(make_node(&mut rng, &mut next_id, 0));
        root += 1;
    }
    roots
}

struct Graph {
    weights: Vec<i64>,
    kinds: Vec<u8>,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn len(&self) -> usize {
        self.adj.len()
    }
}

fn build_graph(seed: u64, count: usize) -> Graph {
    let mut rng = Rng::new(seed ^ 0x5eed_1234_abcd_9876);
    let mut weights = Vec::with_capacity(count);
    let mut kinds = Vec::with_capacity(count);
    let mut adj: Vec<Vec<usize>> = Vec::with_capacity(count);
    let mut index = 0;
    while index < count {
        weights.push(rng.below(21) as i64 - 10);
        kinds.push((rng.next_u32() % 4) as u8);
        adj.push(Vec::new());
        index += 1;
    }
    index = 0;
    while index < count {
        if index + 1 < count {
            adj[index].push(index + 1);
        }
        let extra = rng.below(3);
        let mut edge = 0;
        while edge < extra {
            let target = (rng.next_u64() as usize) % count;
            if target != index {
                adj[index].push(target);
            }
            edge += 1;
        }
        index += 1;
    }
    Graph {
        weights,
        kinds,
        adj,
    }
}

fn bfs(graph: &Graph, start: usize) -> (u64, u64, i64) {
    let count = graph.len();
    if count == 0 {
        return (0, 0, 0);
    }
    let mut seen = Vec::new();
    seen.resize(count, false);
    let mut distance = Vec::new();
    distance.resize(count, 0i64);
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut order = 0u64;
    let mut visited = 0u64;
    let mut distance_sum = 0i64;
    seen[start] = true;
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        visited = visited.wrapping_add(1);
        mix(
            &mut order,
            (node as u64) ^ ((graph.kinds[node] as u64) << 8),
        );
        distance_sum = distance_sum
            .wrapping_add(graph.weights[node])
            .wrapping_add(distance[node]);
        for &next in &graph.adj[node] {
            if !seen[next] {
                seen[next] = true;
                distance[next] = distance[node] + 1;
                queue.push_back(next);
            }
        }
    }
    (order, visited, distance_sum)
}

fn dfs(graph: &Graph, start: usize) -> u64 {
    let count = graph.len();
    if count == 0 {
        return 0;
    }
    let mut seen = Vec::new();
    seen.resize(count, false);
    let mut stack: Vec<usize> = Vec::new();
    let mut order = 0u64;
    stack.push(start);
    while let Some(node) = stack.pop() {
        if seen[node] {
            continue;
        }
        seen[node] = true;
        mix(&mut order, (node as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        let mut edge = graph.adj[node].len();
        while edge > 0 {
            edge -= 1;
            let next = graph.adj[node][edge];
            if !seen[next] {
                stack.push(next);
            }
        }
    }
    order
}

fn body(seed: u64) -> u64 {
    let scene = build_scene(seed);

    let mut aabb = AabbVisitor::new();
    let mut root = 0;
    while root < scene.len() {
        scene[root].accept(&mut aabb, Transform::IDENTITY, 0);
        root += 1;
    }
    emit_u64(aabb.enters);
    emit_u64(aabb.leaves);
    emit_u64(aabb.groups);
    emit_u64(aabb.shapes);
    emit_u64(aabb.markers);
    emit_u64(aabb.max_depth as u64);
    emit_i64(aabb.weight_sum);
    emit_i64(aabb.perimeter);
    emit_i64(aabb.min_x);
    emit_i64(aabb.min_y);
    emit_i64(aabb.max_x);
    emit_i64(aabb.max_y);
    emit_u64(aabb.hash);

    let mut ids = IdVisitor::new();
    let mut node = 0;
    while node < scene.len() {
        scene[node].accept(&mut ids, Transform::IDENTITY, 0);
        node += 1;
    }
    let id_sum = ids
        .ids
        .iter()
        .fold(0u64, |acc, &value| acc.wrapping_add(value as u64));
    let histogram_sum = ids
        .histogram
        .iter()
        .fold(0u32, |acc, &value| acc.wrapping_add(value));
    emit_u64(ids.ids.len() as u64);
    emit_u64(id_sum);
    emit_u64(ids.checksum);
    emit_u64(ids.leaves);
    emit_u64(ids.max_depth as u64);
    emit_u64(histogram_sum as u64);

    let graph_count = 20 + (seed % 13) as usize;
    let graph = build_graph(seed, graph_count);
    let (bfs_order, bfs_visited, bfs_distance) = bfs(&graph, 0);
    let dfs_order = dfs(&graph, 0);
    let weight_sum = graph
        .weights
        .iter()
        .fold(0u64, |acc, &value| acc.wrapping_add(value as u64));
    emit_u64(bfs_order);
    emit_u64(bfs_visited);
    emit_i64(bfs_distance);
    emit_u64(dfs_order);
    emit_u64(weight_sum);

    let mut summary = splitmix64(seed);
    mix(&mut summary, aabb.hash);
    mix(&mut summary, ids.checksum ^ id_sum);
    mix(&mut summary, bfs_order ^ dfs_order);
    mix(&mut summary, bfs_visited);
    mix(&mut summary, bfs_distance as u64);
    mix(&mut summary, aabb.enters ^ aabb.shapes);
    emit_u64(summary);
    summary
}
