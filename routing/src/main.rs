pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefixes = prefix.split("/");
    let sub_paths = request_path.
            split("/").
            map(|p| Some(p)).
            chain(std::iter::once(None));
    // "/v1/publishers/*/books", "/v1/publishers/None
    for (prefix_part, request_part)  in prefixes.zip(sub_paths) {
        match request_part {
            Some(request_part) => {
                if prefix_part != request_part && prefix_part != "*" {
                    return false
                }
            }
            None => return false
        }
    }
    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

fn main() {
    println!("{}",prefix_matches("/v1/publishers", "/v1/publishers"))
}