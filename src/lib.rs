#![feature(try_trait_v2, const_destruct)]
use std::{
    marker::Destruct,
    ops::{ControlFlow, FromResidual, Try},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Validation<T, E, W> {
    pub warnings: Vec<W>,
    pub result: Result<T, Vec<E>>,
}

pub struct ValidationResidual<E, W> {
    warnings: Vec<W>,
    errors: Vec<E>,
}

impl<T, E, W> Validation<T, E, W> {
    pub fn ok(self) -> Option<T> {
        self.result.ok()
    }

    pub fn map<U, F>(self, f: F) -> Validation<U, E, W>
    where
        F: FnOnce(T) -> U + Destruct,
    {
        Validation {
            warnings: self.warnings,
            result: self.result.map(f),
        }
    }

    pub fn warn(&mut self, warning: W) -> &mut Self {
        self.warnings.push(warning);
        self
    }

    pub fn warns<I>(&mut self, warnings: I) -> &mut Self
    where
        I: Iterator<Item = W>,
    {
        for warning in warnings {
            self.warnings.push(warning);
        }

        self
    }
}

impl<T, E, W> Try for Validation<T, E, W> {
    type Output = T;

    type Residual = ValidationResidual<E, W>;

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self.result {
            Err(errors) => ControlFlow::Break(ValidationResidual {
                warnings: self.warnings,
                errors,
            }),
            Ok(value) => ControlFlow::Continue(value),
        }
    }

    fn from_output(output: Self::Output) -> Self {
        Self {
            warnings: Vec::new(),
            result: Ok(output),
        }
    }
}

impl<T, E, W> FromResidual for Validation<T, E, W> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        Self {
            warnings: residual.warnings,
            result: Err(residual.errors),
        }
    }
}
