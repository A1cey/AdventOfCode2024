use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    fs,
};

pub fn run() {
    let input = fs::read_to_string("src/input5.txt").unwrap();

    let (rules, updates_str) = input.split_once("\r\n\r\n").unwrap();

    let mut orderings: HashMap<usize, HashSet<usize>> = HashMap::new();

    rules
        .lines()
        .map(|rule| rule.split_once("|").unwrap())
        .for_each(|(first_num, second_num)| {
            orderings
                .entry(second_num.parse().unwrap())
                .or_default()
                .insert(first_num.parse().unwrap());
        });

    let mut updates = updates_str
        .lines()
        .map(|update| {
            update
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum1 = updates
        .iter()
        .filter(|update| update.is_sorted_by(|a, b| orderings[b].contains(a)))
        .map(|update| update[update.len() / 2])
        .sum::<usize>();

    let sum2 = updates
        .iter_mut()
        .filter(|update| !update.is_sorted_by(|a, b| orderings[b].contains(a)))
        .map(|update| {
            update.sort_by(|a, b| orderings[b].contains(a).cmp(&true));
            update[update.len() / 2]
        })
        .sum::<usize>();

    println!("Sum of the middle page numbers of the sorted updates: {sum1}");
    println!("Sum of the middle page numbers of the unsorted updates after sorting: {sum2}");
    println!();
}

// issue ridden :)

// pub fn run() {
//     println!("Day 5:");

//     let input = fs::read_to_string("src/input5.txt").unwrap();

//     let divider = input.find("\r\n\r\n").unwrap();
//     let rules: Vec<&str> = input[0..divider].lines().collect();
//     let updates: Vec<Vec<i32>> = input[divider..]
//         .lines()
//         .filter(|line| !line.is_empty())
//         .map(|update| update.split(",").map(|s| s.parse().unwrap()).collect())
//         .collect();

//     let ordering = create_page_ordering(&rules);

//     let sum: i32 = updates
//         .iter()
//         .filter(|update| is_update_valid_order(update, &ordering))
//         .map(|update| update[update.len() / 2])
//         .sum();

//     println!("Sum of middle page numbers: {sum}");
//     println!();
// }

// fn is_update_valid_order(update: &[i32], ordering: &HashMap<i32, HashSet<i32>>) -> bool {
//     for i in 0..update.len() {
//         for j in i + 1..update.len() {
//             if ordering
//                 .get(&update[i])
//                 .map_or(false, |blocked| blocked.contains(&update[j]))
//             {
//                 return false;
//             }
//         }
//     }
//     true
// }

// fn create_page_ordering(rules: &[&str]) -> HashMap<i32, HashSet<i32>> {
//     let mut ordering: HashMap<i32, HashSet<i32>> = HashMap::new();

//     for rule in rules {
//         let nums: Vec<i32> = rule.split("|").map(|s| s.parse().unwrap()).collect();
//         let (before, after) = (nums[0], nums[1]);

//         ordering.entry(before).or_default().insert(after);
//     }
//     ordering
// }

// // Not working due to cycles
// fn create_page_ordering_graph(
//     rules: &Vec<&str>,
// ) -> (HashMap<i32, HashSet<i32>>, HashMap<i32, usize>) {
//     let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
//     let mut in_degree: HashMap<i32, usize> = HashMap::new();

//     for rule in rules {
//         let nums: Vec<i32> = rule.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
//         let first_num = nums[0];
//         let second_num = nums[1];

//         graph
//             .entry(first_num)
//             .or_insert_with(HashSet::new)
//             .insert(second_num);

//         in_degree.entry(first_num).or_insert(0);
//         *in_degree.entry(second_num).or_insert(0) += 1;
//     }

//     (graph, in_degree)
// }

// // Kahn's algorithm
// fn topological_sort(
//     graph: HashMap<i32, HashSet<i32>>,
//     mut in_degree: HashMap<i32, usize>,
// ) -> Option<Vec<i32>> {
//     let mut result = Vec::new();
//     let mut queue = VecDeque::new();

//     // enque all nodes with 0 in_degree -> Start nodes
//     in_degree
//         .iter()
//         .filter(|(_, &degree)| degree == 0)
//         .for_each(|(&node, _)| queue.push_back(node));

//     while let Some(node) = queue.pop_front() {
//         result.push(node);

//         if let Some(child_nodes) = graph.get(&node) {
//             child_nodes.iter().for_each(|&child_node| {
//                 // decrement degree of child nodes
//                 if let Some(degree) = in_degree.get_mut(&child_node) {
//                     *degree -= 1;
//                     // push child nodes with no parent degrees to queue
//                     if *degree == 0 {
//                         queue.push_back(child_node);
//                     }
//                 }
//             });
//         }
//     }

//     if result.len() != in_degree.len() {
//         println!("A cycle was detected in the graph.");
//         return None;
//     }

//     Some(result)
// }

// fn create_page_ordering(rules: &Vec<&str>) -> Option<Vec<i32>> {
//     let (graph, in_degree) = create_page_ordering_graph(rules);
//     topological_sort(graph, in_degree)
// }

// not working with input

// fn is_update_valid_order(update: &Vec<i32>, ordering: &Vec<i32>) -> bool {
//     let mut order_idx = 0;
//     update.iter().all(|num| {
//         let new_order_idx = ordering.iter().position(|x| x == num).unwrap();
//         if order_idx > new_order_idx {
//             false
//         } else {
//             order_idx = new_order_idx;
//             true
//         }
//     })
// }

// fn create_page_ordering_old(rules: &Vec<&str>) -> Vec<i32> {
//     let mut ordering: Vec<i32> = vec![];

//     rules.iter().for_each(|rule| {
//         let nums: Vec<i32> = rule.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
//         let first_num = nums[0];
//         let second_num = nums[1];

//         let first_num_idx = ordering.iter().position(|x| x == &first_num);
//         let second_num_idx = ordering.iter().position(|x| x == &second_num);

//         match first_num_idx {
//             Some(idx) => match second_num_idx {
//                 Some(idx2) => {
//                     if idx > idx2 {
//                         ordering.remove(idx);
//                         ordering.insert(idx2, first_num);
//                     }
//                 }
//                 None => {
//                     if idx == ordering.len() - 1 {
//                         ordering.push(second_num);
//                     } else {
//                         ordering.insert(idx + 1, second_num);
//                     }
//                 }
//             },
//             None => match second_num_idx {
//                 Some(idx2) => ordering.insert(idx2, first_num),
//                 None => {
//                     ordering.push(first_num);
//                     ordering.push(second_num);
//                 }
//             },
//         }
//     });

//     println!("Ordering {:?}", ordering);

//     ordering
// }
