use crate::quarto::{Piece, State, WinningStatus};
use crate::random::random_action;
use rand::{thread_rng, Rng};
use std::cmp::max;

fn find_winning_place(state: &State) -> Option<(usize, usize)> {
    for (h, w) in state.legal_placements() {
        if state.can_put_then_win(h, w) {
            return Some((h, w));
        }
    }
    None
}

fn playout(state: &mut State) -> f64 {
    match state.get_winning_status() {
        WinningStatus::WIN => 1.0,
        WinningStatus::DRAW => 0.5,
        WinningStatus::NONE => {
            let winning_place = find_winning_place(state);
            if let Some((h, w)) = winning_place {
                state.put_piece(h, w);
                return 1.0;
            }

            let (place, piece) = random_action(state);
            if let Some(p) = place {
                state.put_piece(p.0, p.1);
            }
            match state.get_winning_status() {
                WinningStatus::WIN => 1.0,
                WinningStatus::DRAW => 0.5,
                WinningStatus::NONE => {
                    if let Some(piece) = piece {
                        state.select_piece(piece);
                    }
                    1.0 - playout(state)
                }
                _ => panic!("unreachable error"),
            }
        }
        _ => panic!("unreachable error"),
    }
}

pub fn primitive_monte_carlo_action(
    state: &State,
    playout_number: usize,
) -> (Option<(usize, usize)>, Option<Piece>) {
    let mut legal_places = Vec::new();
    if !state.is_first_turn() {
        legal_places = state.legal_placements();
    }
    let mut legal_pieces = Vec::new();
    if !state.is_last_turn() {
        legal_pieces = state.legal_pieces();
    }

    let mut values = vec![vec![0.0; max(legal_pieces.len(), 1)]; max(legal_places.len(), 1)];
    let mut cnts = vec![vec![0usize; max(legal_pieces.len(), 1)]; max(legal_places.len(), 1)];
    for cnt in 0..playout_number {
        let mut next_state = *state;
        let next_state = &mut next_state;

        if !legal_places.is_empty() {
            let place = legal_places[cnt % legal_places.len()];
            next_state.put_piece(place.0, place.1);
        };
        if !legal_pieces.is_empty() {
            let piece = legal_pieces[cnt % legal_pieces.len()];
            next_state.select_piece(piece);
        };

        values[cnt % max(legal_places.len(), 1)][cnt % max(legal_pieces.len(), 1)] +=
            1.0 - playout(next_state);
        cnts[cnt % max(legal_places.len(), 1)][cnt % max(legal_pieces.len(), 1)] += 1;
    }

    let mut best_action_idx = (usize::MAX, usize::MAX);
    let mut best_score = f64::NEG_INFINITY;
    for place_idx in 0..max(legal_places.len(), 1) {
        for piece_idx in 0..max(legal_pieces.len(), 1) {
            let value_mean = values[place_idx][piece_idx] / cnts[place_idx][piece_idx] as f64;
            if value_mean > best_score {
                best_score = value_mean;
                best_action_idx = (place_idx, piece_idx);
            }
        }
    }
    let place = if legal_places.is_empty() {
        None
    } else {
        Some(legal_places[best_action_idx.0])
    };
    let piece = if legal_pieces.is_empty() {
        None
    } else {
        Some(legal_pieces[best_action_idx.1])
    };
    (place, piece)
}

const C: f64 = 1.0;

const EXPAND_THRESHOLD: i32 = 10;

struct Node {
    state: State,
    child_nodes: Vec<Node>,
    trials: i32,
    cumulative_value: f64,
    put_place: Option<(usize, usize)>,
    selected_piece: Option<Piece>,
}

impl Node {
    pub fn new(state: State) -> Self {
        Node {
            state,
            child_nodes: Vec::new(),
            trials: 0,
            cumulative_value: 0.0,
            put_place: None,
            selected_piece: None,
        }
    }

    pub fn expand(&mut self) {
        self.child_nodes.clear();
        for p in self.state.legal_placements() {
            if self.state.legal_placements().is_empty() {
                self.child_nodes.push(Node::new(self.state.clone()));
                self.child_nodes
                    .last_mut()
                    .unwrap()
                    .state
                    .put_piece(p.0, p.1);
                self.child_nodes.last_mut().unwrap().put_place = Some(p);
            } else {
                for &s in &self.state.legal_pieces() {
                    self.child_nodes.push(Node::new(self.state.clone()));
                    self.child_nodes
                        .last_mut()
                        .unwrap()
                        .state
                        .put_piece(p.0, p.1);
                    self.child_nodes.last_mut().unwrap().put_place = Some(p);
                    self.child_nodes.last_mut().unwrap().state.select_piece(s);
                    self.child_nodes.last_mut().unwrap().selected_piece = Some(s);
                }
            }
        }
    }

    fn next_child_node_idx(&self) -> usize {
        for (i, child_node) in self.child_nodes.iter().enumerate() {
            if child_node.trials == 0 {
                return i;
            }
        }
        let mut trials = 0;
        for child_node in &self.child_nodes {
            trials += child_node.trials;
        }
        let mut best_value = f64::NEG_INFINITY;
        let mut best_action_idx = usize::MAX;
        for i in 0..self.child_nodes.len() {
            let child_node = &self.child_nodes[i];
            let ucb1_value = 1.0 - child_node.cumulative_value / child_node.trials as f64
                + C * (2.0 * (trials as f64).ln() / child_node.trials as f64).sqrt();

            if ucb1_value > best_value {
                best_action_idx = i;
                best_value = ucb1_value;
            }
        }
        best_action_idx
    }

    pub fn evaluate(&mut self) -> f64 {
        if self.state.is_done() {
            let value = match self.state.get_winning_status() {
                WinningStatus::WIN => 0.0,
                _ => 0.5,
            };
            self.trials += 1;
            self.cumulative_value += value;
            return value;
        }

        if self.child_nodes.is_empty() {
            let mut state_copy = self.state.clone();
            let value = playout(&mut state_copy);
            self.trials += 1;
            self.cumulative_value += value;

            if self.trials == EXPAND_THRESHOLD {
                self.expand();
            }
            return value;
        }

        let next_child_idx = self.next_child_node_idx();
        let value = 1.0 - self.child_nodes[next_child_idx].evaluate();
        self.trials += 1;
        self.cumulative_value += value;
        value
    }
}

pub fn mcts_action(
    state: &State,
    playout_number: usize,
) -> (Option<(usize, usize)>, Option<Piece>) {
    if state.is_first_turn() {
        let mut rng = thread_rng();
        let legal_select = state.legal_pieces();
        return (
            None,
            Some(legal_select[rng.gen::<usize>() % legal_select.len()]),
        );
    }

    if state.is_last_turn() {
        let legal_put = state.legal_placements()[0];
        return (Some(legal_put), None);
    }

    let mut root_node = Node::new(*state);
    root_node.expand();

    for _ in 0..playout_number {
        root_node.evaluate();
    }

    let put = state.legal_placements();
    let select = state.legal_pieces();

    let mut best_action_search_number = i32::MIN;
    let mut best_action_put = None;
    let mut best_action_select = None;

    assert_eq!(put.len() * select.len(), root_node.child_nodes.len());

    for i in 0..root_node.child_nodes.len() {
        let trials = root_node.child_nodes[i].trials;
        if trials > best_action_search_number {
            best_action_put = root_node.child_nodes[i].put_place;
            best_action_select = root_node.child_nodes[i].selected_piece;
            best_action_search_number = trials;
        }
    }

    (best_action_put, best_action_select)
}
