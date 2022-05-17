use crate::powered::*;

pub struct Repeat<R> {
    node: Box<dyn PoweredFunction<World = R>>,
    runs: usize,
    runs_left: usize,
}

impl<R> Repeat<R> {
    pub fn new(node: Box<dyn PoweredFunction<World = R>>, runs: usize) -> Self {
        Repeat {
            node,
            runs,
            runs_left: runs,
        }
    }
}

impl<R: 'static> PoweredFunction for Repeat<R> {
    type World = R;
    fn resume_with(
        self: &mut Self,
        mut gas_left: i32,
        parameter: &mut Self::World,
    ) -> PoweredFunctionState {
        while self.runs_left > 0 {
            let result = self.node.resume_with(gas_left, parameter);
            gas_left = result.get_gas_left();
            match result {
                PoweredFunctionState::Failed(_) => {
                    self.runs_left = self.runs;
                    return result;
                }
                PoweredFunctionState::Complete(_) => {
                    self.runs_left -= 1;
                }
                PoweredFunctionState::InProgress(_) => {
                    // We'll be stepping the current node again.
                    continue;
                }
                _ => {
                    // Waiting, NeedsGas
                    return result;
                }
            }
        }
        self.runs_left = self.runs;
        return PoweredFunctionState::Complete(gas_left);
    }

    fn reset(self: &mut Self, _parameter: &mut Self::World) {
        self.runs_left = self.runs;
    }
}
pub struct RepeatUntilFail<R> {
    node: Box<dyn PoweredFunction<World = R>>,
}

impl<R> RepeatUntilFail<R> {
    pub fn new(node: Box<dyn PoweredFunction<World = R>>) -> Self {
        RepeatUntilFail { node }
    }
}

impl<R: 'static> PoweredFunction for RepeatUntilFail<R> {
    type World = R;
    fn resume_with(
        self: &mut Self,
        mut gas_left: i32,
        parameter: &mut Self::World,
    ) -> PoweredFunctionState {
        loop {
            let result = self.node.resume_with(gas_left, parameter);
            gas_left = result.get_gas_left();
            match result {
                PoweredFunctionState::Failed(_) => {
                    return PoweredFunctionState::Complete(gas_left);
                }
                PoweredFunctionState::Complete(_) | PoweredFunctionState::InProgress(_) => {
                    // We'll be stepping the current node again.
                    continue;
                }
                _ => {
                    // Waiting, NeedsGas
                    return result;
                }
            }
        }
    }

    fn reset(self: &mut Self, _parameter: &mut Self::World) {
        // Nothing to do.
    }
}

#[cfg(test)]
mod tests {
    use crate::func_complete;

    use super::*;

    #[test]
    fn test_repeat_success() {
        let mut repeat = Repeat {
            node: Box::new(ConsumeGas::new(5)),
            runs: 3,
            runs_left: 3,
        };
        let first_run = repeat.resume_with(8, &mut ());
        assert_eq!(
            first_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let second_run = repeat.resume_with(8, &mut ());
        assert_eq!(
            second_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let third_run = repeat.resume_with(8, &mut ());
        assert_eq!(third_run, PoweredFunctionState::Complete(3));
    }

    #[test]
    fn test_repeat_through() {
        let mut repeat = Repeat {
            node: Box::new(ConsumeGas::new(5)),
            runs: 3,
            runs_left: 3,
        };
        let first_run = repeat.resume_with(15, &mut ());
        assert_eq!(first_run, PoweredFunctionState::Complete(0));
    }

    #[test]
    fn test_repeat_until_fail_low_gas() {
        let mut repeat = RepeatUntilFail {
            node: Box::new(ConsumeGas::new(5)),
        };
        let first_run = repeat.resume_with(15, &mut ());
        assert_eq!(
            first_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 0,
                gas_needed: 5
            }
        );
    }

    #[test]
    fn test_repeat_until_fail_immediate() {
        let mut repeat = RepeatUntilFail {
            node: Box::new(ConsumeGasFail::new(5)),
        };
        let first_run = repeat.resume_with(3, &mut ());
        assert_eq!(
            first_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 3,
                gas_needed: 5
            }
        );
        let second_run = repeat.resume_with(5, &mut ());
        assert_eq!(second_run, func_complete!(0),);
    }
}
