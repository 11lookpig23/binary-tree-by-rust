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
    pub fn new() -> Self {
         Tree::<T>
         {
             data : None,
             left : None,
             right : None,
         }
    }

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

    pub fn preorder(&self) -> IterPreorder<T> {
          IterPreorder::new(self)
    }

    pub fn inorder(&self) -> IterInorder<T> {
        IterInorder::new(self)
    }

    pub fn postorder(&self) -> IterPostorder<T> {
        IterPostorder::new(self)
    }


}

    pub struct IterPreorder<'a, T: 'a> {
            stack : Vec<(&'a Tree<T>,i32)>,
    }

    impl<'a, T> IterPreorder<'a, T> {
        fn new(tree:&'a Tree<T>) -> IterPreorder<'a, T>
        {
            IterPreorder
            {
            stack :vec![(tree,0)],
            }
        }
        fn node1( preorder:& mut  IterPreorder<'a ,T>) ->Option<&'a T>// IterPreorder<'a, T>
        {
            let len  = preorder.stack.len();
            let node = preorder.stack[len-1];
            if let Some(ref l) = node.0.left
            {
                let left_node = l;
                preorder.stack.push((left_node,1));
                if let Some(ref key) = (**l).data
                {
                    Some(key)
                }

                else {None}
            }
            else
            {
                preorder.stack[len-1].1 = 2;
                IterPreorder::node2(preorder)
            }
        }

      fn node2( preorder:& mut  IterPreorder<'a ,T>) ->Option<&'a T>
        {
            let len  = preorder.stack.len();
            //println!("{}{}", "len2 ",len);
            let node = preorder.stack[len-1];
                if let Some(ref r) = node.0.right
                {
                    let right_node = r;
                    preorder.stack.push((right_node,1));
                    if let Some(ref key) = (**r).data
                    {
                        Some(key)
                    }

                    else {None}
                }
                //pattern ........
                else
                {
                    //judge_empty_stack
                    //let len3 =
                    let mut s = 0;
                    let mut j = 0;
                    //chu wen ti le
                    for i in 0..len
                    {
                        if preorder.stack[len-i-1].1 == 1
                        {
                            s+=1;
                            j = i;
                            break
                        }
                        else
                        {
                            continue
                        }
                    }

                    if s!=0
                    {
                    for k in 0..j
                    {
                        preorder.stack.pop();
                    }
                    let len2 = preorder.stack.len();
                    preorder.stack[len2-1].1 = 2;
                    //println!("{}{}", "length ",len2);
                    IterPreorder::node2(preorder)
                    }
                    else {//println!("{}", "yep");
                        None}
                }
            }
        }

        impl<'a, T> Iterator for IterPreorder<'a, T>
        {
            type Item = &'a T;
            fn next(&mut self) -> Option<&'a T>
            {
                let len  = self.stack.len();
                //println!("{}{}","node",self.stack[len-1].1);
                if len != 0
            {   let  node = self.stack[len-1];
                if node.1 == 1
                {
                    IterPreorder::node1(self)
                }

                else if node.1==2
                {
                    IterPreorder::node2(self)
                }
                else if node.1 == 0
                {
                    self.stack[len-1].1 = 1;
                    if let Some(ref k) = self.stack[len-1].0.data
                    {
                        Some(k)
                    }
                    else
                    {
                        None
                    }
                }
                else
                {
                    None
                }
            }
            else {None}
            }
        }

pub struct IterInorder<'a, T: 'a> {
        stack : Vec<(&'a Tree<T>,i32)>,
}

impl<'a, T> IterInorder<'a, T> {
    fn new(tree:&'a Tree<T>) -> IterInorder<'a, T>
    {
        IterInorder
        {
        stack :vec![(tree,0)],
        }
    }
    fn Innode1( preorder:& mut  IterInorder<'a ,T>) ->Option<&'a T>// IterPreorder<'a, T>
    {
        let len  = preorder.stack.len();
        let node = preorder.stack[len-1];
        if let Some(ref l) = node.0.left
        {
            let left_node = l;
            preorder.stack.push((left_node,1));
        IterInorder::Innode1(preorder)
        }
        else
        {
            preorder.stack[len-1].1 = 2;
            if let Some(ref key) = node.0.data
            {
                Some(key)
            }
            else {None}
        }
    }

    fn Innode2( preorder:& mut  IterInorder<'a ,T>) ->Option<&'a T>
      {
          let len  = preorder.stack.len();
          //println!("{}{}", "len2 ",len);
          let node = preorder.stack[len-1];
              if let Some(ref r) = node.0.right
              {
                  let right_node = r;
                  preorder.stack.push((right_node,1));
                  IterInorder::Innode1(preorder)
              }
              //pattern ........
              else
              {
                  //judge_empty_stack
                  //let len3 =
                  let mut s = 0;
                  let mut j = 0;
                  //chu wen ti le
                  for i in 0..len
                  {
                      if preorder.stack[len-i-1].1 == 1
                      {
                          s+=1;
                          j = i;
                          break
                      }
                      else
                      {
                          continue
                      }
                  }

                  if s!=0
                  {
                  for k in 0..j
                  {
                      preorder.stack.pop();
                  }
                  let len2 = preorder.stack.len();
                  preorder.stack[len2-1].1 = 2;
                  if let Some(ref key) = preorder.stack[len2-1].0.data
                  {
                      Some(key)

                  }
                  else {None}

                  }
                  else {
                      None}
              }
          }
    }

impl<'a, T> Iterator for IterInorder<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T>
    {
        let len  = self.stack.len();
        if len != 0
    {
        let  node = self.stack[len-1];
        if node.1 == 1
        {
            IterInorder::Innode1(self)
        }
        else if node.1==2
        {
            IterInorder::Innode2(self)
        }
        else if node.1 == 0
        {
            self.stack[len-1].1 = 1;
            IterInorder::Innode1(self)
        }
        else
        {
            None
        }
    }
    else {None}
    }
}


pub struct IterPostorder<'a, T: 'a> {
    stack : Vec<(&'a Tree<T>,i32)>,
}

impl<'a, T> IterPostorder<'a, T> {
    fn new(tree:&'a Tree<T>) -> IterPostorder<'a, T>
    {
        IterPostorder
        {
        stack :vec![(tree,0)],
        }
    }
    fn postnode1( preorder:& mut  IterPostorder<'a ,T>) ->Option<&'a T>// IterPreorder<'a, T>
    {
        let len  = preorder.stack.len();
        let node = preorder.stack[len-1];
        if let Some(ref l) = node.0.left
        {
            let left_node = l;
            preorder.stack.push((left_node,1));
            IterPostorder::postnode1(preorder)
        }
        else
        {
            preorder.stack[len-1].1 = 2;
            IterPostorder::postnode2(preorder)
        }
    }
    fn pop_and_return( preorder:& mut  IterPostorder<'a ,T>) ->Option<&'a T>
    {
        let len3 = preorder.stack.len();
        if len3 != 1
        {if preorder.stack[len3-2].1 == 1
        {
            preorder.stack[len3-2].1 = 2;
            let nodepost = preorder.stack.pop().unwrap();
            if let Some(ref key) = nodepost.0.data
            {
                //preorder.stack[len3-2].1 = 2;
                Some(key)
            }
            else {None}
        }
        else
        {
                preorder.stack[len3-2].1= 3;
                let nodepost = preorder.stack.pop().unwrap();
                if let Some(ref key) = nodepost.0.data
                {
                    //preorder.stack[len3-2].1 = 2;
                    Some(key)
                }
                else {None}

        }
    }
    else
    {
        let nodepost = preorder.stack.pop().unwrap();
        if let Some(ref key) = nodepost.0.data
        {
            //preorder.stack[len3-2].1 = 2;
            Some(key)
        }
        else
        {

            None
        }
    }
    }
    fn postnode2( preorder:& mut  IterPostorder<'a ,T>) ->Option<&'a T>
      {
          let len  = preorder.stack.len();
          //println!("{}{}", "len2 ",len);
          let node = preorder.stack[len-1];
              if let Some(ref r) = node.0.right
              {
                  let right_node = r;
                  preorder.stack.push((right_node,1));
                  IterPostorder::postnode1(preorder)
              }
              //pattern ........
              else
              {
                  if len == 1
                  {
                      let nodepost = preorder.stack.pop().unwrap();
                      if let Some(ref key) = nodepost.0.data
                      {
                          //preorder.stack[len3-2].1 = 2;
                          Some(key)
                      }
                      else
                      {

                          None
                      }
                  }
                  //pop and return value
                  else
                  {
                      IterPostorder::pop_and_return(preorder)
                  }

              }
          }
}

impl<'a, T> Iterator for IterPostorder<'a, T> {
    //unimplemented!();
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T>
    {
        let len  = self.stack.len();
        //println!("{}{}","node",self.stack[len-1].1);
        if len != 0
    {
        let  node = self.stack[len-1];
        if node.1 == 1
        {
            IterPostorder::postnode1(self)
        }
        else if node.1==2
        {
            IterPostorder::postnode2(self)
        }
        else if node.1 == 0
        {
            self.stack[len-1].1 = 1;
            IterPostorder::postnode1(self)
        }
        else if node.1 == 3
        {
            IterPostorder::pop_and_return(self)
            //None
        }
        else
        {None}
    }
    else {None}
    }
}

/*
let mut root = Tree::<i32>::new();
let data = vec![3, 2, 1, 5, 4, 6, 7];
let data2 = vec![4, 5, 3, 1, 6, 2];

for v in data {
    root.insert(v);
}

println!("Inorder");
let k = root.postorder().take(7);
for v in k {
    println!("{}", v);
};*/

//}
