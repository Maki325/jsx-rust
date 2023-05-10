use syn::{parse::ParseStream, Ident, Result, Token};

#[derive(Debug, PartialEq, Eq)]
enum Separator {
  None,
  Colon,
  DoubleColon,
  Dash,
}

impl Separator {
  fn allowed(&self, allowed: &Separator) -> bool {
    if self == &Separator::None {
      return true;
    }
    return self == allowed;
  }

  fn as_str(&self) -> &'static str {
    return match self {
      Separator::None => "",
      Separator::Colon => ":",
      Separator::DoubleColon => "::",
      Separator::Dash => "-",
    };
  }
}

pub fn get_name(input: &ParseStream) -> Result<String> {
  let mut name: Vec<String> = vec![];
  let mut separator = Separator::None;
  let mut parse_ident = true;

  macro_rules! peek_sep {
    ($($token:tt, $sep:tt),+) => {
      $(if input.peek(Token![$token]) {
        if parse_ident {
          return Err(syn::Error::new(
            input.span(),
            "Cannot have separator one after another!",
          ));
        }
        parse_ident = true;
        if !separator.allowed(&Separator::$sep) {
          return Err(syn::Error::new(
            input.span(),
            "Cannot use different separatos in the element name",
          ));
        }
        separator = Separator::$sep;
        input.parse::<Token![$token]>()?;
        continue;
      })*
    };
  }

  loop {
    if input.peek(Ident) {
      if !parse_ident {
        break;
      }
      parse_ident = false;
      let ident: Ident = input.parse()?;
      name.push(ident.to_string());
      continue;
    }
    peek_sep!(
      ::, DoubleColon,
      -, Dash,
      :, Colon
    );
    break;
  }

  return Ok(name.join(separator.as_str()));
}
