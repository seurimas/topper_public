use std::marker::PhantomData;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PoweredFunctionState {
    // The powered function completed some work, further work may be completed for more gas.
    InProgress(i32),
    // The powered function is waiting for something.
    Waiting(i32),
    // The powered function knows it needs more gas.
    NeedsGas { gas_left: i32, gas_needed: i32 },
    // The powered function failed to complete all work (bad state or negative result).
    Failed(i32),
    // The powered function completed all work.
    Complete(i32),
}

impl PoweredFunctionState {
    pub fn get_gas_left(&self) -> i32 {
        match self {
            PoweredFunctionState::InProgress(gas_left)
            | PoweredFunctionState::Waiting(gas_left)
            | PoweredFunctionState::Failed(gas_left)
            | PoweredFunctionState::Complete(gas_left)
            | PoweredFunctionState::NeedsGas { gas_left, .. } => *gas_left,
        }
    }
}

#[macro_export]
macro_rules! use_gas {
    ( $gas_var:ident, $gas_used:expr ) => {
        let temp_gas_used = $gas_used as i32;
        if $gas_var < temp_gas_used {
            return PoweredFunctionState::NeedsGas {
                gas_left: $gas_var,
                gas_needed: temp_gas_used,
            };
        }
        $gas_var -= temp_gas_used;
    };
}

#[macro_export]
macro_rules! func_wait {
    ($gas_left:expr) => {
        PoweredFunctionState::Waiting($gas_left)
    };
}

#[macro_export]
macro_rules! func_progress {
    ($gas_left:expr) => {
        PoweredFunctionState::InProgress($gas_left)
    };
}

#[macro_export]
macro_rules! func_fail {
    ($gas_left:expr) => {
        PoweredFunctionState::Failed($gas_left)
    };
}

#[macro_export]
macro_rules! func_complete {
    ($gas_left:expr) => {
        PoweredFunctionState::Complete($gas_left)
    };
}

pub trait PoweredFunction {
    type World: 'static;
    fn resume_with(
        self: &mut Self,
        gas_left: i32,
        parameter: &mut Self::World,
    ) -> PoweredFunctionState;
    fn reset(self: &mut Self, parameter: &mut Self::World);
}

pub struct ConsumeGas<R>(pub i32, pub PhantomData<R>);

impl<R> ConsumeGas<R> {
    pub fn new(gas: i32) -> Self {
        ConsumeGas(gas, PhantomData)
    }
}

impl<R: 'static> PoweredFunction for ConsumeGas<R> {
    type World = R;
    fn resume_with(
        self: &mut Self,
        mut gas_left: i32,
        _param: &mut Self::World,
    ) -> PoweredFunctionState {
        use_gas!(gas_left, self.0);
        func_complete!(gas_left)
    }

    fn reset(self: &mut Self, parameter: &mut Self::World) {
        // No state.
    }
}

pub struct ConsumeGasFail<R>(pub i32, pub PhantomData<R>);

impl<R> ConsumeGasFail<R> {
    pub fn new(gas: i32) -> Self {
        ConsumeGasFail(gas, PhantomData)
    }
}

impl<R: 'static> PoweredFunction for ConsumeGasFail<R> {
    type World = R;
    fn resume_with(
        self: &mut Self,
        mut gas_left: i32,
        _param: &mut Self::World,
    ) -> PoweredFunctionState {
        use_gas!(gas_left, self.0);
        func_fail!(gas_left)
    }

    fn reset(self: &mut Self, parameter: &mut Self::World) {
        // No state.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Example {
        Fresh,
        Sorted,
        Reversed,
    }

    impl PoweredFunction for Example {
        type World = Vec<usize>;
        fn resume_with(
            self: &mut Self,
            mut gas_left: i32,
            parameter: &mut Self::World,
        ) -> PoweredFunctionState {
            match self {
                Example::Fresh => {
                    use_gas!(gas_left, parameter.len() * parameter.len());
                    *self = Example::Sorted;
                    func_progress!(gas_left)
                }
                Example::Sorted => {
                    use_gas!(gas_left, parameter.len());
                    *self = Example::Reversed;
                    func_progress!(gas_left)
                }
                Example::Reversed => {
                    use_gas!(gas_left, 1);
                    *self = Example::Fresh;
                    func_complete!(gas_left)
                }
                _ => panic!(),
            }
        }

        fn reset(self: &mut Self, parameter: &mut Self::World) {
            // No state.
        }
    }

    #[test]
    fn example_00() {
        let mut vec = vec![8, 6, 7, 5, 3, 0, 9];
        let mut powered_func = Example::Fresh;
        let first_run = powered_func.resume_with(50, &mut vec);
        assert_eq!(first_run, func_progress!(1),);
        assert_eq!(powered_func, Example::Sorted);
        let second_run = powered_func.resume_with(1, &mut vec);
        assert_eq!(
            second_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 1,
                gas_needed: 7
            }
        );
        assert_eq!(powered_func, Example::Sorted);
        let third_run = powered_func.resume_with(7, &mut vec);
        assert_eq!(third_run, func_progress!(0),);
        assert_eq!(powered_func, Example::Reversed);
        let fourth_run = powered_func.resume_with(0, &mut vec);
        assert_eq!(
            fourth_run,
            PoweredFunctionState::NeedsGas {
                gas_left: 0,
                gas_needed: 1
            }
        );
        assert_eq!(powered_func, Example::Reversed);
        let fifth_run = powered_func.resume_with(1, &mut vec);
        assert_eq!(fifth_run, func_complete!(0),);
        assert_eq!(powered_func, Example::Fresh);
    }
}
