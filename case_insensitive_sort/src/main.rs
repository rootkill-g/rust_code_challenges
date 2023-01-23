fn sort_usernames_ci<T: AsRef<str> + Ord>(users: &mut Vec<T>) {
    users.sort_by_cached_key(|x| x.as_ref().to_lowercase());
}

fn main() {
    let mut users = vec!["Todd", "amy"];
    println!("Unsorted: {:?}", &users);
    sort_usernames_ci(&mut users);
    println!("Sorted: {:?}", &users);
}

#[test]
fn test_sort() {
    let mut users = vec![
        "rootkill",
        "Boros1ck",
        "Wellick",
        "mrrobot"
    ];

    sort_usernames_ci(&mut users);
    assert_eq!(users, vec!["Boros1ck", "mrrobot", "rootkill", "Wellick"]);
}
