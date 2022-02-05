use thiserror::Error;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Not a valid molecule")]
    InvalidMolecule,
    #[error("Mismatched parenthesis")]
    BrokenParent,
}

fn add_atom(m: &mut Molecule, atom: Atom) {
    match m.iter_mut().find(|(a, _)| *a == atom.0) {
        Some((_, c)) => *c += atom.1,
        _ => m.push(atom),
    }
}
fn mul_molecule(m: &mut Molecule, f: usize) {
    for (_, s) in m.iter_mut() {
        *s *= f
    }
}
fn merge_molecules(m: &mut Molecule, rhs: &Molecule) {
    for atom in rhs.iter() {
        add_atom(m, atom.clone())
    }
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut rest = s;
    let mut molecule = Molecule::new();
    let mut last_mol = Molecule::new();

    while !rest.is_empty() {
        match rest.chars().nth(0) {
            Some(m) if m.is_uppercase() => {
                let streak = rest
                    .chars()
                    .skip(1)
                    .take_while(|c| c.is_lowercase())
                    .collect::<String>();
                rest = &rest[(1 + streak.len())..];
                merge_molecules(&mut molecule, &last_mol);
                last_mol = Molecule::new();
                add_atom(&mut last_mol, (format!("{}{}", m, streak), 1));
            }
            Some(n) if n.is_numeric() => {
                let end_i = match rest.find(|x: char| !x.is_numeric()) {
                    Some(n) => n,
                    None => rest.len(),
                };
                mul_molecule(&mut last_mol, rest[0..end_i].parse().unwrap());
                rest = &rest[end_i..];
            }
            Some(c @ ('(' | '[' | '{')) => {
                let mut inner = 0;
                let end_s = match c {
                    '[' => ']',
                    '(' => ')',
                    _ => '}',
                };
                let end_i = match rest.chars().position(|x| {
                    if x == c { inner +=1 } else if x == end_s { inner -=1 } 
                    inner == 0 && x == end_s
                }) {
                    Some(n) => n,
                    None => return Err(ParseError::BrokenParent),
                };
                merge_molecules(&mut molecule, &last_mol);
                last_mol = parse_molecule(&rest[1..end_i])?;
                rest = &rest[(end_i + 1)..];
            }
            Some(x) => {
                if x.is_lowercase() {
                    return Err(ParseError::InvalidMolecule);
                } else {
                    return Err(ParseError::BrokenParent);
                }
            }
            None => {}
        }
    }
    merge_molecules(&mut molecule, &last_mol);

    Ok(molecule)
}

#[cfg(test)]
mod tests {
    use super::{parse_molecule, Molecule};

    macro_rules! assert_parse {
        ($formula:expr, $expected:expr, $name:ident) => {
            #[test]
            fn $name() {
                super::assert_parse($formula, &$expected, "");
            }
        };
    }

    mod molecules {
        assert_parse!("H", [("H", 1)], hydrogen);
        assert_parse!("O2", [("O", 2)], oxygen);
        assert_parse!("H2O", [("H", 2), ("O", 1)], water);
        assert_parse!(
            "Mg(OH)2",
            [("Mg", 1), ("O", 2), ("H", 2)],
            magnesium_hydroxide
        );
        assert_parse!(
            "K4[ON(SO3)2]2",
            [("K", 4), ("O", 14), ("N", 2), ("S", 4)],
            fremys_salt
        );
    }

    #[test]
    fn errors() {
        assert_fail("pie", "Not a valid molecule");
        assert_fail("Mg(OH", "Mismatched parenthesis");
        assert_fail("Mg(OH}2", "Mismatched parenthesis");
    }

    fn assert_fail(formula: &str, msg: &str) {
        let result = parse_molecule(formula);
        assert!(
            result.is_err(),
            format!(
                "expected {} {:?} to fail, got {:?}",
                msg,
                formula,
                result.unwrap()
            )
        );
    }

    fn assert_parse(formula: &str, expected: &[(&str, usize)], _mst: &str) {
        let mut expected = expected
            .into_iter()
            .map(|&(name, usize)| (name.to_owned(), usize))
            .collect::<Molecule>();
        let result = parse_molecule(formula);
        assert!(
            result.is_ok(),
            format!("expected {:?} to pass, got {:?}", formula, result)
        );
        let mut actual = result.unwrap();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_parse_molecule() {
    println!("{:?}", parse_molecule("({(Fl)14})17").unwrap())
}
