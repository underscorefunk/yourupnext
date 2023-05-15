/// The Hierarchy module is designed to allow entities to be associated with each other.
/// Setting up hierarchies can become complicated. To simplify the process a design constraint
/// has been applied that only allows a child's parent to be changed. Setting a parent will
/// automatically set the child relationship counterpart. This allows the hierarchy module to
/// manage the one to many side of the relationship. Parents without children will be removed
/// from the hierarchy. A single item can not exist in the hierarchy. A child without a parent will
/// be removed and a parent without children will be removed.
///
/// Todo:
/// - Parse linear set of relationships into a graph to find up or down stream components,
/// cousins, etc. This might not totally be necessary early on though.

use crate::prelude::*;

use std::collections::HashMap;

type Children = Vec<Id>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hierarchy {
    child_parent: HashMap<Id, Id>,
    parent_children: HashMap<Id, Children>,
}

impl Default for Hierarchy {
    fn default() -> Self {
        Self {
            child_parent: HashMap::default(),
            parent_children: HashMap::default(),
        }
    }
}

impl Hierarchy {
    pub fn new() -> Self {
        Self::default()
    }

    // ----------------------------------------------------------------------
    // Query
    // ----------------------------------------------------------------------


    /// Count how many children (have a parent) exist in a Hierarchy
    ///
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    ///
    /// // ┌───┐   ┌───┐
    /// // │ 0 ├─┬▶│ 1 │
    /// // └───┘ │ └───┘
    /// //       │ ┌───┐  ┌───┐
    /// //       └▶│ 2 ├─▶│ 3 │
    /// //         └───┘  └───┘
    ///
    /// let mut h = Hierarchy::new();
    /// let _ = h.set_parent(1, 0);
    /// let _ = h.set_parent(2, 0);
    /// let _ = h.set_parent(3, 2);
    ///
    /// assert_eq!(h.child_count(), 3);
    /// ```
    pub fn child_count(&self) -> usize {
        self.child_parent.len()
    }

    /// Count how many children (have a parent) exist in a Hierarchy
    ///
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    ///
    /// // ┌───┐   ┌───┐
    /// // │ 0 ├─┬▶│ 1 │
    /// // └───┘ │ └───┘
    /// //       │ ┌───┐  ┌───┐
    /// //       └▶│ 2 ├─▶│ 3 │
    /// //         └───┘  └───┘
    ///
    /// let mut h = Hierarchy::new();
    /// let _ = h.set_parent(1, 0);
    /// let _ = h.set_parent(2, 0);
    /// let _ = h.set_parent(3, 2);
    ///
    /// assert_eq!(h.parent_count(), 2);
    /// ```
    pub fn parent_count(&self) -> usize {
        self.parent_children.len()
    }

    /// Check if an Id exists as a parent (has children) in a Hierarchy
    ///
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    /// let mut h = Hierarchy::new();
    /// let _ = h.set_parent(1, 0);
    ///
    /// assert!( h.is_parent(0) );
    /// assert!( ! h.is_parent(1) );
    /// ```
    pub fn is_parent(&self, parent: Id) -> bool {
        match self.parent_children.get(&parent) {
            None => false,
            Some(children) => !children.is_empty()
        }
    }

    /// Get immediate children of a parent.
    /// Non-existent parents will return an empty vec.
    ///
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    /// let mut h = Hierarchy::new();
    /// let _ = h.set_parent(1, 0);
    /// let _ = h.set_parent(2, 0);
    ///
    /// assert_eq!( h.children(0), vec![1,2]);
    /// assert_eq!( h.children(1), vec![]);
    /// assert_eq!( h.children(3), vec![]);
    /// ```
    pub fn children(&self, parent: Id) -> Vec<Id> {
        match self.parent_children.get(&parent) {
            Some(children) => children.clone(),
            None => Vec::with_capacity(0)
        }
    }

    /// Check if an Id exists as a child (has a parent) in a Hierarchy
    ///
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    /// let mut h = Hierarchy::new();
    /// let _ = h.set_parent(1, 0);
    ///
    /// assert!( ! h.is_child(0) );
    /// assert!( h.is_child(1) );
    /// assert!( ! h.is_child(2) );
    /// ```
    pub fn is_child(&self, child: Id) -> bool {
        self.child_parent.contains_key(&child)
    }

    /// Attempt to get a child's parent Id
    ///
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    /// let mut h = Hierarchy::new();
    /// let _ = h.set_parent(1, 0);
    ///
    /// assert!( h.parent(0).is_none() );
    /// assert_eq!( h.parent(1), Some(0) );
    /// ```
    pub fn parent(&self, child: Id) -> Option<Id> {
        match self.child_parent.get(&child) {
            Some(id) => Some(*id),
            None => None
        }
    }


    /// Get parent Ids from furthest to closest
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    /// let mut h = Hierarchy::new();
    /// let _ = h.set_parent(1, 0);
    /// let _ = h.set_parent(2, 1);
    ///
    /// assert_eq!( h.ancestors(2), vec![0,1]);
    /// ```
    pub fn ancestors(&self, child: Id) -> Vec<Id> {
        if !self.is_child(child) {
            return Vec::with_capacity(0);
        }

        let mut ancestors: Vec<Id> = Vec::with_capacity(10);

        let mut child = child;

        while self.is_child(child) {
            let parent = self.parent(child).unwrap();
            ancestors.push(parent);
            child = parent;
        }

        ancestors.reverse();
        ancestors
    }

    /// Get parent Ids from furthest to closest including the child
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    /// let mut h = Hierarchy::new();
    /// let _ = h.set_parent(1, 0);
    /// let _ = h.set_parent(2, 1);
    ///
    /// assert_eq!( h.lineage(2), vec![0,1,2]);
    /// ```
    pub fn lineage(&self, child: Id) -> Vec<Id> {
        let mut ancetors = self.ancestors(child);
        ancetors.push(child);
        ancetors
    }


    // ----------------------------------------------------------------------
    // Command
    // ----------------------------------------------------------------------

    /// Remove a parent from a child
    ///
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    ///
    /// let mut h = Hierarchy::new();
    ///
    /// // P───┐   ┌───┐
    /// // │ 0 ├─┬▶│ 1 │
    /// // └───┘ │ └───C
    /// //       │ P───┐  P───┐  ┌───┐
    /// //       └▶│ 2 ├─▶│ 3 ├─▶│ 4 │
    /// //         └───C  └───C  └───C
    ///
    /// let _ = h.set_parent(1, 0);
    /// let _ = h.set_parent(2, 0);
    /// let _ = h.set_parent(3, 2);
    /// let _ = h.set_parent(4, 3);
    ///
    /// assert_eq!( h.parent_count(), 3 );
    /// assert_eq!( h.child_count(), 4 );
    /// assert_eq!( h.children(0), vec![1,2]);
    /// assert_eq!( h.lineage(4), vec![0,2,3,4]);
    ///
    /// let _ = h.remove_parent(2);
    ///
    /// // P───┐   ┌───┐
    /// // │ 0 ├──▶│ 1 │
    /// // └───┘   └───C
    /// //         P───┐  P───┐  ┌───┐
    /// //         │ 2 ├─▶│ 3 ├─▶│ 4 │
    /// //         └───┘  └───C  └───C
    ///
    /// assert_eq!( h.parent_count(), 3 );
    /// assert_eq!( h.child_count(), 3 );
    /// assert_eq!( h.children(0), vec![1] );
    /// assert_eq!( h.lineage(4), vec![2,3,4] );
    ///
    /// let _ = h.remove_parent(1);
    ///
    /// // P═══╗  Id: 0 — A parent with no
    /// // ║ 0 ║  children will be removed
    /// // ╚═══╝
    /// //         P───┐  P───┐  ┌───┐
    /// //         │ 2 ├─▶│ 3 ├─▶│ 4 │
    /// //         └───C  └───C  └───C
    ///
    /// assert_eq!( h.parent_count(), 2 );
    /// assert_eq!( h.child_count(), 2 );
    /// assert_eq!( h.children(0), vec![] );
    /// assert_eq!( h.lineage(4), vec![2,3,4] );
    /// ```
    pub fn remove_parent(&mut self, child: Id) -> CmdResult<()> {
        if !self.is_child(child) {
            return Err("Unable to remove parent that was not set.".to_string());
        }

        let parent = match self.child_parent.get(&child) {
            None => return Err("Unable to retrieve parent Id".to_string()),
            Some(parent) => *parent
        };

        self.child_parent.remove(&child);
        self.remove_child(parent, child)?;
        Ok(())
    }

    /// Establish hierarchical relationship by assigning a child to a parent
    ///
    /// ```
    /// use yourupnext::prelude::Hierarchy;
    /// let mut h = Hierarchy::new();
    /// let _ = h.set_parent(1, 0);
    ///
    /// assert!( h.is_parent(0) );
    /// assert!( ! h.is_parent(1) );
    /// ```
    pub fn set_parent(&mut self, child: Id, parent: Id) -> CmdResult<()> {
        // If it exists, it needs to be unsed and then reset
        self.child_parent.insert(child, parent);
        self.set_child(parent, child)?;
        Ok(())
    }

    // ----------------------------------------------------------------------
    // Private Query
    // ----------------------------------------------------------------------

    fn default_children() -> Vec<Id> {
        Vec::with_capacity(10)
    }

    // ----------------------------------------------------------------------
    // Private Command
    // ----------------------------------------------------------------------

    fn set_child(&mut self, parent: Id, child: Id) -> CmdResult<()> {
        if !self.is_child(child) {
            return Err("Can not assign non-existent child to parent".to_string());
        }

        if !self.parent_children.contains_key(&parent) {
            let mut children = Self::default_children();
            children.push(child);
            self.parent_children.insert(parent, children);
            return Ok(());
        }

        let children = self.parent_children.get_mut(&parent).unwrap();

        if children.contains(&child) {
            return Err("Can not double assign child to parent".to_string());
        }

        children.push(child);

        Ok(())
    }

    fn remove_child(&mut self, parent: Id, child: Id) -> CmdResult<()> {
        if !self.is_parent(parent) {
            return Err("Unable to remove children that aren't set".to_string());
        }

        let children = self.parent_children.get(&parent);
        if children.is_none() {
            return Err("Unable to get index of child in parent's children".to_string());
        }
        let children = children.unwrap();

        let child_index = children.iter().position(|hs_c| *hs_c == child);
        if child_index.is_none() {
            return Err("Unable to get index of child in parent's children".to_string());
        }
        let child_index = child_index.unwrap();

        self.parent_children.get_mut(&parent).unwrap().remove(child_index);

        if self.parent_children.get(&parent).is_some_and(|children| children.is_empty()) {
            self.parent_children.remove(&parent);
        }

        Ok(())
    }

}