use mock_function_macro::mock_fn;
use utils::MockDisposeBag;

#[mock_fn]
fn my_fun(a: i32, _b: i32) -> i32 {
    a
}

#[test]
fn test_mock_fn() {
    // Arrange
    let mut bag = MockDisposeBag::new();
    bag.add(mock_my_fun(|a: i32, b: i32| -> i32 {
        assert_eq!(a, 2);
        assert_eq!(b, 3);
        5
    }));

    // Act
    let result = my_fun(2, 3);

    // Assert
    assert_eq!(result, 5);
}

#[test]
#[should_panic(expected = "No mock has been set")]
fn test_panics_if_mock_not_set() {
    // Act
    my_fun(2, 3);
}

#[test]
#[should_panic(expected = "No mock has been set")]
fn test_panics_if_mock_not_set_after_test() {
    // Arrange
    {
        let mut bag = MockDisposeBag::new();
        bag.add(mock_my_fun(|a: i32, b: i32| -> i32 {
            assert_eq!(a, 2);
            assert_eq!(b, 3);
            5
        }));
    }

    // Act
    my_fun(2, 3);
}
