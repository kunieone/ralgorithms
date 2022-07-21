use std::fmt::{Debug, Display};

// #[derive(Debug)]
pub struct Stack<T>(Option<(T, Box<Stack<T>>)>);

impl<T: Debug + Display> Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_tuple("->").field(&self.0).finish()
        let mut tmp = &self.0;
        let mut content = "".to_string();
        let mut i = 0;
        loop {
            match tmp {
                Some(t) => {
                    tmp = &(*t.1).0;
                    content.push_str(&format!("{} |{: ^12}|\n", i, t.0).to_string());
                }
                None => {
                    content.push_str(&format!("{} |{: ^12}|", i, "nil"));
                    break;
                }
            }
            i += 1;
        }
        write!(f, "{content}")
    }
}
impl<T: Debug> Stack<T> {
    /// .
    /// 初始化
    /// .
    pub fn new_with_none() -> Self {
        Self(None)
    }
    /// .
    /// 初始化,并添加一个值
    /// .
    pub fn new(value: T) -> Self {
        Self(Some((value, Box::new(Stack(None)))))
    }

    /// .
    /// 入栈
    /// .
    pub fn push(&mut self, v: T) {
        self.0 = Some((v, Box::new(Stack(self.0.take()))))
    }
    ///出栈
    pub fn pop(&mut self) -> Option<T> {
        match self.0.take() {
            None => None,
            Some(v) => {
                let new_one = *v.1;
                *self = new_one;

                Some(v.0)
            }
        }
    }
}

#[test]
fn link_test() {
    let mut l = Stack::new(1);
    l.push(3);
    l.pop();
    l.pop();
    l.push(123456789);
    println!("{:?}", l)
}
