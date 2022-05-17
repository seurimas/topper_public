use crate::powered::*;

pub struct Selector<R> {
    nodes: Vec<Box<dyn PoweredFunction<World = R>>>,
    index: Option<usize>,
}

impl<R> Selector<R> {
    pub fn new(nodes: Vec<Box<dyn PoweredFunction<World = R>>>) -> Self {
        Selector { nodes, index: None }
    }
}

impl<R: 'static> PoweredFunction for Selector<R> {
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
                    PoweredFunctionState::Failed(_) => {
                        // Move on to the next node.
                        running_index += 1;
                    }
                    PoweredFunctionState::InProgress(_) => {
                        // We'll be stepping the current node again.
                        continue;
                    }
                    PoweredFunctionState::Complete(_) => {
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
                return PoweredFunctionState::Failed(gas_left);
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
    fn test_selector_success() {
        let nodes: Vec<Box<dyn PoweredFunction<World = ()>>> = vec![
            Box::new(ConsumeGasFail::new(5)),
            Box::new(ConsumeGasFail::new(5)),
            Box::new(ConsumeGas::new(5)),
        ];
        let mut selector = Selector { nodes, index: None };
        let first_run = selector.resume_with(8, &mut ());
        assert_eq!(
            first_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let second_run = selector.resume_with(8, &mut ());
        assert_eq!(
            second_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let third_run = selector.resume_with(8, &mut ());
        assert_eq!(third_run, PoweredFunctionState::Complete(3));
    }

    #[test]
    fn test_selector_fail() {
        let nodes: Vec<Box<dyn PoweredFunction<World = ()>>> = vec![
            Box::new(ConsumeGasFail::new(5)),
            Box::new(ConsumeGasFail::new(5)),
            Box::new(ConsumeGasFail::new(5)),
        ];
        let mut selector = Selector { nodes, index: None };
        let first_run = selector.resume_with(8, &mut ());
        assert_eq!(
            first_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let second_run = selector.resume_with(8, &mut ());
        assert_eq!(
            second_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let third_run = selector.resume_with(8, &mut ());
        assert_eq!(third_run, PoweredFunctionState::Failed(3));
    }

    #[test]
    fn test_selector_success_through() {
        let nodes: Vec<Box<dyn PoweredFunction<World = ()>>> = vec![
            Box::new(ConsumeGasFail::new(5)),
            Box::new(ConsumeGasFail::new(5)),
            Box::new(ConsumeGas::new(5)),
        ];
        let mut selector = Selector { nodes, index: None };
        let first_run = selector.resume_with(15, &mut ());
        assert_eq!(first_run, PoweredFunctionState::Complete(0));
    }

    #[test]
    fn test_selector_fail_through() {
        let nodes: Vec<Box<dyn PoweredFunction<World = ()>>> = vec![
            Box::new(ConsumeGasFail::new(5)),
            Box::new(ConsumeGasFail::new(5)),
            Box::new(ConsumeGasFail::new(5)),
        ];
        let mut selector = Selector { nodes, index: None };
        let first_run = selector.resume_with(15, &mut ());
        assert_eq!(first_run, PoweredFunctionState::Failed(0));
    }
}
