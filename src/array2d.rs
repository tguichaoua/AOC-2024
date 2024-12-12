use itertools::Itertools;

pub struct Array2D<T> {
    items: Box<[T]>,
    columns_count: usize,
}

impl<T> Array2D<T> {
    #[inline]
    pub fn from_elem(size: Size, mut f: impl FnMut(usize, usize) -> T) -> Self {
        let rows = 0..size.rows_count();
        let columns = 0..size.columns_count;

        let items = rows
            .cartesian_product(columns)
            .map(|(row, column)| f(row, column))
            .collect::<Box<[_]>>();

        debug_assert!(items.len() == size.items_count);

        Array2D {
            items,
            columns_count: size.columns_count,
        }
    }

    #[inline]
    pub fn size(&self) -> Size {
        Size {
            items_count: self.items.len(),
            columns_count: self.columns_count,
        }
    }

    #[inline]
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if column >= self.columns_count {
            return None;
        }
        let index = row * self.columns_count + column;
        self.items.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        if column >= self.columns_count {
            return None;
        }
        let index = row * self.columns_count + column;
        self.items.get_mut(index)
    }

    #[inline]
    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    #[inline]
    pub fn get_by_index_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone + '_ {
        self.items.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> + '_ {
        self.items.iter_mut()
    }

    /// `((row, column), item)`
    #[inline]
    pub fn iter_with_index(
        &self,
    ) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> + Clone + '_ {
        self.items.iter().enumerate().map(|(i, item)| {
            let row = i / self.columns_count;
            let column = i % self.columns_count;
            ((row, column), item)
        })
    }

    /// `((row, column), item)`
    #[inline]
    pub fn iter_mut_with_index(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = ((usize, usize), &mut T)> + '_ {
        self.items.iter_mut().enumerate().map(|(i, item)| {
            let row = i / self.columns_count;
            let column = i % self.columns_count;
            ((row, column), item)
        })
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    items_count: usize,
    columns_count: usize,
}

impl Size {
    #[inline]
    pub fn from_items_columns(items_count: usize, columns_count: usize) -> Self {
        Self {
            items_count,
            columns_count,
        }
    }

    #[inline]
    pub fn from_rows_columns(rows: usize, columns: usize) -> Self {
        Self::from_items_columns(rows * columns, columns)
    }

    #[inline]
    pub fn items_count(&self) -> usize {
        self.items_count
    }

    #[inline]
    pub fn columns_count(&self) -> usize {
        self.columns_count
    }

    #[inline]
    pub fn rows_count(&self) -> usize {
        self.items_count / self.columns_count
    }
}

/* -------------------------------------------------------------------------- */
