use std::cmp::Ordering;

use crate::{AlgorithmError, Vector2};

pub fn fill_squared_distances(
    initial: &[Vector2],
    target: &[Vector2],
) -> Result<Vec<Vec<f32>>, AlgorithmError> {
    if initial.len() != target.len() {
        return Err(AlgorithmError::SizeMismatch(
            "Point sets must have equal size.",
        ));
    }

    let n = initial.len();
    let mut distances = vec![vec![0.0f32; n]; n];

    for (i, &point) in initial.iter().enumerate() {
        for (j, &target_point) in target.iter().enumerate() {
            distances[i][j] = point.squared_distance_to(target_point);
        }
    }

    Ok(distances)
}

pub fn compute_sigma_vector(cost_matrix: &[Vec<f32>]) -> Result<Vec<usize>, AlgorithmError> {
    let n = cost_matrix.len();
    if n == 0 {
        return Ok(Vec::new());
    }

    if cost_matrix.iter().any(|row| row.len() != n) {
        return Err(AlgorithmError::InvalidParameter(
            "Hungarian algorithm requires a square matrix.",
        ));
    }

    let mut u = vec![0.0f32; n + 1];
    let mut v = vec![0.0f32; n + 1];
    let mut p = vec![0usize; n + 1];
    let mut way = vec![0usize; n + 1];

    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0usize;
        let mut minv = vec![f32::INFINITY; n + 1];
        let mut used = vec![false; n + 1];

        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = f32::INFINITY;
            let mut j1 = 0usize;

            for j in 1..=n {
                if used[j] {
                    continue;
                }

                let cur = cost_matrix[i0 - 1][j - 1] - u[i0] - v[j];
                if cur < minv[j] {
                    minv[j] = cur;
                    way[j] = j0;
                }

                if minv[j] < delta {
                    delta = minv[j];
                    j1 = j;
                }
            }

            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }

        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }

    let mut result = vec![0usize; n];
    for (j, &assigned) in p.iter().enumerate().skip(1) {
        if assigned > 0 {
            result[assigned - 1] = j - 1;
        }
    }

    Ok(result)
}

pub fn compute_assignment(
    initial: &[Vector2],
    target: &[Vector2],
) -> Result<Vec<usize>, AlgorithmError> {
    compute_assignment_with(initial, target, |distance| distance * distance)
}

pub fn compute_assignment_with<F>(
    initial: &[Vector2],
    target: &[Vector2],
    energy_func: F,
) -> Result<Vec<usize>, AlgorithmError>
where
    F: Fn(f32) -> f32,
{
    if initial.len() != target.len() {
        return Err(AlgorithmError::SizeMismatch(
            "Point sets must have equal size.",
        ));
    }

    let n = initial.len();
    if n == 0 {
        return Ok(Vec::new());
    }

    let squared_distances = fill_squared_distances(initial, target)?;
    let mut distances = vec![vec![0.0f32; n]; n];
    let mut all_distances = Vec::with_capacity(n * n);

    for i in 0..n {
        for j in 0..n {
            let d = squared_distances[i][j].sqrt();
            distances[i][j] = d;
            all_distances.push(d);
        }
    }

    all_distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let min_max_distance = find_minimal_max_distance(&distances, &all_distances);

    let mut cost_matrix = vec![vec![0.0f32; n]; n];
    let big_m = 1e12f32;

    for i in 0..n {
        for j in 0..n {
            let d = distances[i][j];
            cost_matrix[i][j] = if d <= min_max_distance {
                energy_func(d)
            } else {
                big_m
            };
        }
    }

    compute_sigma_vector(&cost_matrix)
}

pub fn compute_mid_scene_assignment(
    scene_a: &[Vector2],
    scene_b: &[Vector2],
    scene_c: &[Vector2],
) -> Result<Vec<usize>, AlgorithmError> {
    compute_mid_scene_assignment_with_fraction(scene_a, scene_b, scene_c, 0.5)
}

pub fn compute_mid_scene_assignment_with_fraction(
    scene_a: &[Vector2],
    scene_b: &[Vector2],
    scene_c: &[Vector2],
    scene_b_fraction: f32,
) -> Result<Vec<usize>, AlgorithmError> {
    compute_mid_scene_assignment_with(
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

pub fn compute_mid_scene_assignment_with<F, G>(
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

    let mut cost_matrix = vec![vec![0.0f32; count]; count];
    let big_m = 1e12f32;

    for dancer_index in 0..count {
        let start = scene_a[dancer_index];
        let end = scene_c[dancer_index];
        for candidate_index in 0..count {
            let cost = if is_allowed_pair(dancer_index, candidate_index) {
                cost_func(start, scene_b[candidate_index], end)
            } else {
                big_m
            };
            cost_matrix[dancer_index][candidate_index] = cost;
        }
    }

    compute_sigma_vector(&cost_matrix)
}

fn find_minimal_max_distance(distances: &[Vec<f32>], sorted_distances: &[f32]) -> f32 {
    let mut left = 0usize;
    let mut right = sorted_distances.len().saturating_sub(1);
    let mut best = sorted_distances[right];

    while left <= right {
        let mid = (left + right) / 2;
        let threshold = sorted_distances[mid];

        if try_find_perfect_matching(distances, threshold) {
            best = threshold;
            if mid == 0 {
                break;
            }
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    best
}

fn try_find_perfect_matching(distances: &[Vec<f32>], threshold: f32) -> bool {
    let n = distances.len();
    let mut match_to_left = vec![-1i32; n];

    for left_node in 0..n {
        let mut visited = vec![false; n];
        if !augment(
            left_node,
            distances,
            threshold,
            &mut match_to_left,
            &mut visited,
        ) {
            return false;
        }
    }

    true
}

fn augment(
    left_node: usize,
    distances: &[Vec<f32>],
    threshold: f32,
    match_to_left: &mut [i32],
    visited: &mut [bool],
) -> bool {
    let n = distances.len();

    for right_node in 0..n {
        if visited[right_node] {
            continue;
        }

        if distances[left_node][right_node] > threshold {
            continue;
        }

        visited[right_node] = true;

        if match_to_left[right_node] == -1
            || augment(
                match_to_left[right_node] as usize,
                distances,
                threshold,
                match_to_left,
                visited,
            )
        {
            match_to_left[right_node] = left_node as i32;
            return true;
        }
    }

    false
}
