use actix::{Actor, Context, Handler, Message};
use std::collections::HashMap;

pub struct Count(pub String);

impl Message for Count {
    type Result = i32;
}

pub struct CountActor {
    counter: HashMap<String, i32>,
}

impl CountActor {
    pub fn new() -> Self {
        Self {
            counter: HashMap::new(),
        }
    }
}

impl Actor for CountActor {
    type Context = Context<Self>;
}

impl Handler<Count> for CountActor {
    type Result = i32;

    fn handle(&mut self, Count(path): Count, _: &mut Context<Self>) -> Self::Result {
        let value = self.counter.entry(path).or_default();
        *value = *value + 1;
        *value
    }
}
