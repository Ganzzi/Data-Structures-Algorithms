use crate::graph::{vertex::Vertex, Graph};
use hash_map::hash_map::HashMap;

/// # Knight's Tour Puzzle
///
/// The knight's tour is a mathematical problem involving a knight on a chessboard. The knight is placed on any square of board and, moving according to the rules of chess, must visit each square exactly once.
///
/// # Arguments
///
/// * `board_size` - The size of the board.
/// * `start` - The starting position of the knight.
///
/// # Returns
///
/// Option of a vector containing the path of the knight's tour.
///
/// # Example
///
/// ```
/// use crate::graph::knight_tour_puzzle::knight_tour_puzzle;
///
/// let result = knight_tour_puzzle(5, 0);
///
/// if let Some(path) = result {
///    println!("Path: {:?}", path);
///    println!("Total distance: {}", path.len());
/// } else {
///   println!("No solution found.");
/// }
///
/// ```
pub fn knight_tour_puzzle(board_size: u32, start: u32) -> Option<Vec<u32>> {
    let calc_point =
        |row: u32, col: u32, board_size: u32| -> u32 { (row % board_size) * board_size + col };

    let mut graph = Graph::new();
    for row_cord in 0..board_size {
        for col_cord in 0..board_size {
            let valid_modes = valid_moves(row_cord, col_cord, board_size);
            for cords in valid_modes {
                let src = calc_point(row_cord, col_cord, board_size);
                let des = calc_point(cords.0, cords.1, board_size);
                graph.add_edge(&src, &des, 1);
            }
        }
    }

    let mut path = vec![];
    let mut explorations = HashMap::new();
    graph.get_vertex_keys().iter().for_each(|x| {
        explorations.insert(*x, false);
    });

    let start = graph.get_vertex(&start).unwrap().clone();
    explorations.insert(start.get_key().clone(), true);

    let success = start_knight_tour(&graph, board_size, &mut path, &mut explorations, start, 0);

    if success {
        return Some(path);
    } else {
        return None;
    }

    fn valid_moves(row_cord: u32, col_cord: u32, board_size: u32) -> Vec<(u32, u32)> {
        let possible_moves = vec![
            (2, 1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
        ];
        let is_valid_position =
            |x, y, board_size| x >= 0 && y >= 0 && x < board_size && y < board_size;

        let mut moves = vec![];
        for (x, y) in possible_moves {
            let new_row = row_cord as i32 + x;
            let new_col = col_cord as i32 + y;
            if is_valid_position(new_row, new_col, board_size as i32) {
                moves.push((new_row as u32, new_col as u32));
            }
        }

        moves
    }

    fn start_knight_tour(
        graph: &Graph<u32, u32>,
        board_size: u32,
        path: &mut Vec<u32>,
        explorations: &mut HashMap<u32, bool>,
        vertext: Vertex<u32, u32>,
        depth: u32,
    ) -> bool {
        path.push(*vertext.get_key());

        let mut done = false;
        if depth < board_size * board_size - 1 {
            let neightbours = vertext.get_neighbors();
            let mut i = 0;

            while i < neightbours.len() && !done {
                let neighbor_vertex = graph.get_vertex(&neightbours[i].0).unwrap().clone();

                if *explorations.get(&neightbours[i].0.clone()).unwrap() == false {
                    explorations.insert(neightbours[i].0.clone(), true);
                    done = start_knight_tour(
                        graph,
                        board_size,
                        path,
                        explorations,
                        neighbor_vertex,
                        depth + 1,
                    );
                    if !done {
                        path.pop();
                        explorations.insert(neightbours[i].0.clone(), false);
                    }
                }
                i += 1;
            }
        } else {
            done = true;
        }

        done
    }
}
