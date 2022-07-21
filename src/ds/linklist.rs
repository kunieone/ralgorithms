/* 听说rust写链表超级难，但是值得一试哈 */
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("->").field(&self.0).finish()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }
    pub fn unshift(&mut self, data: T) {
        self.0 = Some((data, Box::new(LinkedList(self.0.take()))))
    }

    pub fn push(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push(data),
            None => self.unshift(data),
        }
    }
}

#[test]
fn test_linked_list() {
    let mut l: LinkedList<&str> = LinkedList::new();
    l.push("我是第一个");
    l.push("我是第二个");
    l.push("我是最后一个！");
    println!("{:?}", l)
}
