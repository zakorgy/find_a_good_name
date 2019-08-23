use crate::entity::{Entity, MessageDispatcher};
use std::marker::PhantomData;
use std::rc::Rc;
use std::cell::RefCell;

pub trait State<E: Entity> {
    fn enter(&self, owner: &mut E, dispatcher: &mut MessageDispatcher);
    fn execute(&self, owner: &mut E, dispatcher: &mut MessageDispatcher);
    fn exit(&self, owner: &mut E, dispatcher: &mut MessageDispatcher);
}

pub struct StateMachine<E: Entity, S: State<E> + Clone + Copy + Eq + PartialEq> {
    global_state: Option<Rc<RefCell<S>>>,
    prev_state: S,
    current_state: S,
    phantom: PhantomData<E>,
}

impl<E: Entity, S: State<E> + Clone + Copy + Eq + PartialEq> StateMachine<E, S> {
    pub fn new(global_state: Option<Rc<RefCell<S>>>, state: S) -> Self {
        Self {
            global_state,
            prev_state: state,
            current_state: state,
            phantom: PhantomData,
        }
    }
    pub fn update(&self, owner: &mut E, dispatcher: &mut MessageDispatcher) {
        if let Some(state) = self.global_state.as_ref() {
            state.borrow().execute(owner, dispatcher);
        }
        self.current_state.execute(owner, dispatcher);
    }

    pub fn change_state(&mut self, owner: &mut E, mut state: S, dispatcher: &mut MessageDispatcher) {
        self.current_state.exit(owner, dispatcher);
        std::mem::swap(&mut self.prev_state, &mut self.current_state);
        std::mem::swap(&mut self.current_state, &mut state);
        self.current_state.enter(owner, dispatcher);
    }

    pub fn revert_to_prev_state(&mut self, owner: &mut E, dispatcher: &mut MessageDispatcher) {
        self.change_state(owner, self.prev_state.clone(), dispatcher)
    }

    pub fn is_in_state(&self, state: &S) -> bool {
        &self.current_state == state
    }
}
