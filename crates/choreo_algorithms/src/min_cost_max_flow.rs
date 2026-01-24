use crate::{AlgorithmError, Vector2};

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    rev: usize,
    capacity: i32,
    cost: f32,
    is_reverse: bool,
}

#[derive(Debug, Clone)]
pub struct MinCostMaxFlowSolver {
    adjacency: Vec<Vec<Edge>>,
}

impl MinCostMaxFlowSolver {
    pub fn new(node_count: usize) -> Result<Self, AlgorithmError> {
        if node_count == 0 {
            return Err(AlgorithmError::InvalidParameter("node_count must be positive."));
        }

        Ok(Self {
            adjacency: vec![Vec::new(); node_count],
        })
    }

    pub fn node_count(&self) -> usize {
        self.adjacency.len()
    }

    pub fn add_edge(
        &mut self,
        from_node: usize,
        to_node: usize,
        capacity: i32,
        cost: f32,
    ) -> Result<(), AlgorithmError> {
        if from_node >= self.node_count() {
            return Err(AlgorithmError::InvalidNode("from_node is out of range."));
        }
        if to_node >= self.node_count() {
            return Err(AlgorithmError::InvalidNode("to_node is out of range."));
        }
        if capacity < 0 {
            return Err(AlgorithmError::InvalidParameter(
                "capacity must be non-negative.",
            ));
        }
        if !cost.is_finite() {
            return Err(AlgorithmError::InvalidParameter(
                "cost must be finite.",
            ));
        }

        let from_index = self.adjacency[from_node].len();
        let to_index = self.adjacency[to_node].len();

        let forward = Edge {
            to: to_node,
            rev: to_index,
            capacity,
            cost,
            is_reverse: false,
        };
        let reverse = Edge {
            to: from_node,
            rev: from_index,
            capacity: 0,
            cost: -cost,
            is_reverse: true,
        };

        self.adjacency[from_node].push(forward);
        self.adjacency[to_node].push(reverse);

        Ok(())
    }

    pub fn compute_min_cost_flow(
        &mut self,
        source_node: usize,
        sink_node: usize,
        requested_flow: i32,
    ) -> Result<(i32, f32), AlgorithmError> {
        if source_node >= self.node_count() {
            return Err(AlgorithmError::InvalidNode("source_node is out of range."));
        }
        if sink_node >= self.node_count() {
            return Err(AlgorithmError::InvalidNode("sink_node is out of range."));
        }
        if requested_flow < 0 {
            return Err(AlgorithmError::InvalidParameter(
                "requested_flow must be non-negative.",
            ));
        }
        if source_node == sink_node {
            return Err(AlgorithmError::InvalidParameter(
                "source_node and sink_node must be different.",
            ));
        }

        let node_count = self.node_count();
        let mut potentials = vec![0.0f32; node_count];
        let mut distances = vec![f32::INFINITY; node_count];
        let mut previous = vec![None::<(usize, usize)>; node_count];

        let mut total_sent_flow = 0i32;
        let mut total_cost = 0.0f32;

        while total_sent_flow < requested_flow {
            distances.fill(f32::INFINITY);
            previous.fill(None);
            let mut used = vec![false; node_count];

            distances[source_node] = 0.0;

            for _ in 0..node_count {
                let mut best_node = None;
                let mut best_distance = f32::INFINITY;

                for node in 0..node_count {
                    if !used[node] && distances[node] < best_distance {
                        best_distance = distances[node];
                        best_node = Some(node);
                    }
                }

                let Some(current) = best_node else {
                    break;
                };

                used[current] = true;
                if current == sink_node {
                    break;
                }

                for (edge_index, edge) in self.adjacency[current].iter().enumerate() {
                    if edge.capacity <= 0 {
                        continue;
                    }

                    let reduced_cost = edge.cost + potentials[current] - potentials[edge.to];
                    let candidate = distances[current] + reduced_cost;

                    if candidate < distances[edge.to] {
                        distances[edge.to] = candidate;
                        previous[edge.to] = Some((current, edge_index));
                    }
                }
            }

            if previous[sink_node].is_none() {
                break;
            }

            for node in 0..node_count {
                if distances[node].is_finite() {
                    potentials[node] += distances[node];
                }
            }

            let mut augmenting_flow = requested_flow - total_sent_flow;
            let mut node = sink_node;
            while node != source_node {
                let (prev_node, edge_index) = previous[node]
                    .ok_or(AlgorithmError::NoPerfectAssignment("No augmenting path."))?;
                let capacity = self.adjacency[prev_node][edge_index].capacity;
                if capacity < augmenting_flow {
                    augmenting_flow = capacity;
                }
                node = prev_node;
            }

            let mut node = sink_node;
            while node != source_node {
                let (prev_node, edge_index) = previous[node]
                    .ok_or(AlgorithmError::NoPerfectAssignment("No augmenting path."))?;
                let cost = self.adjacency[prev_node][edge_index].cost;
                let reverse_index = self.adjacency[prev_node][edge_index].rev;

                if prev_node < node {
                    let (left, right) = self.adjacency.split_at_mut(node);
                    left[prev_node][edge_index].capacity -= augmenting_flow;
                    right[0][reverse_index].capacity += augmenting_flow;
                } else if prev_node > node {
                    let (left, right) = self.adjacency.split_at_mut(prev_node);
                    right[0][edge_index].capacity -= augmenting_flow;
                    left[node][reverse_index].capacity += augmenting_flow;
                } else {
                    let edges = &mut self.adjacency[prev_node];
                    edges[edge_index].capacity -= augmenting_flow;
                    edges[reverse_index].capacity += augmenting_flow;
                }

                total_cost += augmenting_flow as f32 * cost;
                node = prev_node;
            }

            total_sent_flow += augmenting_flow;
        }

        Ok((total_sent_flow, total_cost))
    }

    pub fn enumerate_forward_edges(
        &self,
    ) -> Vec<(usize, usize, i32, f32, i32)> {
        let mut edges = Vec::new();

        for (from_node, adjacency) in self.adjacency.iter().enumerate() {
            for edge in adjacency {
                if edge.is_reverse {
                    continue;
                }

                let reverse_capacity = self.adjacency[edge.to][edge.rev].capacity;
                edges.push((
                    from_node,
                    edge.to,
                    edge.capacity,
                    edge.cost,
                    reverse_capacity,
                ));
            }
        }

        edges
    }
}

pub fn solve_assignment(
    initial_points: &[Vector2],
    target_points: &[Vector2],
) -> Result<Vec<usize>, AlgorithmError> {
    solve_assignment_with(initial_points, target_points, |distance| distance * distance, |_, _| true)
}

pub fn solve_assignment_with<F, G>(
    initial_points: &[Vector2],
    target_points: &[Vector2],
    cost_func: F,
    is_allowed_pair: G,
) -> Result<Vec<usize>, AlgorithmError>
where
    F: Fn(f32) -> f32,
    G: Fn(usize, usize) -> bool,
{
    if initial_points.len() != target_points.len() {
        return Err(AlgorithmError::SizeMismatch("Point sets must have equal size."));
    }

    let point_count = initial_points.len();
    if point_count == 0 {
        return Ok(Vec::new());
    }

    let source_node = 0usize;
    let first_initial_node = 1usize;
    let first_target_node = first_initial_node + point_count;
    let sink_node = first_target_node + point_count;
    let node_count = sink_node + 1;

    let mut solver = MinCostMaxFlowSolver::new(node_count)?;

    for (initial_index, _) in initial_points.iter().enumerate() {
        let initial_node = first_initial_node + initial_index;
        solver.add_edge(source_node, initial_node, 1, 0.0f32)?;
    }

    for (initial_index, initial_point) in initial_points.iter().enumerate() {
        let initial_node = first_initial_node + initial_index;

        for (target_index, target_point) in target_points.iter().enumerate() {
            if !is_allowed_pair(initial_index, target_index) {
                continue;
            }

            let target_node = first_target_node + target_index;
            let distance = initial_point.distance_to(*target_point);
            let cost = cost_func(distance);

            solver.add_edge(initial_node, target_node, 1, cost)?;
        }
    }

    for (target_index, _) in target_points.iter().enumerate() {
        let target_node = first_target_node + target_index;
        solver.add_edge(target_node, sink_node, 1, 0.0f32)?;
    }

    let (sent_flow, _) = solver.compute_min_cost_flow(source_node, sink_node, point_count as i32)?;
    if sent_flow != point_count as i32 {
        return Err(AlgorithmError::NoPerfectAssignment(
            "No perfect assignment exists under the given constraints.",
        ));
    }

    let mut assignment = vec![usize::MAX; point_count];

    for (from_node, to_node, _residual_capacity, _cost, reverse_residual_capacity) in
        solver.enumerate_forward_edges()
    {
        let from_is_initial = from_node >= first_initial_node && from_node < first_initial_node + point_count;
        let to_is_target = to_node >= first_target_node && to_node < first_target_node + point_count;
        if !from_is_initial || !to_is_target {
            continue;
        }

        if reverse_residual_capacity == 1 {
            let initial_index = from_node - first_initial_node;
            let target_index = to_node - first_target_node;
            assignment[initial_index] = target_index;
        }
    }

    if assignment.contains(&usize::MAX) {
        return Err(AlgorithmError::NoPerfectAssignment(
            "Assignment decoding failed.",
        ));
    }

    Ok(assignment)
}

pub fn solve_three_scene_assignment(
    scene_a: &[Vector2],
    scene_b: &[Vector2],
    scene_c: &[Vector2],
) -> Result<Vec<usize>, AlgorithmError> {
    solve_three_scene_assignment_with_fraction(scene_a, scene_b, scene_c, 0.5)
}

pub fn solve_three_scene_assignment_with_fraction(
    scene_a: &[Vector2],
    scene_b: &[Vector2],
    scene_c: &[Vector2],
    scene_b_fraction: f32,
) -> Result<Vec<usize>, AlgorithmError> {
    solve_three_scene_assignment_with(
        scene_a,
        scene_b,
        scene_c,
        scene_b_fraction,
        move |start, mid, end| {
            let expected_mid = Vector2::lerp(start, end, scene_b_fraction);
            let path_energy = (mid - start).length_squared() + (end - mid).length_squared();
            let deviation_energy = (mid - expected_mid).length_squared();
            path_energy + deviation_energy
        },
        |_, _| true,
    )
}

pub fn solve_three_scene_assignment_with<F, G>(
    scene_a: &[Vector2],
    scene_b: &[Vector2],
    scene_c: &[Vector2],
    scene_b_fraction: f32,
    cost_func: F,
    is_allowed_pair: G,
) -> Result<Vec<usize>, AlgorithmError>
where
    F: Fn(Vector2, Vector2, Vector2) -> f32,
    G: Fn(usize, usize) -> bool,
{
    let count = scene_a.len();
    if count != scene_b.len() || count != scene_c.len() {
        return Err(AlgorithmError::SizeMismatch(
            "All scene position sets must have the same length.",
        ));
    }

    if count == 0 {
        return Ok(Vec::new());
    }

    if !(0.0..=1.0).contains(&scene_b_fraction) {
        return Err(AlgorithmError::InvalidParameter(
            "scene_b_fraction must be in [0,1].",
        ));
    }

    let source_node = 0usize;
    let first_start_node = 1usize;
    let first_mid_node = first_start_node + count;
    let sink_node = first_mid_node + count;
    let node_count = sink_node + 1;

    let mut solver = MinCostMaxFlowSolver::new(node_count)?;

    for (start_index, _) in scene_a.iter().enumerate() {
        let start_node = first_start_node + start_index;
        solver.add_edge(source_node, start_node, 1, 0.0f32)?;
    }

    for (start_index, (start, end)) in scene_a.iter().zip(scene_c).enumerate() {
        let start_node = first_start_node + start_index;

        for (mid_index, mid) in scene_b.iter().enumerate() {
            if !is_allowed_pair(start_index, mid_index) {
                continue;
            }

            let mid_node = first_mid_node + mid_index;
            let cost = cost_func(*start, *mid, *end);
            solver.add_edge(start_node, mid_node, 1, cost)?;
        }
    }

    for (mid_index, _) in scene_b.iter().enumerate() {
        let mid_node = first_mid_node + mid_index;
        solver.add_edge(mid_node, sink_node, 1, 0.0f32)?;
    }

    let (sent_flow, _) = solver.compute_min_cost_flow(source_node, sink_node, count as i32)?;
    if sent_flow != count as i32 {
        return Err(AlgorithmError::NoPerfectAssignment(
            "No perfect assignment exists with the provided constraints.",
        ));
    }

    let mut assignment = vec![usize::MAX; count];

    for (from_node, to_node, _residual_capacity, _cost, reverse_residual_capacity) in
        solver.enumerate_forward_edges()
    {
        let from_is_start = from_node >= first_start_node && from_node < first_start_node + count;
        let to_is_mid = to_node >= first_mid_node && to_node < first_mid_node + count;
        if !from_is_start || !to_is_mid {
            continue;
        }

        if reverse_residual_capacity == 1 {
            let start_index = from_node - first_start_node;
            let mid_index = to_node - first_mid_node;
            assignment[start_index] = mid_index;
        }
    }

    if assignment.contains(&usize::MAX) {
        return Err(AlgorithmError::NoPerfectAssignment(
            "Assignment decoding failed.",
        ));
    }

    Ok(assignment)
}
