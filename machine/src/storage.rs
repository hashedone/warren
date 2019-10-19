/// Single Cell in storage for public interface
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// Empty cell
    Empty,
    /// Reference to another Cell
    Ref(usize),
    /// Structure
    Struct(usize),
    /// Structure Functor (with its ident and arity)
    Funct(usize, usize),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}

/// Mutable storage of "cells"
pub trait StorageMut {
    /// Pushes self-referencing ref and returns its cell
    fn push_var(&mut self) -> Cell;

    /// Pushes new structure to storage and returns index to it
    ///
    /// This should be followed by `arity` additional subcells pushing
    /// to keep storage valid
    fn push_struct(&mut self, ident: usize, arity: usize) -> Cell;

    /// Binds values in storage
    ///
    /// After this operation if one of the addressed cell is self-reference,
    /// it becomes a reference to the other cell. If both cells are
    /// self-references, the one is left, and the other becomes reference to
    /// the first. If no cell is self-reference, nothing happens.
    fn bind(&mut self, left: usize, right: usize);

    /// Unifies two addresses in storage
    fn unify(&mut self, a1: usize, a2: usize) -> bool;
}

impl StorageMut for Vec<Cell> {
    fn push_var(&mut self) -> Cell {
        self.push(Cell::Ref(self.len()));
        *self.last().unwrap()
    }

    fn push_struct(&mut self, ident: usize, arity: usize) -> Cell {
        self.push(Cell::Struct(self.len() + 1));
        self.push(Cell::Funct(ident, arity));
        self[self.len() - 2]
    }

    fn bind(&mut self, left: usize, right: usize) {
        match (self[left], self[right]) {
            (Cell::Ref(lcell), _) if lcell == left => {
                self[left] = Cell::Ref(right)
            },
            (_, Cell::Ref(rcell)) if rcell == right => {
                self[right] = Cell::Ref(left)
            },
            _ => ()
        }
    }

    fn unify(&mut self, a1: usize, a2: usize) -> bool {
        let mut pld = vec![(a1, a2)];

        while let Some((d1, d2)) = pld.pop() {
            let d1 = self.deref(d1);
            let d2 = self.deref(d2);
            if d1 != d2 {
                match (self[d1], self[d2]) {
                    (Cell::Ref(_), _) | (_, Cell::Ref(_)) =>
                        self.bind(d1, d2),
                    (Cell::Struct(v1), Cell::Struct(v2)) => {
                        if let (Cell::Funct(f1, n1), Cell::Funct(f2, n2)) =
                            (self[v1], self[v2])
                        {
                            if f1 == f2 && n1 == n2 {
                                for i in 1..=n1 {
                                    pld.push((v1 + i, v2 + i))
                                }
                            } else { return false }
                        } else { return false }
                    },
                    _ => return false
                }
            }
        }

        true
    }
}

/// Storage of "cells"
pub trait Storage {
    /// Dereferences cell under given address
    fn deref(&self, addr: usize) -> usize;
}

impl Storage for [Cell] {
    fn deref(&self, mut addr: usize) -> usize {
        while let Some(Cell::Ref(a)) = self.get(addr) {
            if *a == addr {
                return addr;
            } else {
                addr = *a;
            }
        }

        addr
    }
}

#[cfg(test)]
mod test {
    use super::{Cell, Storage, StorageMut};

    #[test]
    fn push_var() {
        let mut storage = vec![];
        let var = storage.push_var();

        if let Cell::Ref(v) = var {
            assert_eq!(var, storage[v]);
        } else {
            panic!()
        }
    }

    #[test]
    fn push_const() {
        let mut storage = vec![];
        let c = storage.push_struct(0, 0);

        if let Cell::Struct(s) = c {
            assert_eq!(Cell::Funct(0, 0), storage[s]);
        } else {
            panic! {}
        }
    }

    #[test]
    fn push_struct() {
        let mut storage = vec![];
        let s = storage.push_struct(0, 2);
        let v1 = storage.push_var();
        let v2 = storage.push_var();

        if let Cell::Struct(s) = s {
            assert_eq!(&[Cell::Funct(0, 2), v1, v2,], &storage[s..s + 3]);
        }
    }

    #[test]
    fn deref() {
        let mut storage = vec![];

        storage.push_struct(0, 0);
        let v = storage.push_var();
        storage.push(Cell::Ref(0));
        storage.push(v.clone());

        assert_eq!(0, storage.deref(0));
        assert_eq!(1, storage.deref(1));
        assert_eq!(2, storage.deref(2));
        assert_eq!(0, storage.deref(3));
        assert_eq!(2, storage.deref(4));
    }
}
