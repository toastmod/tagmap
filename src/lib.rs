use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests {
    use super::*;

    /// I'm aware this test is pretty redundant but I was just bored.
    #[test]
    fn it_works() {
        
        let get: fn(Option<&String>) -> String = |x| {
            x.expect("Integrity error!").clone() 
        }; 

        let mut tagmap = TagMap::new();
        
        let two = tagmap.add(String::from("Two fish!"));
        let sam = tagmap.add(String::from("Sam I am!"));
        let four = tagmap.add(String::from("Blue fish!"));
        let green_eggs = tagmap.add(String::from("Green Eggs!"));
        let three = tagmap.add(String::from("Red fish!"));
        let one = tagmap.add(String::from("One fish!"));

        assert_eq!(tagmap.len(), 6);

        tagmap.eject(sam);
        tagmap.eject(green_eggs);

        assert_eq!(tagmap.len(), 4);

        assert_eq!(get(tagmap[one].as_ref()), "One fish!");
        assert_eq!(get(tagmap[two].as_ref()), "Two fish!");
        assert_eq!(get(tagmap[three].as_ref()), "Red fish!");
        assert_eq!(get(tagmap[four].as_ref()), "Blue fish!");

    }
}

#[allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagMap<T> {
    map: Vec<Option<T>>,
    avail: Vec<usize>,
    len: usize,
}

impl<T> TagMap<T> {
    pub fn new() -> Self {
        Self {
            map: vec![],
            avail: vec![],
            len: 0usize,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Add an item to the map and return a `usize` tag.
    /// ```rust
    /// use tagmap::TagMap;
    /// let mut tagmap = TagMap::new();
    /// 
    /// let my_thing_tag = tagmap.add(999);
    /// assert_eq!(tagmap[my_thing_tag].expect("Integrity error!"), 999);
    /// ```
    pub fn add(&mut self, item: T) -> usize {
        self.len += 1;
        match self.avail.pop() {
            Some(avail) => {
                self.map[avail] = Some(item); 
                avail
            },
            None => {
                self.map.push(Some(item));
                self.map.len()-1
            },
        }
    }

    /// Eject an item from the map.
    /// ```rust
    /// use tagmap::TagMap;
    /// let mut tagmap = TagMap::new();
    /// 
    /// let my_thing_tag = tagmap.add(999);
    /// let my_thing = tagmap.eject(my_thing_tag).expect("Integrity error!");
    /// ```
    pub fn eject(&mut self, tag: usize) -> Option<T> { 
        self.len -= 1;
        let o = self.map[tag].take();
        if o.is_some() {
            self.avail.push(tag);
        }
        o
    }
}

impl<T> Index<usize> for TagMap<T> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.map[index]
    }
}

impl<T> IndexMut<usize> for TagMap<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.map[index]
    }
}
