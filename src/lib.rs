#![feature(test)]

use std::ops::Index;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use ahash::AHashMap;

fn test_naive<'a, 'b, T: Index<usize, Output = PyObject>>(
    py: Python<'b>,
    v: &'a T,
    get_item: impl Fn(Python<'b>, &'a PyObject) -> &'a PyAny,
) -> PyResult<()>
where
    &'a T: IntoIterator<Item = &'a PyObject>,
{
    let hashes: Vec<(isize, &PyAny)> = v
        .into_iter()
        .map(|item| {
            let item = get_item(py, item);
            item.hash().map(|h| (h, item))
        })
        .collect::<Result<Vec<(isize, &PyAny)>, _>>()?;
    for (item_i, (item_h, item)) in hashes.iter().enumerate() {
        for (_, other) in hashes
            .iter()
            .skip(item_i + 1)
            .filter(|(other_h, _)| item_h == other_h)
        {
            if item.eq(other)? {
                return Err(PyErr::new::<PyTypeError, _>("error"));
            }
        }
    }
    Ok(())
}

fn test_sorted<'a, 'b, T: Index<usize, Output = PyObject>>(
    py: Python<'b>,
    data: &'a T,
    get_item: impl Fn(Python<'b>, &'a PyObject) -> &'a PyAny,
) -> PyResult<()>
where
    &'a T: IntoIterator<Item = &'a PyObject>,
{
    let mut vec: Vec<(isize, &PyAny)> = data
        .into_iter()
        .map(|item| {
            let item = get_item(py, item);
            item.hash().map(|h| (h, item))
        })
        .collect::<Result<Vec<_>, _>>()?;
    vec.sort_unstable_by_key(|(item_h, _)| *item_h);
    for (i, (item_h, item)) in vec.iter().enumerate() {
        for (_, other) in vec[i + 1..]
            .iter()
            .take_while(|(other_h, _)| other_h == item_h)
        {
            if item.eq(other)? {
                return Err(PyErr::new::<PyTypeError, _>("error"));
            }
        }
    }
    Ok(())
}

fn test_ahash<
    'a,
    'b,
    T: Index<usize, Output = PyObject>,
    X: ExactSizeIterator<Item = &'a PyObject>,
>(
    py: Python<'b>,
    v: &'a T,
    get_item: impl Fn(Python<'b>, &'a PyObject) -> &'a PyAny,
) -> PyResult<()>
where
    &'a T: IntoIterator<Item = &'a PyObject, IntoIter = X>,
{
    let iter = v.into_iter();
    let mut set: AHashMap<isize, (&PyAny, Vec<&PyAny>)> = AHashMap::with_capacity(iter.len());
    for item in v.into_iter() {
        let item = get_item(py, item);
        let hash = item.hash()?;
        if let Some((head, bucket)) = set.get_mut(&hash) {
            if head.eq(item)? {
                return Err(PyErr::new::<PyTypeError, _>("error"));
            }
            for other in bucket.iter() {
                if other.eq(item)? {
                    return Err(PyErr::new::<PyTypeError, _>("error"));
                }
            }
            bucket.push(item);
        } else {
            set.insert(hash, (item, Vec::new()));
        }
    }
    Ok(())
}

fn get_test_input(py: Python<'_>, size: usize, modu: usize) -> (Vec<PyObject>, Vec<PyObject>) {
    let module = PyModule::from_code(
        py,
        &format!(
            "
class A:
    def __init__(self, x):
        self.x = x
    def __hash__(self):
        return self.x % {modu}
    def __eq__(self, other):
        return self.x == other.x
def valid():
    return [A(i) for i in range({size})]
def invalid():
    l = valid()
    l[int(len(l) / 2)] = l[-1]
    return l
            ",
        ),
        "",
        "",
    )
    .unwrap();
    (
        module
            .getattr("valid")
            .unwrap()
            .call0()
            .unwrap()
            .extract()
            .unwrap(),
        module
            .getattr("invalid")
            .unwrap()
            .call0()
            .unwrap()
            .extract()
            .unwrap(),
    )
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::*;
    use test::{black_box, Bencher};

    macro_rules! bench_unique {
        ($name:ident, $target:ident, $size:literal, $modu:expr) => {
            #[bench]
            fn $name(bench: &mut Bencher) {
                Python::with_gil(|py| {
                    let (valid, invalid) = get_test_input(py, $size, $modu);
                    test_naive(py, &valid, |py, i| i.as_ref(py)).unwrap();
                    test_naive(py, &invalid, |py, i| i.as_ref(py)).unwrap_err();
                    bench.iter(|| black_box($target(py, black_box(&valid), |py, i| i.as_ref(py))));
                })
            }
        };
    }

    macro_rules! bench_unique_target {
        ($name:ident, $target:ident) => {
            mod $name {
                use super::*;

                bench_unique!(bench_10, $target, 10, usize::MAX);
                bench_unique!(bench_100, $target, 100, usize::MAX);
                bench_unique!(bench_10_000, $target, 10_000, usize::MAX);
                bench_unique!(bench_bad_100, $target, 100, 5);
                bench_unique!(bench_bad_10_000, $target, 10_000, 500);
            }
        };
    }

    bench_unique_target!(naive, test_naive);
    bench_unique_target!(ahash, test_ahash);
    bench_unique_target!(sorted, test_sorted);
}
