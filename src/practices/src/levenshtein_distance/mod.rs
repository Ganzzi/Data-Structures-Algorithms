use std::cmp::min;

pub fn levenshtein_distance(source: &str, target: &str) -> usize {
    let source_count = source.chars().count();
    let target_count = target.chars().count();

    if source.is_empty() {
        return target_count;
    } else if target.is_empty() {
        return source_count;
    }

    let mut distance_matrix = vec![vec![0; target_count + 1]; source_count + 1];
    (1..=source_count).for_each(|i| distance_matrix[i][0] = i);
    (1..=target_count).for_each(|j| distance_matrix[0][j] = j);

    for (i, source_c) in source.chars().enumerate() {
        for (j, target_c) in target.chars().enumerate() {
            let ins = distance_matrix[i][j + 1] + 1;
            let del = distance_matrix[i + 1][j] + 1;
            let sub = distance_matrix[i][j] + (source_c != target_c) as usize;
            distance_matrix[i + 1][j + 1] = min(sub, min(ins, del));
        }
    }

    *distance_matrix.last().and_then(|c| c.last()).unwrap()
}
