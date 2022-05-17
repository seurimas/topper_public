use crate::powered::*;

pub struct Sequence<R> {
    nodes: Vec<Box<dyn PoweredFunction<World = R>>>,
    index: Option<usize>,
}

impl<R> Sequence<R> {
    pub fn new(nodes: Vec<Box<dyn PoweredFunction<World = R>>>) -> Self {
        Sequence { nodes, index: None }
    }
}

#[macro_export]
macro_rules! sequence {
    ( $($node:expr),* ) => {
        {
            let mut temp_nodes = Vec::new();
            $(
                temp_nodes.push(Box::new($node));
            )*
            Sequence {
                nodes: temp_nodes,
                index: None,
            }
        }
    };
}

impl<R: 'static> PoweredFunction for Sequence<R> {
    type World = R;
    fn resume_with(
        self: &mut Self,
        mut gas_left: i32,
        parameter: &mut Self::World,
    ) -> PoweredFunctionState {
        let mut running_index = self.index.unwrap_or(0);
        while gas_left >= 0 {
            if let Some(node) = self.nodes.get_mut(running_index) {
                let result = node.resume_with(gas_left, parameter);
                gas_left = result.get_gas_left();
                match result {
                    PoweredFunctionState::Complete(_) => {
                        // Move on to the next node.
                        running_index += 1;
                    }
                    PoweredFunctionState::InProgress(_) => {
                        // We'll be stepping the current node again.
                        continue;
                    }
                    PoweredFunctionState::Failed(_) => {
                        self.index = None;
                        return result;
                    }
                    _ => {
                        // Waiting, NeedsGas
                        self.index = Some(running_index);
                        return result;
                    }
                }
            } else {
                self.index = None;
                return PoweredFunctionState::Complete(gas_left);
            }
        }
        return PoweredFunctionState::InProgress(gas_left);
    }

    fn reset(self: &mut Self, _parameter: &mut Self::World) {
        self.index = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_success() {
        let nodes: Vec<Box<dyn PoweredFunction<World = ()>>> = vec![
            Box::new(ConsumeGas::new(5)),
            Box::new(ConsumeGas::new(5)),
            Box::new(ConsumeGas::new(5)),
        ];
        let mut sequence = Sequence { nodes, index: None };
        let first_run = sequence.resume_with(8, &mut ());
        assert_eq!(
            first_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let second_run = sequence.resume_with(8, &mut ());
        assert_eq!(
            second_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let third_run = sequence.resume_with(8, &mut ());
        assert_eq!(third_run, PoweredFunctionState::Complete(3));
    }

    #[test]
    fn test_sequence_fail() {
        let nodes: Vec<Box<dyn PoweredFunction<World = ()>>> = vec![
            Box::new(ConsumeGas::new(5)),
            Box::new(ConsumeGas::new(5)),
            Box::new(ConsumeGasFail::new(5)),
        ];
        let mut sequence = Sequence { nodes, index: None };
        let first_run = sequence.resume_with(8, &mut ());
        assert_eq!(
            first_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let second_run = sequence.resume_with(8, &mut ());
        assert_eq!(
            second_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let third_run = sequence.resume_with(8, &mut ());
        assert_eq!(third_run, PoweredFunctionState::Failed(3));
    }

    #[test]
    fn test_sequence_succeed_through() {
        let nodes: Vec<Box<dyn PoweredFunction<World = ()>>> = vec![
            Box::new(ConsumeGas::new(5)),
            Box::new(ConsumeGas::new(5)),
            Box::new(ConsumeGas::new(5)),
        ];
        let mut sequence = Sequence { nodes, index: None };
        let first_run = sequence.resume_with(15, &mut ());
        assert_eq!(first_run, PoweredFunctionState::Complete(0));
    }

    #[test]
    fn test_sequence_fail_through() {
        let nodes: Vec<Box<dyn PoweredFunction<World = ()>>> = vec![
            Box::new(ConsumeGas::new(5)),
            Box::new(ConsumeGas::new(5)),
            Box::new(ConsumeGasFail::new(5)),
        ];
        let mut sequence = Sequence { nodes, index: None };
        let first_run = sequence.resume_with(15, &mut ());
        assert_eq!(first_run, PoweredFunctionState::Failed(0));
    }
}
