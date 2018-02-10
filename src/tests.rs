use super::init_rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn hello_world() {
    let client = Client::new(init_rocket()).expect("valid rocket instance");
    let mut response = client.get("/alive").dispatch();
    assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.body_string(), Some("Hello, world!".into()));
}

// TODO: For testing, we can use either a test_transaction, or an in-memory database

// TODO: test_transaction http://docs.diesel.rs/diesel/connection/trait.Connection.html#method.begin_test_transaction

// TODO: in-memory database https://github.com/diesel-rs/diesel/issues/419 (basically use ":memory:" as the connection string

describe! route_tests {
    before_each {
        let rocket = init_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
    }

    describe! alive {
        before_each {
            let mut res = client.get("/alive").dispatch();
            let body_str = res.body().and_then(|b| b.into_string()).unwrap();
        }

        it "responds with status OK 200" {
            assert_eq!(res.status(), Status::Ok);
        }

        it "responds with current year" {
            assert!(body_str.contains("2018"));
        }
    }

    describe! nested_example {
        ignore "this is ignored" {
            assert_eq!(1, 2);
        }

        failing "this fails" {
            assert_eq!(1, 2);
        }
    }
}
