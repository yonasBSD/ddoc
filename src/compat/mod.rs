pub mod before_0_11;
pub mod before_0_16;

use crate::*;

/// For support of old ddoc versions (<= 0.11), convert old nav components
/// if the new `body` field is empty
pub fn fix_old_config(config: &mut Config) {
    if config.body.is_empty() {
        config.body = config.old.to_body_composite();
    }
}
