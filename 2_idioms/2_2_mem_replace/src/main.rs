use std::fmt::Debug;
use std::mem;

fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s)
}

#[derive(Clone, Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Clone> Trinity<T> {
    fn rotate(&mut self) {
        mem::swap(&mut self.a, &mut self.b);
        mem::swap(&mut self.b, &mut self.c);
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: Clone + PartialEq + Debug> Solver<T> {
    /// Removes any item from `unsolved` that matches `expected` in *any* of its 3 rotations.
    fn resolve(&mut self) {
        self.unsolved.retain_mut(|trinity| {
            for _ in 0..3 {
                if *trinity == self.expected {
                    return false;
                }
                trinity.rotate();
            }
            true
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trinity_rotate() {
        let mut t = Trinity { a: 1, b: 2, c: 3 };
        t.rotate();
        assert_eq!(t, Trinity { a: 2, b: 3, c: 1 });

        t.rotate();
        assert_eq!(t, Trinity { a: 3, b: 1, c: 2 });

        t.rotate();
        assert_eq!(t, Trinity { a: 1, b: 2, c: 3 });
    }

    #[test]
    fn test_solver_resolve() {
        let mut solver = Solver {
            expected: Trinity { a: 1, b: 2, c: 3 },
            unsolved: vec![
                Trinity { a: 1, b: 2, c: 3 },
                Trinity { a: 2, b: 1, c: 3 },
                Trinity { a: 2, b: 3, c: 1 },
                Trinity { a: 3, b: 1, c: 2 },
            ],
        };

        solver.resolve();

        assert_eq!(solver.unsolved, vec![Trinity { a: 2, b: 1, c: 3 },]);
    }

    #[test]
    fn test_solver_resolve_no_unsolved() {
        let mut solver = Solver {
            expected: Trinity { a: 1, b: 2, c: 3 },
            unsolved: vec![Trinity { a: 1, b: 2, c: 3 }, Trinity { a: 1, b: 2, c: 3 }],
        };

        solver.resolve();

        assert!(solver.unsolved.is_empty());
    }

    #[test]
    fn test_solver_resolve_all_unsolved() {
        let mut solver = Solver {
            expected: Trinity { a: 1, b: 2, c: 3 },
            unsolved: vec![Trinity { a: 4, b: 5, c: 6 }, Trinity { a: 7, b: 8, c: 9 }],
        };

        solver.resolve();

        assert_eq!(
            solver.unsolved,
            vec![Trinity { a: 4, b: 5, c: 6 }, Trinity { a: 7, b: 8, c: 9 },]
        );
    }
}
