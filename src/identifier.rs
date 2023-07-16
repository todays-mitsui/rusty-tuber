use std::collections::HashSet;

/// ラムダ式や関数定義における識別子を表現する
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(s: &str) -> Identifier {
        Identifier(String::from(s))
    }

    pub fn new_name(&self, vars: &HashSet<Identifier>) -> Identifier {
        let mut name = self.0.to_uppercase();

        if !vars.contains(&Identifier(name.clone())) {
            return Identifier(name);
        }

        let mut i = 0;
        while vars.contains(&Identifier(name.clone())) {
            name = format!("{}{}", self.0.to_uppercase(), i);
            i += 1;
        }
        Identifier(name)
    }
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Identifier::new(s)
    }
}

#[test]
fn test_new_name() {
    let mut set: HashSet<Identifier> = HashSet::new();

    set.insert("x".into());
    set.insert("X".into());
    set.insert("X0".into());
    set.insert("X1".into());
    set.insert("X2".into());
    set.insert("X3".into());
    set.insert("X4".into());
    set.insert("X5".into());
    set.insert("X7".into());

    assert_eq!(Identifier::new("x").new_name(&set), Identifier::new("X6"));
    assert_eq!(Identifier::new("y").new_name(&set), Identifier::new("Y"));
}
