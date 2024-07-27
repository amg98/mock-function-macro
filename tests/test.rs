use mock_function_macro::mock_fn;

#[mock_fn]
fn my_fun(a: i32, _b: i32) -> i32 {
    a
}

#[test]
fn test_mock_fn() {
    // Arrange
    set_mock_for_my_fun(|a: i32, b: i32| -> i32 {
        assert_eq!(a, 2);
        assert_eq!(b, 3);
        5
    });

    // Act
    let result = my_fun(2, 3);
    
    // Assert
    assert_eq!(result, 5);
    clear_mock_for_my_fun();
}

#[test]
#[should_panic(expected = "No mock has been set")]
fn test_panics_if_mock_not_set() {
    // Act
    my_fun(2, 3);
}

#[test]
#[should_panic(expected = "You forgot to clear this mock")] 
fn test_panics_if_forgot_to_clear_mock() {
    // Arrange
    set_mock_for_my_fun(|a: i32, b: i32| -> i32 {
        assert_eq!(a, 2);
        assert_eq!(b, 3);
        5
    });

    // Act
    set_mock_for_my_fun(|a: i32, b: i32| -> i32 { a - b });
}


#[test]
#[should_panic(expected = "This mock is already cleared")]
fn test_panics_if_clearing_mock_twice() {
    clear_mock_for_my_fun();
}

