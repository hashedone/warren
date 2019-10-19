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

    fn deref(&self) -> &[Cell] { &self.store }
}

impl std::ops::DerefMut for Storage {
    fn deref_mut(&mut self) -> &mut [Cell] { &mut self.store }
}

impl Storage {
    pub fn new() -> Self { Default::default() }

    #[cfg(test)]
    pub(crate) fn from_iter(
        regs: usize,
        store: impl Iterator<Item=Cell>
    ) -> Self {
        Self {
            regs,
            store: store.collect()
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

    /// Dereferences cell from given index
    /// Returns None if index is out of bound, or if
    /// referencing cell out of storage
    pub fn deref(&self, mut addr: usize) -> Option<Cell> {
        let mut r = self.store.get(addr).cloned();

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


