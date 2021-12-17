use std::collections::{HashMap, HashSet};

fn build_graph(input: &str) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let v = line.split_once('-').unwrap();
        let cave = graph.entry(v.0.to_string()).or_insert(HashSet::new());
        cave.insert(v.1.to_string());

        let cave = graph.entry(v.1.to_string()).or_insert(HashSet::new());
        cave.insert(v.0.to_string());
    }
    graph
}

fn inner_cave_traversal(
    current_cave: &str,
    graph: &HashMap<String, HashSet<String>>,
    seen_small_cave: &HashSet<String>,
) -> u32 {
    if current_cave == "end" {
        return 1;
    }

    if seen_small_cave.contains(current_cave) || current_cave == "start" {
        return 0;
    }

    let mut new_seen_small_cave = seen_small_cave.clone();
    if current_cave.to_lowercase() == current_cave.to_string() {
        new_seen_small_cave.insert(current_cave.to_string());
    }

    let mut found_paths = 0;
    if let Some(paths) = graph.get(current_cave) {
        for path in paths {
            found_paths += inner_cave_traversal(&path, &graph, &new_seen_small_cave);
        }
    }

    found_paths
}

fn cave_traversal(graph: &HashMap<String, HashSet<String>>) -> u32 {
    let start = graph.get("start").unwrap();
    let mut found_paths = 0;
    for path in start {
        found_paths += inner_cave_traversal(path, &graph, &HashSet::new());
    }

    found_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_cave_traversal() {
        let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
        let graph = build_graph(&input);
        let result = cave_traversal(&graph);

        assert_eq!(10, result);
        let result = cave_traversal(&graph);
    }

    #[test]
    fn second_example_cave_traversal() {
        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
        let graph = build_graph(&input);
        let result = cave_traversal(&graph);

        assert_eq!(19, result);
    }

    #[test]
    fn third_example_cave_traversal() {
        let input = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";
        let graph = build_graph(&input);
        let result = cave_traversal(&graph);

        assert_eq!(226, result);
    }

    #[test]
    fn puzzle_cave_traversal() {
        let input = "TR-start\nxx-JT\nxx-TR\nhc-dd\nab-JT\nhc-end\ndd-JT\nab-dd\nTR-ab\nvh-xx\nhc-JT\nTR-vh\nxx-start\nhc-ME\nvh-dd\nJT-bm\nend-ab\ndd-xx\nend-TR\nhc-TR\nstart-vh";
        let graph = build_graph(&input);
        let result = cave_traversal(&graph);

        assert_eq!(5157, result);
    }
}
