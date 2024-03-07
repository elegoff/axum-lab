use crate::ctx::Ctx;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Ticket typees
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub creator_id: u64,
    pub title: String,
}

//Payload for ticket creation
//doesn not include id whihch will be generated by backend
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// Model controller
//
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }

    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        //next id
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let creator_id = ctx.user_id();
        let ticket = Ticket {
            id,
            creator_id,
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self, ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        //filter_mao allows exlusion of None(s)
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
