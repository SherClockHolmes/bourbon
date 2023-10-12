# bourbon
Templating on the rocks.

```rust
use bourbon::prelude::*;

#[template(text = "Hello, {name}! You are {age} years old.")]
struct TestTemplate {
    name: String,
    age: i32,
}

#[test]
fn test_render() {
    let values = TestTemplate::new(("Alice".to_string(), 30));
    assert_eq!(values.render().unwrap(), "Hello, Alice! You are 30 years old.");
}
```

### Filters

There are no built-in template functions in bourbon, if you want to call functions in your template then add it within the funcs scope.

```rust
mod funcs {
    pub fn lower<E>(s: String) -> Result<String, E> {
        Ok(s.as_str().to_lowercase())
    }
}

#[template(text = "{lower(name)}")]
struct TestTemplate {
    name: String,
}
```

### Blocks

Unlike other template solutions that utilize an "include" or "import" keyword, bourbon facilitates the embedding of sub-templates through the composition of other template structs.

```rust
#[template(text = "{name}")]
struct TemplateBlock {
    name: String,
}

#[template(text = "Hello, {name}, {name2}!")]
struct TemplateWithBlock {
    name: String,
    name2: TemplateBlock,
}
```

### Integrations

To use `rocket` with bourbon, enable the following feature to Cargo.toml.

```yaml
bourbon = { version = "0.2.0", path = "../bourbon/bourbon", features = ["rocket"] }
```

```rust
#[template(file = "templates/index.html")]
struct TestTemplate {
    name: String,
    age: i32,
}

#[get("/")]
fn index() -> TestTemplate {
    TestTemplate::new(("Alice".to_string(), 27))
}
```
