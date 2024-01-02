// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::super::{
    error::InvalidInput::{self, InvalidErrorBudget},
    modeling::{ErrorBudget, PhysicalQubit, ProtocolSpecification},
    LogicalResourceCounts,
};

use super::{tfactory::TFactoryDistillationUnitSpecifications, Constraints};

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct JobParams {
    #[serde(default)]
    qec_scheme: ProtocolSpecification,

    #[serde(default)]
    error_budget: ErrorBudgetSpecification,

    #[serde(default)]
    qubit_params: Rc<PhysicalQubit>,

    #[serde(default, skip_serializing_if = "Constraints::is_default")]
    constraints: Constraints,

    #[serde(default, skip_serializing_if = "Profiling::is_default")]
    profiling: Profiling,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    distillation_unit_specifications: TFactoryDistillationUnitSpecifications,
}

impl JobParams {
    #[must_use]
    #[inline]
    pub fn qec_scheme(&self) -> &ProtocolSpecification {
        &self.qec_scheme
    }

    #[must_use]
    #[inline]
    pub fn qec_scheme_mut(&mut self) -> &mut ProtocolSpecification {
        &mut self.qec_scheme
    }

    #[must_use]
    #[inline]
    pub fn error_budget(&self) -> &ErrorBudgetSpecification {
        &self.error_budget
    }

    #[must_use]
    #[inline]
    pub fn qubit_params(&self) -> &Rc<PhysicalQubit> {
        &self.qubit_params
    }

    #[must_use]
    #[inline]
    pub fn constraints(&self) -> &Constraints {
        &self.constraints
    }

    #[must_use]
    #[inline]
    pub fn distillation_unit_specifications(&self) -> &TFactoryDistillationUnitSpecifications {
        &self.distillation_unit_specifications
    }
}

#[derive(Serialize, Deserialize, Default, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Profiling {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_stack_depth: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_functions: Option<bool>,
}

impl Profiling {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(
    untagged,
    rename_all(serialize = "camelCase", deserialize = "camelCase")
)]
pub enum ErrorBudgetSpecification {
    Total(f64),
    #[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
    Parts {
        logical: f64,
        t_states: f64,
        rotations: f64,
    },
}

impl ErrorBudgetSpecification {
    pub fn total(&self) -> f64 {
        match *self {
            ErrorBudgetSpecification::Total(total) => total,
            ErrorBudgetSpecification::Parts {
                logical,
                t_states,
                rotations,
            } => logical + t_states + rotations,
        }
    }

    pub fn partitioning(
        &self,
        counts: &LogicalResourceCounts,
    ) -> core::result::Result<ErrorBudget, InvalidInput> {
        let has_tgates = counts.t_count > 0
            || counts.ccz_count > 0
            || counts.ccix_count > 0
            || counts.rotation_count > 0;
        let has_rotations = counts.rotation_count > 0;

        match *self {
            ErrorBudgetSpecification::Total(total) => {
                if total <= 0.0 || total >= 1.0 {
                    return Err(InvalidErrorBudget(total));
                }

                Ok(match (has_tgates, has_rotations) {
                    (true, true) => ErrorBudget::new(total / 3.0, total / 3.0, total / 3.0),
                    (true, false) => ErrorBudget::new(total / 2.0, total / 2.0, 0.0),
                    (false, false) => ErrorBudget::new(total, 0.0, 0.0),
                    _ => unreachable!("rotations require T gates"),
                })
            }
            ErrorBudgetSpecification::Parts {
                logical,
                t_states: tstates,
                rotations,
            } => {
                let total = logical + tstates + rotations;
                if total >= 1.0 {
                    return Err(InvalidErrorBudget(total));
                }

                if logical <= 0.0 {
                    return Err(InvalidErrorBudget(logical));
                }

                if tstates < 0.0 || (has_tgates && tstates == 0.0) {
                    return Err(InvalidErrorBudget(tstates));
                }

                if rotations < 0.0 || (has_rotations && rotations == 0.0) {
                    return Err(InvalidErrorBudget(rotations));
                }

                Ok(ErrorBudget::new(logical, tstates, rotations))
            }
        }
    }
}

impl Default for ErrorBudgetSpecification {
    fn default() -> Self {
        Self::Total(1e-3)
    }
}