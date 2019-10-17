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
}

/// Storage of "cells"
pub trait Storage {
    /// Dereferences cell under given address
    fn deref(&self, addr: usize) -> Option<Cell>;
}

impl Storage for [Cell] {
    fn deref(&self, mut addr: usize) -> Option<Cell> {
        let mut r = self.get(addr).cloned();

        while let Some(Cell::Ref(a)) = r {
            if a == addr {
                return r;
            } else {
                addr = a;
                r = self.get(addr).cloned()
            }
        }

        r
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

        assert_eq!(Cell::Struct(1), storage.deref(0).unwrap());
        assert_eq!(Cell::Funct(0, 0), storage.deref(1).unwrap());
        assert_eq!(Cell::Ref(2), storage.deref(2).unwrap());
        assert_eq!(Cell::Struct(1), storage.deref(3).unwrap());
        assert_eq!(Cell::Ref(2), storage.deref(4).unwrap());
    }
}
