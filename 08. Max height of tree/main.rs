use std::cmp::max;
struct Node{
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node{
    fn new(val:i32) -> Self {
        Node {
            val,
            left:None,
            right:None
        }
    }
}

fn insert_into_bst(root:&mut Option<Box<Node>>,val:i32) {
    if let Some(node) =root {
        if val< node.val {
            insert_into_bst(&mut node.left,val);
        }else{
            insert_into_bst(&mut node.right,val);
        }
    } else {
        *root = 
        Some(
            Box::new(
                Node::new(val)
            )
        );
    }
}
fn max_height_of_tree(root: &Option<Box<Node>>) ->i32 {
    match root{
        None =>0,
        Some(node) => 1+ max(
            max_height_of_tree(&node.left),
            max_height_of_tree(&node.right)
        ),
    }
}

fn main(){
    let arr = vec![5,6,4,9,2,3,1,7,8];
    let mut root = None;
    for &val in &arr {
        insert_into_bst(&mut root,val);
    }
    println!("Depth of thr tree is {}",max_height_of_tree(&root));
}