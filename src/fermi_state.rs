use crate::models::item::ItemCard;
use fermi::Atom;

pub static mut LEFT_VEC: Atom<Vec<ItemCard>> = Atom(|_| vec![]);
pub static mut RIGHT_VEC: Atom<Vec<ItemCard>> = Atom(|_| vec![]);
