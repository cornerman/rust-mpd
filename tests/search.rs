extern crate mpd;

mod helpers;
use helpers::connect;
use mpd::search::Query;

#[test]
fn search() {
    let mut mpd = connect();
    let mut query = Query::new();
    query.filter(mpd::Term::Any, "Soul");
    query.limit(1);
    query.window(1, 2);
    query.fuzzy();
    let songs = mpd.find(query);
    println!("{:?}", songs);
}

/*
#[test]
fn count() {
    let mut mpd = connect();
    let song = mpd.search(mpd::Query {
        clauses: vec![mpd::Clause(mpd::Term::Any, "Soul".to_owned())],
        window: None,
        group: None
    }).unwrap();
    println!("{:?}", song);
}
*/
