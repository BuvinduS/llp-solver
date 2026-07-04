//! Turn user-facing input (raw constraints + variable-restriction flags)
//! into the internal `Constraint` list the geometry engine works with.

use crate::types::{Constraint, Operator};

/// Build the full constraint list used internally: the user's own
/// constraints, plus synthetic `x >= 0` / `y >= 0` constraints if those
/// restrictions are enabled.
///
/// Doing this expansion in one place means the rest of the solver never
/// has to special-case "is this the x>=0 constraint" -- by the time the
/// geometry code sees the list, every entry is just an ordinary
/// constraint.
pub fn build_constraint_list(
    user_constraints: &[Constraint],
    restrict_x_nonneg: bool,
    restrict_y_nonneg: bool,
) -> Vec<Constraint> {
    // `.to_vec()` makes an owned copy of the slice's contents -- we don't
    // want to mutate the caller's original list, so we build a new one.
    let mut all = user_constraints.to_vec();

    if restrict_x_nonneg {
        all.push(Constraint {
            a: 1.0,
            b: 0.0,
            op: Operator::Ge,
            c: 0.0,
            label: Some("x >= 0".to_string()),
        });
    }
    if restrict_y_nonneg {
        all.push(Constraint {
            a: 0.0,
            b: 1.0,
            op: Operator::Ge,
            c: 0.0,
            label: Some("y >= 0".to_string()),
        });
    }

    all
}
