#![allow(non_snake_case)] // A lot of linear algebra in this file where we want capital matrices


use faer::{Index, SimpleEntity, sparse::{SymbolicSparseColMat, SparseColMatRef, SparseColMat}};

/*
pub fn reshape<I: Index, E: SimpleEntity>(A: SparseColMatRef<'_, I, E>,
                                               (m, n): (I, I)) -> SparseColMat<I, E> {
    //! Reshape A into (m,n) in Fortran (column-major) order.
    let oldn: I = A.ncols();
    let mut triplets: Vec<(I, I, E)> = Vec::with_capacity(A.compute_nnz()); // Check this is the
                                                                            // write method
    for oldi in 0..oldn {
        for (oldj, v) in A.col_indices_of_row(oldi).zip(A.values_of_row(oldi)) {
            triplets.push((oldj * oldn + oldi) % m, (oldj * oldn + oldi) / m, *v);
        }
    }
    SparseColMat::try_new_from_triplets(m, n, triplets).unwrap()
} */

pub fn eye(n: u64) -> SparseColMat<u64, f64> {
    let n_usize = n.try_into().unwrap();
    SparseColMat::new(
        SymbolicSparseColMat::<u64>::new_checked(
            n_usize, n_usize,
            (0..n+1).collect::<Vec<_>>(),
            Some([1u64].repeat(n_usize)),
            (0..n).collect::<Vec<_>>()),
        [1.0f64].repeat(n_usize))
}
