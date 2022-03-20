use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImmutableTree<T> {
    items: Vec<ImmutableTreeNode<T>>,
    root_count: usize,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImmutableTreeNode<T> {
    val: T,
    idx: usize,
    layer: usize,
    parent: Option<usize>,
    children_anchors: Option<(usize, usize)>,
}

impl<T> ImmutableTree<T> {
    pub fn new<I>(root_iter: I) -> ImmutableTree<T>
    where
        I: IntoIterator<Item = T>,
    {
        let items = root_iter
            .into_iter()
            .enumerate()
            .map(|(i, val)| ImmutableTreeNode {
                val,
                idx: i,
                layer: 0,
                parent: None,
                children_anchors: None,
            })
            .collect::<Vec<_>>();

        let root_count = items.len();

        ImmutableTree { items, root_count }
    }

    pub fn add_layer<F, I>(&mut self, iter_gen: F)
    where
        F: Fn(&T) -> Option<I>,
        I: IntoIterator<Item = T>,
    {
        if self.items.is_empty() {
            return;
        }

        let mut idx = self.items.len();
        let last_item = self.items.last().unwrap();
        let last_layer = last_item.layer;
        let new_layer = last_layer + 1;

        let mut new_items = Vec::with_capacity(2_u32.pow(self.items.last().unwrap().layer as u32) as usize);

        for item in self.items.iter_mut().skip_while(|p| p.layer != last_layer) {
            let start = idx;

            if let Some(iter) = iter_gen(&item.val) {
                new_items.extend(iter.into_iter().enumerate().map(|(_, val)| {
                    let ret = ImmutableTreeNode {
                        val,
                        idx,
                        layer: new_layer,
                        parent: Some(item.idx),
                        children_anchors: None,
                    };

                    idx += 1;

                    ret
                }));

                if start != idx {
                    item.children_anchors = Some((start, idx));
                }
            }
        }

        self.items.append(&mut new_items);
    }

    pub fn add_layers_recursively<F, I>(&mut self, iter_gen: F)
    where
        F: Fn(&T) -> Option<I>,
        I: IntoIterator<Item = T>,
    {
        if self.items.is_empty() {
            return;
        }

        loop {
            let size = self.len();

            self.add_layer(&iter_gen);

            if self.len() == size {
                break;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn root_count(&self) -> usize {
        self.root_count
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<ImmutableTreeNode<T>> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<ImmutableTreeNode<T>> {
        self.items.iter_mut()
    }

    pub fn get(&self, idx: usize) -> Option<&ImmutableTreeNode<T>> {
        self.items.get(idx)
    }
}

impl<T> ImmutableTree<T> {
    pub fn print<F: Clone + Fn(&mut Formatter<'_>, &T) -> core::fmt::Result>(
        &self,
        f: &mut Formatter<'_>,
        print_func: F,
    ) -> core::fmt::Result {
        if self.items.is_empty() {
            write!(f, "━━root\n")?;
        }

        write!(f, "root\n")?;

        for item in self.items.iter().take_while(|v| v.parent.is_none()) {
            self.print_item(f, item, print_func.clone(), 1)?;
        }

        Ok(())
    }

    pub fn print_item<F: Clone + Fn(&mut Formatter<'_>, &T) -> core::fmt::Result>(
        &self,
        f: &mut Formatter<'_>,
        item: &ImmutableTreeNode<T>,
        print_func: F,
        indent: usize,
    ) -> core::fmt::Result {
        write!(
            f,
            "{}{}━",
            {
                let mut ret = String::new();

                if let Some(parent) = item.parent {
                    let mut curr = &self.items[parent];

                    while let Some(parent) = curr.parent {
                        let idx = curr.idx;
                        curr = &self.items[parent];
                        if idx != curr.children_anchors.unwrap().1 - 1 {
                            ret.insert_str(0, "┃ ")
                        } else {
                            ret.insert_str(0, "  ")
                        }
                    }

                    if curr.idx != self.root_count - 1 {
                        ret.insert_str(0, "┃ ")
                    } else {
                        ret.insert_str(0, "  ")
                    }
                }

                ret
            },
            if let Some(parent) = item.parent {
                if item.idx == self.items[parent].children_anchors.unwrap().1 - 1 {
                    "┗"
                } else {
                    "┣"
                }
            } else {
                if item.idx != self.root_count - 1 {
                    "┣"
                } else {
                    "┗"
                }
            }
        )?;

        print_func(f, &item.val)?;

        write!(f, "\n")?;

        if let Some(anchors) = item.children_anchors {
            for i in anchors.0..anchors.1 {
                self.print_item(f, &self.items[i], print_func.clone(), indent + 1)?;
            }
        }

        Ok(())
    }
}

impl<T: Debug> Debug for ImmutableTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(f, |f, v| write!(f, "{:?}", v))
    }
}

impl<T: Display> Display for ImmutableTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(f, |f, v| write!(f, "{}", v))
    }
}

impl<T> IntoIterator for ImmutableTree<T> {
    type Item = ImmutableTreeNode<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> ImmutableTreeNode<T> {
    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn layer(&self) -> usize {
        self.layer
    }

    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    pub fn children_anchors(&self) -> Option<(usize, usize)> {
        self.children_anchors
    }
}
