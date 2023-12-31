use std::any::Any;
use std::rc::Rc;

use itertools::Itertools;
use anyhow::Result;
use anyhow::anyhow;

use crate::db::JiraDatabase;
use crate::models::Action;

mod page_helpers;
use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>
}
impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");

        //let epics = self.db.deref().database.deref().read_db()?.epics;
        let epics = self.db.read_db()?.epics;

        epics.keys()
            .sorted()
            .map(|epic_id| (epic_id, &epics[epic_id]))
            .map(|(epic_id, epic)| (epic_id, &epic.name, &epic.status))
            .map(|(epic_id, epic_name, epic_status) | (get_column_string(&epic_id.to_string(), 12) , get_column_string(&epic_name, 34), get_column_string(&format!("{}",epic_status), 18)))
            .for_each(|(epic_id, epic_name, epic_status)| println!("{}|{}|{}", epic_id, epic_name, epic_status));

        println!();
        println!();

        println!("[q] quit | [c] create epic | [:id:] navigate to epic");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        // match against the user input and return the corresponding action. If the user input was invalid return None.
        match input {
            "q" => Ok(Some(Action::Exit)),
            "c"  => Ok(Some(Action::CreateEpic)),
            other => {
                let parsed_input = other.parse::<u32>();
                match parsed_input {
                    Ok(parsed_input) => {
                        let read_db = self.db.read_db()?;
                        let epic = read_db.epics.get(&parsed_input);
                        match epic {
                            Some(_) => Ok(Some(Action::NavigateToEpicDetail { epic_id: parsed_input })),
                            None => Ok(None),
                        }
                    },
                    Err(_) => Ok(None),

                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EpicDetail {
    pub epic_id: u32,
    pub db: Rc<JiraDatabase>
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epic = db_state.epics.get(&self.epic_id).ok_or_else(|| anyhow!("could not find epic!"))?;

        println!("------------------------------ EPIC ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        // TODO: print out epic details using get_column_string()
        println!("{}|{}|{}|{}", 
            get_column_string(&self.epic_id.to_string(), 6), 
            get_column_string(&epic.name, 14), 
            get_column_string(&epic.description, 29), 
            get_column_string(&format!("{}",epic.status),14)
        );
  
        println!();

        println!("---------------------------- STORIES ----------------------------");
        println!("     id     |               name               |      status      ");

        let stories = &db_state.stories;

        // TODO: print out stories using get_column_string(). also make sure the stories are sorted by id
        stories.keys()
            .sorted()
            .map(|story_id| (story_id, &stories[story_id]))
            .map(|(story_id, story)| (story_id, &story.name, &story.status))
            .map(|(story_id, story_name, story_status) | (get_column_string(&story_id.to_string(), 12) , get_column_string(&story_name, 34), get_column_string(&format!("{}",story_status), 18)))
            .for_each(|(story_id, story_name, story_status)| println!("{}|{}|{}", story_id, story_name, story_status));

        println!();
        println!();

        println!("[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        // match against the user input and return the corresponding action. If the user input was invalid return None.
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u"  => Ok(Some(Action::UpdateEpicStatus { epic_id: self.epic_id })),
            "d" => Ok(Some(Action:: DeleteEpic { epic_id: self.epic_id })),
            "c" => Ok(Some(Action::CreateStory { epic_id: self.epic_id })),
            other => {
                let parsed_input = other.parse::<u32>();
                match parsed_input {
                    Ok(parsed_input) => {
                        let read_db = self.db.read_db()?;
                        let story = read_db.stories.get(&parsed_input);
                        match story {
                            Some(_) => Ok(Some(Action::NavigateToStoryDetail { epic_id: self.epic_id, story_id: parsed_input })),
                            None => Ok(None),
                        }
                    },
                    Err(_) => Ok(None),

                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct StoryDetail {
    pub epic_id: u32,
    pub story_id: u32,
    pub db: Rc<JiraDatabase>
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let story = db_state.stories.get(&self.story_id).ok_or_else(|| anyhow!("could not find story!"))?;

        println!("------------------------------ STORY ------------------------------");
        println!("  id  |     name     |         description         |    status    ");
        
        // TODO: print out story details using get_column_string()
        println!("{}|{}|{}|{}", 
            get_column_string(&self.story_id.to_string(), 6), 
            get_column_string(&story.name, 14), 
            get_column_string(&story.description, 29), 
            get_column_string(&format!("{}",story.status),14)
        );
        
        println!();
        println!();

        println!("[p] previous | [u] update story | [d] delete story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        // match against the user input and return the corresponding action. If the user input was invalid return None.
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u"  => Ok(Some(Action::UpdateStoryStatus { story_id: (self.story_id) })),
            "d" => Ok(Some(Action:: DeleteStory { epic_id: self.epic_id, story_id: self.story_id } )),
            _ => Ok(None),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::test_utils::MockDB};
    use crate::models::{Epic, Story};

    mod home_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { db };
            assert_eq!(page.draw_page().is_ok(), true);
        }
        
        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = HomePage { db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic = Epic::new("".to_owned(), "".to_owned());

            let epic_id = db.create_epic(epic).unwrap();

            let page = HomePage { db };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(page.handle_input(&valid_epic_id).unwrap(), Some(Action::NavigateToEpicDetail { epic_id: 1 }));
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        }
    }

    mod epic_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let page = EpicDetail { epic_id: 999, db };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = EpicDetail { epic_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let c = "c";
            let invalid_story_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(page.handle_input(u).unwrap(), Some(Action::UpdateEpicStatus { epic_id: 1 }));
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteEpic { epic_id: 1 }));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateStory { epic_id: 1 }));
            assert_eq!(page.handle_input(&story_id.to_string()).unwrap(), Some(Action::NavigateToStoryDetail { epic_id: 1, story_id: 2 }));
            assert_eq!(page.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        } 
    }

    mod story_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_story_id() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let _ = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id: 999, db };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });

            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();

            let page = StoryDetail { epic_id, story_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let some_number = "1";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(page.handle_input(u).unwrap(), Some(Action::UpdateStoryStatus { story_id }));
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteStory { epic_id, story_id }));
            assert_eq!(page.handle_input(some_number).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        } 
    }
}