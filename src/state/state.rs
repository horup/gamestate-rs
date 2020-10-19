use super::Things;

#[derive(Clone)]
pub struct State<T> where T : Clone + PartialEq + Copy + Default
{
    pub things:Things<T>
}

impl<T> State<T> where T : Clone + PartialEq + Copy + Default
{
    pub fn new() -> State<T>
    {
        State {
            things:Things::new()
        }
    }
}