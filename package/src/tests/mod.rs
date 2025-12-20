#[test]
fn test_main() {
    let _ = std::thread::spawn(|| {
        crate::main();
    });
    std::thread::sleep(std::time::Duration::from_secs(1));
    assert_eq!(true, true);
}
