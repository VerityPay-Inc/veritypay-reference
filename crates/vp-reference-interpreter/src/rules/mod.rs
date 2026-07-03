//! Evaluation rules implementing normative protocol semantics.

mod vp_rule_0001;
mod vp_rule_0002;
mod vp_rule_0011;

pub use vp_rule_0001::{VpRule0001, VP_RULE_0001_REFERENCE};
pub use vp_rule_0002::{VpRule0002, VP_RULE_0002_REFERENCE};
pub use vp_rule_0011::{VpRule0011, VP_RULE_0011_REFERENCE};
