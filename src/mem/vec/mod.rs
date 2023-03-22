use {
    alloc::alloc::{alloc, dealloc, realloc, Layout},
    core::mem::size_of,
};

#[derive(Debug, Clone)]
pub struct Vec<Item: Sized> {
    data: *mut Item,
    len: usize,
    capacity: usize,
}

impl<Item: Sized> Vec<Item> {
    const ITEM_SIZE: usize = size_of::<Item>();
    const ALIGN: usize = core::mem::align_of::<Item>();

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: unsafe {
                alloc(Layout::from_size_align_unchecked(
                    capacity * Self::ITEM_SIZE,
                    Self::ALIGN,
                )) as _
            },
            len: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: Item) {
        if self.len >= self.capacity {
            self.resize(2 * self.capacity);
        }

        unsafe {
            self.data.add(self.len).write(item);
        }
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn resize(&mut self, capacity: usize) {
        self.data = unsafe {
            realloc(
                self.data as _,
                Layout::from_size_align_unchecked(capacity * Self::ITEM_SIZE, Self::ALIGN),
                capacity,
            ) as _
        };

        self.capacity = capacity;
    }
}

impl<Item: Clone> From<&[Item]> for Vec<Item> {
    fn from(items: &[Item]) -> Self {
        let data: *mut Item = unsafe { alloc(Layout::new::<&[Item]>()) as _ };
        let data = unsafe { core::slice::from_raw_parts_mut(data, items.len()) };
        data.clone_from_slice(items);

        Vec {
            data: data.as_mut_ptr(),
            len: items.len(),
            capacity: items.len(),
        }
    }
}

impl<Item> core::ops::Deref for Vec<Item> {
    type Target = [Item];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.data, self.len) }
    }
}

impl<Item> core::ops::Index<usize> for Vec<Item> {
    type Output = Item;

    fn index(&self, idx: usize) -> &Self::Output {
        unsafe { &*self.data.add(idx) }
    }
}

impl<Item> core::ops::IndexMut<usize> for Vec<Item> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        unsafe { &mut *self.data.add(idx) }
    }
}

impl<Item> core::iter::FromIterator<Item> for Vec<Item> {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut list = Self::with_capacity(1);
        for x in iter {
            list.push(x);
        }

        list
    }
}

impl<Item> Drop for Vec<Item> {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.data as _,
                Layout::from_size_align_unchecked(self.capacity, core::mem::align_of::<Item>()),
            );
        }
    }
}
