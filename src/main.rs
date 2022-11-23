use crate::buffer::Buffer;

mod buffer;
mod editor;

fn main() {
    let mut data = Buffer::from_text("");
    for i in 10..20 {
        data.replace(data.offset, i.to_string().as_str());
    }

    println!("{:?}\n", data);

    data.undo();
    println!("{:?}\n", data);
    data.undo();
    println!("{:?}\n", data);
    data.undo();
    println!("{:?}\n", data);

    // data.insert(data.offset, 11.to_string().as_str());
    // println!("{:?}\n", data);
    // data.insert(data.offset, 12.to_string().as_str());
    // println!("{:?}\n", data);

    data.redo();
    println!("{:?}\n", data);
    data.redo();
    println!("{:?}\n", data);

    data.undo();
    println!("{:?}\n", data);
    data.undo();
    println!("{:?}\n", data);
    data.undo();
    println!("{:?}\n", data);
}

#[derive(Debug, Default)]
pub struct Data {
    pub count: i32,
    pub his: History,
}

impl Data {
    pub fn insert(&mut self, val: i32) {
        self.count += val;
        self.his.stack.insert(self.his.pos, Type::Insert(val));
        self.his.pos += 1;
    }

    pub fn delete(&mut self, val: i32) {
        self.count -= val;
        self.his.stack.insert(self.his.pos, Type::Delete(val));
        self.his.pos += 1;
    }

    pub fn undo(&mut self) {
        if self.his.pos <= 0 { return };
        self.his.pos -= 1;
        match self.his.stack.remove(self.his.pos) {
            Type::Insert(x) => {
                self.delete(x)}
            ,
            Type::Delete(y) => self.insert(y),
        }
        self.his.pos -= 1;
    }

    pub fn redo(&mut self) {
        if self.his.pos >= self.his.stack.len() { return };
        match self.his.stack.remove(self.his.pos) {
            Type::Insert(x) => self.delete(x),
            Type::Delete(y) => self.insert(y),
        }
    }
}

#[derive(Debug, Default)]
pub struct History {
    pub stack: Vec<Type>,
    pub pos: usize,
}

impl History {
}

#[derive(Debug)]
pub enum Type {
    Insert(i32),
    Delete(i32),
}