use metrics::counter;
use vector_core::internal_event::InternalEvent;

use vector_common::internal_event::{error_stage, error_type};

#[derive(Debug, Copy, Clone)]
pub struct VrlConditionExecutionError<'a> {
    pub error: &'a str,
}

impl<'a> InternalEvent for VrlConditionExecutionError<'a> {
    fn emit(self) {
        error!(
            message = "VRL condition execution failed.",
            error = %self.error,
            error_type = error_type::SCRIPT_FAILED,
            stage = error_stage::PROCESSING,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "error_type" => error_type::SCRIPT_FAILED,
            "stage" => error_stage::PROCESSING,
        );
    }
}
