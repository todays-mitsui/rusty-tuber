use std::collections::HashSet;

/// ラムダ式や関数定義における識別子を表現する
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ident(String);

impl Ident {
    pub fn new(s: &str) -> Ident {
        Ident(String::from(s))
    }

    pub fn new_name(&self, vars: &HashSet<Ident>) -> Ident {
        let mut name = self.0.to_uppercase();

        if !vars.contains(&Ident(name.clone())) {
            return Ident(name);
        }

        let mut i = 0;
        while vars.contains(&Ident(name.clone())) {
            name = format!("{}{}", self.0.to_uppercase(), i);
            i += 1;
        }
        Ident(name)
    }

    pub fn label(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Ident::new(s)
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[test]
fn test_new_name() {
    let mut set: HashSet<Ident> = HashSet::new();

    set.insert("x".into());
    set.insert("X".into());
    set.insert("X0".into());
    set.insert("X1".into());
    set.insert("X2".into());
    set.insert("X3".into());
    set.insert("X4".into());
    set.insert("X5".into());
    set.insert("X7".into());

    assert_eq!(Ident::new("x").new_name(&set), Ident::new("X6"));
    assert_eq!(Ident::new("X").new_name(&set), Ident::new("X6"));
    assert_eq!(Ident::new("y").new_name(&set), Ident::new("Y"));
}
