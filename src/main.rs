pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    println!("chekcing: prefix={} request={}", prefix, request_path);
    if prefix == "" {
        return true;
    }
    if request_path.len() < prefix.len() {
        return false;
    }

    if let Some(idx) = prefix.find("*") {
        println!("found {:?} at {}", prefix, idx);
        if request_path[..idx] != prefix[..idx] {
            return false;
        }
        let (_, after) = request_path.split_at(idx);
        if let Some(slashidx) = after.find("/") {
            println!(
                "request after split is {:} and it has slash at {}",
                after, slashidx
            );
            return prefix_matches(&prefix[idx + 1..], &after[slashidx..]);
        } else {
            return true;
        }
    } else {
        if prefix == request_path {
            return true;
        }
        let res = request_path.find(prefix);
        println!(
            "Checking if {} contains prefix {}: {:?}",
            request_path, prefix, res,
        );
        match res {
            Some(_) => {
                request_path
                    .chars()
                    .skip(prefix.len())
                    .next()
                    .unwrap_or('.')
                    == '/'
            }
            None => false,
        }
    }
}

fn main() {
    println!(
        "{:}",
        prefix_matches("/my/*/house", "/my/goo/house/in/play"),
    );
}

#[test]
fn test_matches_without_wildcard_same() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
}
#[test]
fn test_matches_without_wildcard_123() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
}
#[test]
fn test_matches_without_wildcard_ok() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));
}

#[test]
fn test_matches_without_wildcard_v1() {
    assert!(!prefix_matches("/v1/publishers", "/v1"));
}
#[test]
fn test_matches_without_wildcard_noslash() {
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
}
#[test]
fn test_matches_without_wildcard_invalid() {
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

#[test]
fn request_shorter() {
    assert!(!prefix_matches("/v1/publishers/*", "/v1/publishers/"));
    assert!(!prefix_matches("/v1/publishers/*", "/v1/publishers"));
}
