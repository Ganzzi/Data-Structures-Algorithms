use hash_map::hash_map::HashMap;

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
///
/// let start = "hit";
/// let end = "cog";
/// let words = vec!["hit", "hot", "dot", "dog", "cog"];
///
/// let result = word_ladder_puzzle(words, start, end);
///
/// assert_eq!(result.0, 4);
/// assert_eq!(result.1, vec!["hit", "hot", "dot", "dog", "cog"]);
/// ```
pub fn word_ladder_puzzle(words: Vec<&str>, start: &str, end: &str) -> (u32, Vec<String>) {
    let mut pattern_dic: HashMap<String, Vec<String>> = HashMap::new();
    for word in words {
        for i in 0..word.len() {
            let pattern = word[..i].to_string() + "_" + &word[i + 1..];
            if pattern_dic.contains(&pattern) {
                pattern_dic
                    .get_mut(&pattern)
                    .unwrap()
                    .push(word.to_string());
            } else {
                pattern_dic.insert(pattern, vec![word.to_string()]);
            }
        }
    }

    let mut graph = Graph::new();
    for (_, words) in pattern_dic.iter() {
        for w1 in words {
            for w2 in words {
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
