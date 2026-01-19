use bevy::math::{vec2, Vec2};

fn rotate(v: Vec2) -> Vec2 {
    vec2(v.y, v.x)
}

/// Convert grid path to a list of portals
fn build_portals(points: &Vec<Vec2>) -> Vec<(Vec2, Vec2)> {
    let mut portals = Vec::new();
    if points.len() < 2 {
        return portals;
    }

    for i in 0..points.len() - 1 {
        let p1 = (points[i] - 0.5).round() + 0.5;
        let p2 = (points[i + 1] - 0.5).round() + 0.5;

        let p1f = rotate(p1);

        let dy = (p2.x - p1.x) as i32;
        let dx = (p2.y - p1.y) as i32;

        let (left, right) = match (dy, dx) {
            // Horizontal and vertical movement (logic unchanged)
            (0, 1) => (
                rotate(vec2(p1f.y - 0.5, p1f.x + 0.5)),
                rotate(vec2(p1f.y + 0.5, p1f.x + 0.5)),
            ), // Right
            (0, -1) => (
                rotate(vec2(p1f.y + 0.5, p1f.x - 0.5)),
                rotate(vec2(p1f.y - 0.5, p1f.x - 0.5)),
            ), // Left
            (1, 0) => (
                rotate(vec2(p1f.y + 0.5, p1f.x + 0.5)),
                rotate(vec2(p1f.y + 0.5, p1f.x - 0.5)),
            ), // Down
            (-1, 0) => (
                rotate(vec2(p1f.y - 0.5, p1f.x - 0.5)),
                rotate(vec2(p1f.y - 0.5, p1f.x + 0.5)),
            ), // Up

            // Diagonal movement (new logic)
            // Virtual portal vertices are at the centers of adjacent "obstacle" cells
            (1, 1) => (
                rotate(vec2(p1f.y + 0.5, p1f.x + 0.5)),
                rotate(vec2(p1f.y + 0.5, p1f.x + 0.5)),
            ), // Bottom-right
            (1, -1) => (
                rotate(vec2(p1f.y + 0.5, p1f.x - 0.5)),
                rotate(vec2(p1f.y + 0.5, p1f.x - 0.5)),
            ), // Bottom-left
            (-1, -1) => (
                rotate(vec2(p1f.y - 0.5, p1f.x - 0.5)),
                rotate(vec2(p1f.y - 0.5, p1f.x - 0.5)),
            ), // Top-left
            (-1, 1) => (
                rotate(vec2(p1f.y - 0.5, p1f.x + 0.5)),
                rotate(vec2(p1f.y - 0.5, p1f.x + 0.5)),
            ), // Top-right

            _ => {
                // For non-continuous paths (jumps) or other invalid movements, this will panic
                panic!(
                    "Invalid or non-continuous movement detected in path: from {:?} to {:?}",
                    p1, p2
                );
            }
        };
        portals.push((left, right));
    }
    portals
}

/// Simplify the path generated on a 2D grid using the funnel algorithm.
///
/// # Arguments
/// * `points` - A vector of grid cell coordinates `(row, column)` representing the path.
///              The path must be continuous and contain only horizontal and vertical movements.
///
/// # Returns
/// A vector of simplified path points `(y, x)`.
pub fn simplify_path(points: &Vec<Vec2>) -> Vec<Vec2> {
    // 1. Handle edge cases
    if points.len() < 3 {
        return points.clone();
    }

    let mut simplified_points = Vec::new();

    // 2. Convert grid path to portal list
    let mut portals = build_portals(points);

    // 3. Set start and end points
    let start_pos = rotate(points.first().unwrap().clone());
    let end_pos = rotate(points.last().unwrap().clone());

    // Add a zero-width end portal to ensure the algorithm can process to the end of the path
    portals.push((end_pos, end_pos));

    simplified_points.push(start_pos);

    // 4. Initialize the funnel
    let mut apex = start_pos;
    let (mut left_tentacle, mut right_tentacle) = portals[0];

    let mut i = 1;
    while i < portals.len() {
        let (new_left, new_right) = portals[i];

        // 5. Try to update right tentacle
        // Vector: apex -> right_tentacle
        let vec_r = right_tentacle - apex;
        // Vector: apex -> new_right
        let vec_nr = new_right - apex;

        // If new_right is to the left of or collinear with right_tentacle (cross product <= 0),
        // the funnel has not widened on the right side.
        if vec_r.perp_dot(vec_nr) <= 0.0 {
            // Vector: apex -> left_tentacle
            let vec_l = left_tentacle - apex;
            // Check if new_right is still to the right of left_tentacle
            if vec_l.perp_dot(vec_nr) >= 0.0 {
                // new_right is still inside the funnel, tighten right tentacle
                right_tentacle = new_right;
            } else {
                // new_right crossed the left tentacle, meaning we found a turning point (left_tentacle).
                // Add left_tentacle to the path.
                simplified_points.push(left_tentacle);
                // Update apex to this new turning point
                apex = left_tentacle;

                // Restart from after the portal containing this turning point
                // We need to find the index where left_tentacle first appears as a portal vertex
                let restart_idx = portals
                    .iter()
                    .position(|p| p.0 == left_tentacle || p.1 == left_tentacle)
                    .unwrap_or(i);
                i = restart_idx + 1;

                // Reset the funnel
                if i < portals.len() {
                    left_tentacle = portals[i].0;
                    right_tentacle = portals[i].1;
                }
                continue;
            }
        }

        // 6. Try to update left tentacle (symmetric to right side)
        let vec_l = left_tentacle - apex;
        let vec_nl = new_left - apex;

        if vec_l.perp_dot(vec_nl) >= 0.0 {
            let vec_r = right_tentacle - apex;
            if vec_r.perp_dot(vec_nl) <= 0.0 {
                left_tentacle = new_left;
            } else {
                simplified_points.push(right_tentacle);
                apex = right_tentacle;

                let restart_idx = portals
                    .iter()
                    .position(|p| p.0 == right_tentacle || p.1 == right_tentacle)
                    .unwrap_or(i);
                i = restart_idx + 1;

                if i < portals.len() {
                    left_tentacle = portals[i].0;
                    right_tentacle = portals[i].1;
                }
                continue;
            }
        }
        i += 1;
    }

    // 7. Add end point
    simplified_points.push(end_pos);

    // 8. Convert to user-expected output format
    simplified_points.into_iter().map(|v| rotate(v)).collect()
}
