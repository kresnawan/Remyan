use crate::page::Pages;

#[derive(Clone)]
pub enum State {
    MovePage(Pages),
    OpenDialogueBox(u8),
    CloseDialogueBox(u8)
}