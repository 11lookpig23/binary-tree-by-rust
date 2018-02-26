use std::ptr;
use std::cmp::Ordering;
//fn main()
//{
    type Node<T> = Option<Box<Tree<T>>>;
    pub struct Tree<T>
    {
    data : Option<T>,
    left :  Node<T>,
    right : Node<T>,
    }

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {

         Tree::<T>
         {
             data : None,
             left : None,
             right : None,
         }
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        if let Some(ref t) = self.data
        {
            match key.cmp(t)
            {
                Ordering::Equal => {false},
                Ordering::Less =>
                {
                    if let Some(ref mut l) = self.left {l.insert(key)}
                    else
                    {
                        self.left = Some(Box::new(Tree{data:Some(key),left:None,right:None}));
                        true
                    }
                 },
                Ordering::Greater =>
                {
                    if let Some(ref mut r) = self.right {r.insert(key)}
                    else
                    {
                        self.right = Some(Box::new(Tree{data:Some(key),left:None,right:None}));
                        true
                    }
                 },
            }
        }
        else
        {
            self.data = Some(key);
            true
        }
    /*let new_tree = Tree{data:Some(key),left:None,right:None};
    if self.data == None
    {true}
    else if self.data < new_tree.data
    {
        match self.right
        {
        None => {self.right = Some(Box::new(new_tree));
            true },
        Some(ref mut r) => {
                match new_tree.data {
                    Some(value) =>{ r.insert(value)},
                    _ =>{true},
                }
            },
        }
    }
    else if self.data > new_tree.data
    {
        match self.left
        {
        None => {self.left = Some(Box::new(new_tree));
            true },
        Some(ref mut r) => {
                match new_tree.data {
                    Some(value) =>{ r.insert(value)},
                    _ =>{true},
                }
                        },
        }
    }

    else if  self.data == new_tree.data
    {false}
    else {true}*/
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key:&T) -> bool
    {
        if let Some(ref t) = self.data{
        match key.cmp(t)   //&self.data
        {
            Ordering::Equal => {true},
            Ordering::Less =>
            {
            if let Some(ref l) = self.left
            {l.find(key)}
            else {false}
            },
            Ordering::Greater =>
            {
                if let Some(ref r) = self.right
                {r.find(key)}
                else {false}
            },
        }
    }
    else {false}
    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T>
    {
     let mut stack = vec![];
     let mut nothing:Vec<&T> = vec![];
     let mut Left:Vec<&T> = vec![];
     let mut Right:Vec<&T> = vec![];
     if let Some(ref node) = self.data{stack.push(node);}
     else {}
     if let Some(ref l) = self.left
     {
         Left = l.preorder();
         for j in  0..Left.len()
         {
             stack.push(Left[j]);
         }
     }
     else {}
     if let Some(ref r) = self.right
     {
         Right = r.preorder();
         for i in  0..Right.len()
         {
             stack.push(Right[i]);
         }
     }
     else {}

     stack
    }

/// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut stack = vec![];
        let mut nothing:Vec<&T> = vec![];
        let mut Left:Vec<&T> = vec![];
        let mut Right:Vec<&T> = vec![];
        if let Some(ref l) = self.left
        {
            Left = l.inorder();
            for j in  0..Left.len()
            {
                stack.push(Left[j]);
            }
        }
        else {}
        if let Some(ref node) = self.data{stack.push(node);}
        else {}
        if let Some(ref r) = self.right
        {
            Right = r.inorder();
            for i in  0..Right.len()
            {
                stack.push(Right[i]);
            }
        }
        else {}
        stack
    }


    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut stack = vec![];
        let mut nothing:Vec<&T> = vec![];
        let mut Left:Vec<&T> = vec![];
        let mut Right:Vec<&T> = vec![];
        if let Some(ref l) = self.left
        {
            Left = l.postorder();
            for j in  0..Left.len()
            {
                stack.push(Left[j]);
            }
        }
        else {}
        if let Some(ref r) = self.right
        {
            Right = r.postorder();
            for i in  0..Right.len()
            {
                stack.push(Right[i]);
            }
        }
        else {}
        if let Some(ref node) = self.data{stack.push(node);}
        else {}
        stack
    }
}
/*
let empty_tree = Tree::<i32>::new();
let re3 = Tree::<i32>{data : Some(5),left:None ,right : None};
let le2 = Tree::<i32>{data : Some(1),left :None ,right : None};
let le =  Tree::<i32>{data : Some(2),left:Some(Box::new(le2)),right :None};
let le3 = Tree::<i32>{data : Some(1),left:None ,right : None};
let re2 =  Tree::<i32>{data : Some(7),left:None ,right :None};
let re =  Tree::<i32>{data : Some(4),left: None,right :Some(Box::new(re3))} ;
let my_tree = Tree::<i32>{data : Some(3),left:Some(Box::new(le)) ,right : Some(Box::new(re))};
let v = my_tree.postorder();
for i in 0..v.len()
{
    println!("{}", v[i]);
}*/
//}
