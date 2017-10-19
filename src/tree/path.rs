
use std::fmt;

#[derive(Clone, Debug)]
pub struct PathRef<'p> {
    path: &'p [usize],
}

#[derive(Clone, Debug)]
pub struct Path {
    path: Vec<usize>,
}

impl Path {
    pub fn new() -> Self {
        Path { path: vec![0] }
    }

    pub fn from_vec(path: Vec<usize>) -> Self {
        Path { path }
    }

    pub fn push(&mut self, element: usize) {
        self.path.push(element)
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn raw(&self) -> &[usize] {
        &self.path
    }

    pub fn parent(&self) -> PathRef {
        PathRef { path: &self.path[..self.len() - 1] }
    }

    pub fn reference(&self) -> PathRef {
        PathRef { path: &self.path }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reference())
    }
}

impl<'p> fmt::Display for PathRef<'p> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.path {
            p if p.is_empty() => write!(f, ""),
            p if p.len() == 1 => write!(f, "{}", p[0]),
            p => {
                let id = (&p[1..]).iter().fold(p[0].to_string(), |id, n| {
                    id + &format!(".{}", n)
                });
                write!(f, "{}", id)
            }
        }
    }
}