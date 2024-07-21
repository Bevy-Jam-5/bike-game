macro_rules! single {
    ($query:expr) => {
        match $query.get_single() {
            Ok(value) => value,
            Err(_error) => {
                return Default::default();
            }
        }
    };
}

macro_rules! single_mut {
    ($query:expr) => {
        match $query.get_single_mut() {
            Ok(value) => value,
            Err(_error) => {
                return Default::default();
            }
        }
    };
}

pub(crate) use single;
pub(crate) use single_mut;
