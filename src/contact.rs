use serde::Deserialize;

#[derive(Clone)]
pub struct Contact {
    pub id: u64,
    pub first: String,
    pub last: String,
    pub phone: u64,
    pub email: String,
}

#[derive(Clone, Deserialize)]
pub struct NewContact {
    pub first: String,
    pub last: String,
    pub phone: u64,
    pub email: String,
}

impl Default for NewContact {
    fn default() -> Self {
        Self {
            first: String::new(),
            last: String::new(),
            phone: 0,
            email: String::new(),
        }
    }
}

impl NewContact {
    pub fn into_contact(self, id: u64) -> Result<Contact, ContactErrors> {
        let mut errors = ContactErrors::default();

        if self.first.is_empty() {
            errors.first.push_str("First name must not be empty");
        }

        if self.last.is_empty() {
            errors.last.push_str("Last name must not be empty");
        }

        if self.phone == 0 {
            errors.phone.push_str("Phone number must be set");
        }

        if self.email.is_empty() {
            errors.email.push_str("Email must not be empty");
        }

        if errors.has_error() {
            return Err(errors);
        }

        Ok(Contact {
            id,
            first: self.first,
            last: self.last,
            phone: self.phone,
            email: self.email,
        })
    }
}

#[derive(Clone)]
pub struct Contacts {
    contacts: Vec<Contact>,
}

// #[derive(Clone, Deserialize)]
pub struct ContactErrors {
    pub id: String,
    pub email: String,
    pub first: String,
    pub last: String,
    pub phone: String,
}

impl ContactErrors {
    pub fn has_error(&self) -> bool {
        !self.email.is_empty()
            || !self.first.is_empty()
            || !self.last.is_empty()
            || !self.phone.is_empty()
    }
}

impl Default for ContactErrors {
    fn default() -> Self {
        Self {
            id: String::new(),
            email: String::new(),
            first: String::new(),
            last: String::new(),
            phone: String::new(),
        }
    }
}

impl Contacts {
    pub fn new() -> Self {
        Self {
            contacts: vec![
                Contact {
                    id: 0,
                    first: "Margaret".to_string(),
                    last: "Hamilton".to_string(),
                    phone: 19365551234,
                    email: "margaret@apollo.nasa".to_string(),
                },
                Contact {
                    id: 1,
                    first: "Ada".to_string(),
                    last: "Lovelace".to_string(),
                    phone: 1815551234,
                    email: "ada@analytical.engine".to_string(),
                },
                Contact {
                    id: 2,
                    first: "Alan".to_string(),
                    last: "Turing".to_string(),
                    phone: 19125551234,
                    email: "alan@bletchley.uk".to_string(),
                },
                Contact {
                    id: 3,
                    first: "Grace".to_string(),
                    last: "Hopper".to_string(),
                    phone: 19065551234,
                    email: "grace@cobol.mil".to_string(),
                },
                Contact {
                    id: 4,
                    first: "Linus".to_string(),
                    last: "Torvalds".to_string(),
                    phone: 19695551234,
                    email: "linus@kernel.org".to_string(),
                },
            ],
        }
    }

    pub fn all(&self) -> Vec<Contact> {
        self.contacts.clone()
    }

    pub fn search(&self, term: &str) -> Vec<Contact> {
        self.all()
            .into_iter()
            .filter(|c| {
                c.first.to_lowercase().contains(&term.to_lowercase())
                    || c.last.to_lowercase().contains(&term.to_lowercase())
                    || c.email.to_lowercase().contains(&term.to_lowercase())
                    || c.phone.to_string().contains(&term.to_lowercase())
            })
            .collect()
    }

    pub fn add(&mut self, new: NewContact) -> Result<(), ContactErrors> {
        let id = self.contacts.iter().map(|c| c.id).max().unwrap_or(0) + 1;

        let contact = new.into_contact(id)?;
        self.contacts.push(contact);

        Ok(())
    }

    pub fn get_by_id(&self, contact_id: u64) -> Option<Contact> {
        self.contacts.iter().find(|c| c.id == contact_id).cloned()
    }

    pub fn has_id(&self, contact_id: u64) -> bool {
        self.contacts.iter().any(|c| c.id == contact_id)
    }

    pub fn edit(&mut self, contact_id: u64, edited: NewContact) -> Result<(), ContactErrors> {
        let contact = edited.into_contact(contact_id)?;

        if let Some(current) = self.contacts.iter_mut().find(|c| c.id == contact_id) {
            current.first = contact.first;
            current.last = contact.last;
            current.email = contact.email;
            current.phone = contact.phone;
            Ok(())
        } else {
            let mut errors = ContactErrors::default();
            errors.id.push_str("Could not find contact ID");
            Err(errors)
        }
    }
}
