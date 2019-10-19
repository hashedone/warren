/// Single Cell in storage for public interface
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// Reference to another Cell
    Ref(usize),
    /// Structure
    Struct(usize),
    /// Structure Functor (with its ident and arity)
    Funct(usize, usize),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Ref(0)
    }
}

impl Cell {
    pub fn to_ref(self) -> Option<usize> {
        if let Self::Ref(r) = self {
            Some(r)
        } else {
            None
        }
    }

    pub fn to_struct(self) -> Option<usize> {
        if let Self::Struct(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn to_funct(self) -> Option<(usize, usize)> {
        if let Self::Funct(f, n) = self {
            Some((f, n))
        } else {
            None
        }
    }
}

/// Address space for machine
#[derive(Debug)]
pub struct Storage {
    /// Store begins with number of registers, defined before calulation,
    /// followed by heap which grows infienetely
    ///
    /// Addressing heap and registers is actually unificated - all the
    /// difference is that registers has adresses lower than `regs`, and
    /// anything with adress higher or equal than is heap.
    store: Vec<Cell>,

    /// Number for registers reserved (also index of first heap cell)
    regs: usize,
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            store: vec![],
            regs: 0,
        }
    }
}

impl std::ops::Deref for Storage {
    type Target = [Cell];

    fn deref(&self) -> &[Cell] {
        &self.store
    }
}

impl std::ops::DerefMut for Storage {
    fn deref_mut(&mut self) -> &mut [Cell] {
        &mut self.store
    }
}

impl Storage {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(test)]
    pub(crate) fn from_iter(regs: usize, store: impl Iterator<Item = Cell>) -> Self {
        Self {
            regs,
            store: store.collect(),
        }
    }

    /// Resets storage before execution
    ///
    /// * `regs` - Number of registers to be used in this calculation
    pub fn reset(&mut self, regs: usize) {
        self.regs = regs;
        self.store.resize_with(regs, Default::default)
    }

    /// Returns slice of all registers
    pub fn registers(&self) -> &[Cell] {
        &self.store[0..self.regs]
    }

    /// Pushes struct to heap, and returns pushed struct cell
    pub fn push_struct(&mut self, ident: usize, arity: usize) -> Cell {
        self.store.push(Cell::Struct(self.store.len() + 1));
        self.store.push(Cell::Funct(ident, arity));
        self.store[self.store.len() - 2]
    }

    /// Pushes new variable (self referenced cell) on heap,
    /// and returns pushed cell
    pub fn push_var(&mut self) -> Cell {
        self.store.push(Cell::Ref(self.store.len()));
        *self.last().unwrap()
    }

    /// Pushes cell on heap, and returns pushed cell
    pub fn push_cell(&mut self, cell: Cell) -> Cell {
        self.store.push(cell);
        *self.last().unwrap()
    }

    /// Dereferences cell from given index, and returns
    /// index of destinated cell
    ///
    /// Returns None if index is out of bound, or if
    /// referencing cell out of bound
    pub fn deref_idx(&self, mut addr: usize) -> Option<usize> {
        while let Cell::Ref(a) = self.store.get(addr)? {
            if *a == addr {
                return Some(*a);
            } else {
                addr = *a
            }
        }

        Some(addr)
    }

    /// Dereferences cell from given index, and returns
    /// contained cell value
    ///
    /// Returns None if index is out of bound, or if
    /// referencing cell out of bound
    pub fn deref(&self, addr: usize) -> Option<Cell> {
        self.deref_idx(addr).map(|idx| self.store[idx])
    }

    /// Binds self referenced cell to the other cell if one of
    /// given cell is self referencing
    pub fn bind(&mut self, a1: usize, a2: usize) {
        match (self.store[a1], self.store[a2]) {
            (Cell::Ref(r1), _) if r1 == a1 => self.store[a1] = Cell::Ref(a2),
            (_, Cell::Ref(r2)) if r2 == a2 => self.store[a2] = Cell::Ref(a1),
            _ => (),
        }
    }

    /// Unifies two cells in storage
    ///
    /// Returns true if unification succeed, false otherwise
    pub fn unify(&mut self, a1: usize, a2: usize) -> bool {
        // Try block workaround
        || -> Option<()> {
            let mut pld = vec![(a1, a2)];

            while let Some((d1, d2)) = pld.pop() {
                let d1 = self.deref_idx(d1)?;
                let d2 = self.deref_idx(d2)?;

                if d1 != d2 {
                    match (self.store[d1], self.store[d2]) {
                        (Cell::Ref(_), _) | (_, Cell::Ref(_)) => self.bind(d1, d2),
                        (Cell::Struct(v1), Cell::Struct(v2)) => {
                            let (f1, n1) = self.store.get(v1)?.to_funct()?;
                            let (f2, n2) = self.store.get(v2)?.to_funct()?;

                            if f1 == f2 && n1 == n2 {
                                for i in 1..=n1 {
                                    pld.push((v1 + i, v2 + i))
                                }
                            } else {
                                None?
                            };
                        }
                        _ => None?,
                    }
                }
            }

            Some(())
        }()
        .is_some()
    }
}
