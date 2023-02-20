#[cfg(test)]
mod master_group_tests;

#[cfg(test)]
mod prepare_data_tests;

#[cfg(test)]
mod update_data_tests;

#[cfg(test)]
mod mint_root_tests;

#[cfg(test)]
mod helpers;


/* Here we could have tests in ::root, or we could create modules here as well, but then we would need to do imports
#[test]
fn test_something() {
    assert_eq!(2,2, "Two should be two");
}


#[cfg(test)]
mod main {
    use near_sdk::test_utils::accounts;

    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(5,5, "Five should be Five");
    }
}*/