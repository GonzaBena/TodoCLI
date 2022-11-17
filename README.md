# Todo CLI

## Description: Todo CLI is a terminal ToDo List with users, lists and other features

## - Dependencies

``` toml
[dependencies]
colored = "0.2.0"
```

## `Use` to add

### [main.rs]

``` rust
use std::collections::HashMap;
use std::io::{self, Write};
```

### [lib.rs]

``` rust
use std::collections::HashMap;
use std::io::{self, Write};
```

## - Structs

### User

[**Signature**]

```rust
#[derive(Debug, PartialEq)] // With this, you can realize the tests
pub struct User {
    pub name: String,
    pub pass: String,
}
```

[**User's Methods**]

``` rust
pub fn new(name: String, pass: String) -> User;
pub fn change_pass(&mut self, pass: String);
```

### Task

[**Signature**]

``` rust
#[derive(Debug, PartialEq)] // With this, you can realize the tests
pub struct Task {
    pub id: u32,
    pub description: String,
    pub user: User,
    pub status: bool,
}
```

[**Task's Methods**]

``` rust
pub fn new(id: &mut u32, description: String, user: User, status: bool) -> Task;
pub fn belongs_to(&self, user: &User) -> bool;
```

### UserList

[**Signature**]

``` rust
#[derive(Debug, PartialEq)] // With this, you can realize the tests
pub struct UserList {
    pub users: HashMap<String, String>
}
```

[**UserList's Methods**]

``` rust
pub fn new() -> UserList;
pub fn login(&self) -> bool;
pub fn register(&mut self) -> bool;
```

### TodoList

[**Signature**]

``` rust
#[derive(Debug, PartialEq)] // With this, you can realize the tests
pub struct TodoList {
    pub tasks: Vec<Task>,
}
```

[**TodoList's Methods**]

``` rust
pub fn new() -> TodoList;
pub fn from(tasks: Vec<Task>) -> TodoList;
pub fn insert(&mut self, task: Task);
pub fn show_all(&self);
pub fn find_all(&self, user: &User) -> Vec<&Task>;
pub fn find_one(&self, user: &User) -> &Task;
```
