// use crate::board_manipulations;
// use rand::Rng;

// struct Node {
//     // MCTS node
//     // The number of times this node has been visited.
//     visits: f32,
//     // The total value of all the visits to this node.
//     value: f32,
//     // The averate rethrn
//     average_return: f32,
//     tiles: [u32; 16],
//     next_move_depends_on_player: bool,
//     children: Option<Box<Vec<Self>>>,
// }

// impl Node {
//     fn new(tiles: &[u32; 16], next_move_depends_on_player: bool) -> Self {
//         Self {
//             tiles: tiles.clone(),
//             visits: 0.0, // store visits as f32 to avoid casting
//             value: 0.0,
//             average_return: 0.0,
//             next_move_depends_on_player: next_move_depends_on_player,
//             children: None, // using box as the size of Node is not known at compile time (it can reference 1,2,3 children of the same type -> recursion)
//         }
//     }

//     pub fn new_player_move_node(tiles: &[u32; 16]) -> Self {
//         Self::new(tiles, false)
//     }

//     pub fn new_tile_generation_move(tiles: &[u32; 16]) -> Self {
//         Self::new(tiles, true)
//     }

//     pub fn select_best_child(&self) -> &Self {
//         // Select the child with the highest UCB1 value
//         // The UCB1 value is calculated as follows:
//         // UCB1 = average_return + sqrt(2 * ln(visits) / child_visits)
//         // The child with the highest UCB1 value is returned
//         // If the node has not been expanded, it is expanded
//         if self.children.is_none() {
//             panic!("Trying to select a child from a node that has not been expanded.")
//         }
//         if self.next_move_depends_on_player {
//             // return the child with the highest UCB1 value, since the move depends on the player
//             let parent_part = 2.0 * self.visits.ln();
//             &self
//                 .children
//                 .as_ref()
//                 .unwrap()
//                 .iter()
//                 .max_by(|&a, &b| {
//                     let a_score = a.average_return + (parent_part / a.visits).sqrt();
//                     let b_score = b.average_return + (parent_part / b.visits).sqrt();
//                     a_score.partial_cmp(&b_score).unwrap()
//                 })
//                 .unwrap()
//         } else {
//             // return random element from self.children if the move does not depend on the player
//             let index = rand::thread_rng().gen_range(0..self.children.as_ref().unwrap().len());
//             &self.children.as_ref().unwrap()[index]
//         }
//     }

//     pub fn expand(&mut self) {
//         // Expand the node by creating all possible children
//         // depends on the type of the node (player move or tile generation)
//         if self.children.is_some() {
//             panic!("Trying to expand a node that has already been expanded.")
//         }
//         self.children = Some(Box::new(if self.next_move_depends_on_player {
//             board_manipulations::next_tiles_after_player_move(&self.tiles)
//                 .iter()
//                 .map(Node::new_tile_generation_move)
//                 .collect()
//         } else {
//             board_manipulations::next_tiles_after_spawn(&self.tiles)
//                 .iter()
//                 .map(Node::new_player_move_node)
//                 .collect()
//         }));
//     }
//     pub fn run_monte_carlo(&self, iterations: u32) {
//         // Run the monte carlo algorithm for the given number of iterations
//         for _ in 0..iterations {
//             let mut current_node = &mut self;
//             let mut path: Vec<&Node> = Vec::<&Node>::new();
//             // Select the best child until a leaf node is reached, save the path
//             while current_node.children.is_some() {
//                 path.push(current_node);
//                 current_node = current_node.select_best_child();
//             }
//             // Expand the leaf node
//             current_node.expand();
//             // Select a random child
//             let child = current_node.select_best_child();
//             // Simulate the game from the selected child
//             let score = simulate_game(&child.tiles, 100);
//             // Update the value of the child and all its parents
//             let mut current_node = child;
//             for current_node in path.iter_mut().rev() {
//                 current_node.visits += 1.0;
//                 current_node.value += score;
//                 current_node.average_return = current_node.value / current_node.visits;
//             }
//         }
//     }
// }

// fn simulate_game(tiles: &[u32; 16], max_moves: i32) -> f32 {
//     0.0
// }
