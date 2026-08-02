use rand::{Rng, RngExt};

pub trait GameState: Clone {
    type Action: Clone + Copy + Eq;

    /// 現在の局面からの合法手リストを得る
    fn get_legal_actions(&self) -> Vec<Self::Action>;

    /// 手を適用させた局面を返す
    fn apply(&self, action: Self::Action) -> Self;

    /// 手番プレイヤーから見た勝敗(1 or -1)、または終局していなければNoneを得る
    fn get_terminal_value(&self) -> Option<f64>;
}

struct Node<S: GameState> {
    /// 現在局面
    state: S,

    /// どの手でこの局面に来たか
    action: Option<S::Action>,

    /// 訪問回数
    visits: u32,

    /// 手番プレイヤー視点での累積価値
    value_sum: f64,

    /// この局面から遷移可能な局面
    children: Vec<Node<S>>,

    /// まだ試していない手
    untried: Vec<S::Action>,

    /// 終局値のキャッシュ
    terminal: Option<f64>,
}

impl<S: GameState> Node<S> {
    fn new(state: S, action: Option<S::Action>) -> Self {
        let terminal = state.get_terminal_value();
        let untried = if terminal.is_some() {
            Vec::new()
        } else {
            state.get_legal_actions()
        };
        Node {
            state,
            action,
            visits: 0,
            value_sum: 0.0,
            children: Vec::new(),
            untried,
            terminal
        }
    }

    fn is_fully_expanded(&self) -> bool {
        self.untried.is_empty()
    }

    fn is_terminal(&self) -> bool {
        self.terminal.is_some()
    }

    /// 終局までランダムに進め、現在手番プレイヤー視点の値を返す
    fn rollout(&self, rng: &mut impl Rng) -> f64 {
        let mut state = self.state.clone();
        let mut sign = 1.0;

        loop {
            if let Some(v) = state.get_terminal_value() {
                return sign * v;
            }
            let actions = state.get_legal_actions();
            let action = actions[rng.random_range(0..actions.len())];
            state = state.apply(action);
            sign = -sign;
        }
    }

    fn select_child(&self, c: f64) -> usize {
        let ln_parent_visits = (self.visits as f64).ln();
        let mut best_index = 0;
        let mut best_score = f64::NEG_INFINITY;
        for (i, child) in self.children.iter().enumerate() {
            let child_mean = child.value_sum / child.visits as f64;
            let exploit = -child_mean;  // 子のvalue_sumは相手視点であることに注意
            let explore = c * (ln_parent_visits / child.visits as f64).sqrt();
            let score = exploit + explore;
            if score > best_score {
                best_score = score;
                best_index = i;
            }
        }
        best_index
    }

    fn iterate(&mut self, c: f64, rng: &mut impl Rng) -> f64 {
        if let Some(v) = self.terminal {
            self.visits += 1;
            self.value_sum += v;
            return v;
        }

        let value;
        if !self.is_fully_expanded() {
            // Expansion
            let index = rng.random_range(0..self.untried.len());
            let action = self.untried.swap_remove(index);
            let next_state = self.state.apply(action);
            let mut child = Node::new(next_state, Some(action));

            // Simulation
            let child_value = child.rollout(rng);
            child.visits += 1;
            child.value_sum += child_value;
            self.children.push(child);

            value = -child_value;
        } else {
            // Selection
            let best_child_index = self.select_child(c);
            let child_value = self.children[best_child_index].iterate(c, rng);
            value = -child_value;
        }

        // Backpropagation（このノードの分のみ）
        self.visits += 1;
        self.value_sum += value;
        value
    }
}

pub fn mcts_search<S: GameState>(
    root_state: S,
    c: f64,
    rng: &mut impl Rng,
    mut should_stop: impl FnMut(u32) -> bool,
) -> Option<S::Action> {
    let mut root = Node::new(root_state, None);

    let mut iterations: u32 = 0;
    loop {
        root.iterate(c, rng);
        iterations += 1;
        if should_stop(iterations) {
            break;
        }
    }

    root.children.iter()
        .max_by_key(|child| child.visits)
        .and_then(|best| best.action)
}
