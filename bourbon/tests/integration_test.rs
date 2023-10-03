use bourbon::prelude::*;

mod funcs {
    pub fn lower<E>(s: String) -> Result<String, E> {
        Ok(s.as_str().to_lowercase())
    }

    pub fn upper<E>(s: String) -> Result<String, E> {
        Ok(s.as_str().to_uppercase())
    }
}

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

#[template(text = "Hello, {lower(name)}, {upper(name)}!")]
struct TestTemplateFunc {
    name: String,
}

#[test]
fn test_render_func() {
    let values = TestTemplateFunc::new("Alice".to_string());
    assert_eq!(values.render().unwrap(), "Hello, alice, ALICE!");
}

#[template(text = "{name}")]
struct TestTemplateBlock {
    name: String,
}

#[template(text = "Hello, {name}, {name2}!")]
struct TestTemplateWithBlock {
    name: String,
    name2: TestTemplateBlock,
}

#[test]
fn test_render_block() {
    let values = TestTemplateWithBlock::new(
        ("Alice".to_string(),
            TestTemplateBlock::new("Bob".to_string())));
    assert_eq!(values.render().unwrap(), "Hello, Alice, Bob!");
}
