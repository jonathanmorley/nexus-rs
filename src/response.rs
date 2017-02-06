use ::Client;

use std::fmt::{Debug, Error, Formatter};
use std::ops::Deref;
use std::vec::IntoIter;
use std::convert::Into;

#[derive(Clone)]
pub struct Response<'a, T> {
    pub client: &'a Client,
    pub item: T,
}

impl<'a, T: Debug> Debug for Response<'a, T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        self.item.fmt(formatter)
    }
}

impl<'a, T> Deref for Response<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.item
    }
}

impl<'a, T: PartialEq> PartialEq for Response<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.item.eq(&other.item)
    }
}

impl<'a, T> Into<Vec<Response<'a, T>>> for Response<'a, Vec<T>> {
    fn into(self) -> Vec<Response<'a, T>> {
        let client = self.client;
        self.item.into_iter().map(move |elem| Response { client: client, item: elem }).collect()
    }
}

impl<'a, T> IntoIterator for Response<'a, Vec<T>> {
    type Item = Response<'a, T>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let vec: Vec<Self::Item> = self.into();
        vec.into_iter()
    }
}
