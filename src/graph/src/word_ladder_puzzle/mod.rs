use hash_map::hash_map::HashMap;
use vec::{linked_vec, vec::LinkedVec};

use crate::{
    breadth_first_search::breadth_first_search,
    graph::{vertex::Vertex, Graph},
};

/// # Word Ladder Puzzle
///
/// word_ladder_puzzle is a game that finds the shortest path between two words in a list of words. The game use a graph data structure to represent the words and their connections. The connections are determined by the words that differ by only one letter.
///
/// # Arguments
///
/// * `words` - A vector of words.
/// * `start` - The starting word.
/// * `end` - The ending word.
///
/// # Returns
///
/// A tuple containing the total distance between the two words and the path between them.
///
/// # Example
///
/// ```
/// use crate::graph::{graph::Graph, word_ladder_puzzle::word_ladder_puzzle};
/// use vec::linked_vec;
///
/// let start = "hit";
/// let end = "cog";
/// let words = linked_vec!["hit", "hot", "dot", "dog", "lot", "log", "cog"];

/// let result = word_ladder_puzzle(words, start, end);
///
/// assert_eq!(result.0, 4);
/// ```
pub fn word_ladder_puzzle(
    words: LinkedVec<&str>,
    start: &str,
    end: &str,
) -> (u32, LinkedVec<String>) {
    let mut pattern_dic: HashMap<String, LinkedVec<String>> = HashMap::new();
    for word in words.iter() {
        for i in 0..word.len() {
            let pattern = word[..i].to_string() + "_" + &word[i + 1..];
            if pattern_dic.contains(&pattern) {
                pattern_dic
                    .get_mut(&pattern)
                    .unwrap()
                    .push(word.to_string());
            } else {
                pattern_dic.insert(pattern, linked_vec![word.to_string()]);
            }
        }
    }

    let mut graph = Graph::new();
    for (_, words) in pattern_dic.iter() {
        for w1 in words.iter() {
            for w2 in words.iter() {
                if w1 != w2 {
                    graph.add_edge(w1, w2, 1);
                }
            }
        }
    }

    let start_vertex = graph
        .get_vertex(&start.to_string())
        .unwrap_or(Vertex::new(start.to_string()));
    let end_vertex = graph
        .get_vertex(&end.to_string())
        .unwrap_or(Vertex::new(end.to_string()));

    breadth_first_search(&mut graph, start_vertex, end_vertex)
}
