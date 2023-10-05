use std::collections::BTreeMap;
use std::cell::{RefCell, Cell};

use ic_cdk::{update, query};

type TodoStore = BTreeMap<u8, String>;

thread_local! {
    static TODOS: RefCell<TodoStore> = RefCell::default();
    static NEXT_ID: Cell<u8> = Cell::new(0);
}

#[update(name = "create")]
fn create(td: String) -> Result<u8, String>{
    let next_id  =  NEXT_ID.with(|nid| {
        let current = nid.get();
        nid.set(current + 1);
        current
    });
    TODOS.with(|todos| todos.borrow_mut().insert(next_id, td) );
    Ok(next_id)
}

#[query(name = "read")]
fn read(id: u8) -> Result<String, String> {
    TODOS.with(|todos| {
        match todos.borrow().get(&id) {
            Some(val) => Ok(val.clone()),
            None => Err("Invalid id for todo".to_string())
        }
    })
}

#[update(name = "update")]
fn update(id: u8, content: String) -> Result<(), String>{
    TODOS.with(|todos| {
        match todos.borrow().get(&id) {
            Some(_) => {
                todos.borrow_mut().insert(id, content);
                Ok(())
            },
            None => Err(format!("Invalid operation. Unable to find the todo with id {}", id).to_string())
        }
    } )
}


#[update(name = "delete")]
fn delete(id: u8) -> Result<(), String> {
    TODOS.with(|todos| {
        match  todos.borrow_mut().remove_entry(&id) {
            Some(_) => Ok(()),
            None => Err(format!("No todo with id {} actively present", id).to_string())
        }
    })
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
