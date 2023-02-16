use anyhow::Context;

use crate::functions::time_domain_check::TimeDomainCheck;
use crate::functions::{Implementation, Pushdown, Registry};

/// The `is_new` pattern used for basic aggregations.
const AGGREGATION_IS_NEW: &str = "(logical_or ?window_is_new ?input_is_new)";

pub(super) fn register(registry: &mut Registry) {
    registry
        .register("count_if(input: any, window: window = null) -> u32")
        .with_dfg_signature(
            "count_if(input: any, window: window = null, duration: i64 = null) -> u32",
        )
        .with_implementation(Implementation::new_pattern(&format!(
            "(count_if ({}) ({}) ({}))",
            "transform (if ?input_is_new ?input_value) (merge_join ?input_op ?window_op)",
            "?window_value",
            "?duration_value",
        )))
        .with_is_new(Implementation::new_pattern(AGGREGATION_IS_NEW))
        .with_time_domain_check(TimeDomainCheck::Aggregation);

    registry
        .register("count(input: any, window: window = null) -> u32")
        .with_dfg_signature(
            "count_if(input: any, window: window = null, duration: i64 = null) -> u32",
        )
        .with_implementation(Implementation::new_pattern(&format!(
            "(count_if ({}) ({}) ({}))",
            "transform (is_valid (if ?input_is_new ?input_value)) (merge_join ?input_op \
             ?window_op)",
            "?window_value",
            "?duration_value",
        )))
        .with_is_new(Implementation::new_pattern(AGGREGATION_IS_NEW))
        .with_time_domain_check(TimeDomainCheck::Aggregation);

    registry
        .register("sum(input: number, window: window = null) -> number")
        .with_dfg_signature(
            "sum(input: number, window: window = null, duration: i64 = null) -> number",
        )
        .with_implementation(Implementation::new_pattern(&format!(
            "(sum ({}) ({}) ({}))",
            "transform (if ?input_is_new ?input_value) (merge_join ?input_op ?window_op)",
            "?window_value",
            "?duration_value"
        )))
        .with_is_new(Implementation::new_pattern(AGGREGATION_IS_NEW))
        .with_time_domain_check(TimeDomainCheck::Aggregation);

    registry
        .register("min(input: ordered, window: window = null) -> ordered")
        .with_dfg_signature(
            "min(input: ordered, window: window = null, duration: i64 = null) -> ordered",
        )
        .with_implementation(Implementation::new_pattern(&format!(
            "(min ({}) ({}) ({}))",
            "transform (if ?input_is_new ?input_value) (merge_join ?input_op ?window_op)",
            "?window_value",
            "?duration_value"
        )))
        .with_is_new(Implementation::new_pattern(AGGREGATION_IS_NEW))
        .with_time_domain_check(TimeDomainCheck::Aggregation);

    registry
        .register("max(input: ordered, window: window = null) -> ordered")
        .with_dfg_signature(
            "max(input: ordered, window: window = null, duration: i64 = null) -> ordered",
        )
        .with_implementation(Implementation::new_pattern(&format!(
            "(max ({}) ({}) ({}))",
            "transform (if ?input_is_new ?input_value) (merge_join ?input_op ?window_op)",
            "?window_value",
            "?duration_value"
        )))
        .with_is_new(Implementation::new_pattern(AGGREGATION_IS_NEW))
        .with_time_domain_check(TimeDomainCheck::Aggregation);

    registry
        .register("mean(input: number, window: window = null) -> f64")
        .with_dfg_signature(
            "mean(input: number, window: window = null, duration: i64 = null) -> f64",
        )
        .with_implementation(Implementation::new_pattern(&format!(
            "(mean ({}) ({}) ({}))",
            "transform (if ?input_is_new ?input_value) (merge_join ?input_op ?window_op)",
            "?window_value",
            "?duration_value"
        )))
        .with_is_new(Implementation::new_pattern(AGGREGATION_IS_NEW))
        .with_time_domain_check(TimeDomainCheck::Aggregation);

    registry
        .register("variance(input: number, window: window = null) -> f64")
        .with_dfg_signature(
            "variance(input: number, window: window = null, duration: i64 = null) -> f64",
        )
        .with_implementation(Implementation::new_pattern(&format!(
            "(variance ({}) ({}) ({}))",
            "transform (if ?input_is_new ?input_value) (merge_join ?input_op ?window_op)",
            "?window_value",
            "?duration_value"
        )))
        .with_is_new(Implementation::new_pattern(AGGREGATION_IS_NEW))
        .with_time_domain_check(TimeDomainCheck::Aggregation);

    registry
        .register("stddev(input: number, window: window = null) -> f64")
        .with_dfg_signature(
            "stddev(input: number, window: window = null, duration: i64 = null) -> f64",
        )
        .with_implementation(Implementation::new_pattern(&format!(
            "(powf (variance ({}) ({}) ({})) 0.5f64)",
            "transform (if ?input_is_new ?input_value) (merge_join ?input_op ?window_op)",
            "?window_value",
            "?duration_value"
        )))
        .with_time_domain_check(TimeDomainCheck::Aggregation);

    registry
        .register("last(input: any, window: window = null) -> any")
        .with_dfg_signature("last(input: any, window: window = null, duration: i64 = null) -> any")
        .with_implementation(Implementation::Pushdown(Box::new(
            Pushdown::try_new(
                0,
                &format!(
                    "(last ({}) ({}) ({}))",
                    "transform (if ?is_new ?input_value) (merge_join ?op ?window_op)",
                    "?window_value",
                    "?duration_value"
                ),
                // The per-field pattern produces the last value of the field.
                // The outer if and last is handling the case where the latest *record*
                // contained a null value for the field, by only using the last value of
                // the field if the record is new and valid and the input field is valid in
                // that record.
                &format!(
                    "(if (last ({}) ({}) ({})) ?recurse_on_input_field)",
                    "transform (if (logical_and ?is_new (is_valid ?input_record)) (is_valid \
                     ?input_field)) (merge_join ?op ?window_op)",
                    "?window_value",
                    "?duration_value"
                ),
                // The result pattern treats the resulting record as `null` if there haven't
                // been any new non-null records observed. Eg., requires the count to be > 0.
                &format!(
                    "(if (gt (count_if ({}) ({}) ({})) 0u32) ?result_record)",
                    "transform (logical_and ?is_new (is_valid ?input_record)) (merge_join ?op \
                     ?window_op)",
                    "?window_value",
                    "?duration_value"
                ),
            )
            .context("last")
            .unwrap(),
        )))
        .with_is_new(Implementation::new_pattern(AGGREGATION_IS_NEW))
        .with_time_domain_check(TimeDomainCheck::Aggregation);

    registry
        .register("first(input: any, window: window = null) -> any")
        .with_dfg_signature("first(input: any, window: window = null, duration: i64 = null) -> any")
        .with_implementation(Implementation::Pushdown(Box::new(
            Pushdown::try_new(
                0,
                &format!(
                    "(first({}) ({}) ({}))",
                    "transform (if ?is_new ?input_value) (merge_join ?op ?window_op)",
                    "?window_value",
                    "?duration_value"
                ),
                // The per-field pattern produces the last value of the field.
                // The outer if and last is handling the case where the latest *record*
                // contained a null value for the field, by only using the last value of
                // the field if the record is new and valid and the input field is valid in
                // that record.
                &format!(
                    "(if (first ({}) ({}) ({})) ?recurse_on_input_field)",
                    "transform (if (logical_and ?is_new (is_valid ?input_record)) (is_valid \
                     ?input_field)) (merge_join ?op ?window_op)",
                    "?window_value",
                    "?duration_value"
                ),
                // The result pattern treats the resulting record as `null` if there haven't
                // been any new non-null records observed. Eg., requires the count to be > 0.
                &format!(
                    "(if (gt (count_if ({}) ({}) ({})) 0u32) ?result_record)",
                    "transform (logical_and ?is_new (is_valid ?input_record)) (merge_join ?op \
                     ?window_op)",
                    "?window_value",
                    "?duration_value"
                ),
            )
            .context("first")
            .unwrap(),
        )))
        .with_is_new(Implementation::new_pattern(AGGREGATION_IS_NEW))
        .with_time_domain_check(TimeDomainCheck::Aggregation);
}