struct Node {
    elem: i32,
    next: Link,
}
pub enum List {
    Empty,
    More(Box<Node>)
}