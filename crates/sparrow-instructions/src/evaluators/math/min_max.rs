use std::marker::PhantomData;
use std::sync::Arc;

use arrow::array::{ArrayRef, PrimitiveArray};
use arrow::compute::math_op;
use arrow::datatypes::{ArrowNativeTypeOp, ArrowNumericType};
use sparrow_plan::ValueRef;

use crate::{Evaluator, EvaluatorFactory, RuntimeInfo, StaticInfo};

/// Evaluator for ZipMin.
pub(in crate::evaluators) struct ZipMinEvaluator<T: ArrowNumericType> {
    lhs: ValueRef,
    rhs: ValueRef,
    // Use the type parameter and indicate it is invariant.
    _phantom: PhantomData<fn(T) -> T>,
}

impl<T: ArrowNumericType> EvaluatorFactory for ZipMinEvaluator<T>
where
    T::Native: ArrowNativeTypeOp,
{
    fn try_new(info: StaticInfo<'_>) -> anyhow::Result<Box<dyn Evaluator>> {
        let (lhs, rhs) = info.unpack_arguments()?;
        Ok(Box::new(Self {
            lhs,
            rhs,
            _phantom: PhantomData::default(),
        }))
    }
}

impl<T: ArrowNumericType> Evaluator for ZipMinEvaluator<T>
where
    T::Native: ArrowNativeTypeOp,
{
    fn evaluate(&mut self, info: &dyn RuntimeInfo) -> anyhow::Result<ArrayRef> {
        let lhs = info.value(&self.lhs)?.primitive_array()?;
        let rhs = info.value(&self.rhs)?.primitive_array()?;
        let result = zip_min::<T>(lhs.as_ref(), rhs.as_ref())?;
        Ok(result)
    }
}

/// Evaluator for ZipMax.
pub(in crate::evaluators) struct ZipMaxEvaluator<T: ArrowNumericType> {
    lhs: ValueRef,
    rhs: ValueRef,
    // Use the type parameter and indicate it is invariant.
    _phantom: PhantomData<fn(T) -> T>,
}

impl<T: ArrowNumericType> EvaluatorFactory for ZipMaxEvaluator<T>
where
    T::Native: ArrowNativeTypeOp,
{
    fn try_new(info: StaticInfo<'_>) -> anyhow::Result<Box<dyn Evaluator>> {
        let (lhs, rhs) = info.unpack_arguments()?;
        Ok(Box::new(Self {
            lhs,
            rhs,
            _phantom: PhantomData::default(),
        }))
    }
}

impl<T: ArrowNumericType> Evaluator for ZipMaxEvaluator<T>
where
    T::Native: ArrowNativeTypeOp,
{
    fn evaluate(&mut self, info: &dyn RuntimeInfo) -> anyhow::Result<ArrayRef> {
        let lhs = info.value(&self.lhs)?.primitive_array()?;
        let rhs = info.value(&self.rhs)?.primitive_array()?;
        let result = zip_max::<T>(lhs.as_ref(), rhs.as_ref())?;
        Ok(result)
    }
}

/// Return the per-element minimum of two numeric arrays. If a < b, a is
/// returned. Otherwise b.
///
/// Note that this operates on types implementing the PartialOrd trait, which
/// can have possibly surprising behavior when ordering certain floating-point
/// types e.g. NaN. See <https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#tymethod.partial_cmp> for specifics.
///
/// # Errors
/// When passed arrays of differing length.
fn zip_min<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> anyhow::Result<ArrayRef>
where
    T: ArrowNumericType,
    T::Native: ArrowNativeTypeOp,
{
    let result = math_op(left, right, |a, b| if a < b { a } else { b })?;
    Ok(Arc::new(result))
}

/// Return the per-element maximum of two numeric arrays. If a > b, a is
/// returned. Otherwise b.
///
/// Note that this operates on types implementing the PartialOrd trait, which
/// can have possibly surprising behavior when ordering certain floating-point
/// types e.g. NaN. See
/// <https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#tymethod.partial_cmp>
/// for specifics.
///
/// # Errors
/// When passed arrays of differing length.
fn zip_max<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> anyhow::Result<ArrayRef>
where
    T: ArrowNumericType,
    T::Native: ArrowNativeTypeOp,
{
    let result = math_op(left, right, |a, b| if a > b { a } else { b })?;
    Ok(Arc::new(result))
}

#[cfg(test)]
mod tests {
    use arrow::array::{Float32Array, Int32Array};
    use arrow::datatypes::{Float32Type, Int32Type};

    use super::*;

    #[test]
    fn test_zip_min_int32() {
        let left: PrimitiveArray<Int32Type> = Int32Array::from(vec![Some(1), Some(5), None]);
        let right: PrimitiveArray<Int32Type> = Int32Array::from(vec![Some(0), Some(6), None]);
        let actual = zip_min::<Int32Type>(&left, &right).unwrap();
        assert_eq!(
            actual.as_ref(),
            &Int32Array::from(vec![Some(0), Some(5), None])
        );
    }

    #[test]
    fn test_zip_min_float32() {
        let left: PrimitiveArray<Float32Type> =
            Float32Array::from(vec![Some(1.0), Some(-5.1), None]);
        let right: PrimitiveArray<Float32Type> =
            Float32Array::from(vec![Some(0.5), Some(-5.0), None]);
        let actual = zip_min::<Float32Type>(&left, &right).unwrap();
        assert_eq!(
            actual.as_ref(),
            &Float32Array::from(vec![Some(0.5), Some(-5.1), None])
        );
    }

    #[test]
    fn test_zip_max_int32() {
        let left: PrimitiveArray<Int32Type> = Int32Array::from(vec![Some(1), Some(5), None]);
        let right: PrimitiveArray<Int32Type> = Int32Array::from(vec![Some(0), Some(6), None]);
        let actual = zip_max::<Int32Type>(&left, &right).unwrap();
        assert_eq!(
            actual.as_ref(),
            &Int32Array::from(vec![Some(1), Some(6), None])
        );
    }

    #[test]
    fn test_zip_max_float32() {
        let left: PrimitiveArray<Float32Type> =
            Float32Array::from(vec![Some(1.0), Some(-5.1), None]);
        let right: PrimitiveArray<Float32Type> =
            Float32Array::from(vec![Some(0.5), Some(-5.0), None]);
        let actual = zip_max::<Float32Type>(&left, &right).unwrap();
        assert_eq!(
            actual.as_ref(),
            &Float32Array::from(vec![Some(1.0), Some(-5.0), None])
        );
    }
}