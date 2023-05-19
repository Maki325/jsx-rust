use super::{element::ElementValue, Element};
use crate::{attribute::Attribute, utils};
use syn::{
  parse::{Parse, ParseStream},
  spanned::Spanned,
  Block, Expr, Lit, Result, Stmt, Token,
};

impl Element {
  fn parse_element(input: ParseStream) -> Result<Self> {
    input.parse::<Token![<]>()?;
    let name: String = utils::get_name(&input)?;

    let mut attributes = vec![];

    loop {
      let attribute: Attribute = match input.parse() {
        Ok(attribute) => attribute,
        Err(_) => break,
      };
      attributes.push(attribute);
    }

    let self_closing = input.peek(Token![/]);
    if self_closing {
      input.parse::<Token![/]>()?;
    }
    input.parse::<Token![>]>()?;

    if self_closing {
      return Ok(Element::Element(ElementValue::new_with_attributes(
        name.to_string(),
        attributes,
      )));
    }

    let mut children: Vec<Element> = vec![];

    while !input.is_empty() {
      if input.peek(Token![<]) && input.peek2(Token![/]) {
        break;
      }
      let child: Element = input.parse()?;
      children.push(child.into());
    }

    input.parse::<Token![<]>()?;
    input.parse::<Token![/]>()?;
    let closing: String = utils::get_name(&input)?;

    if !closing.eq(&name) {
      return Err(syn::Error::new(
        closing.span(),
        "Closing tag does not match opening tag",
      ));
    }
    input.parse::<Token![>]>()?;

    return Ok(Element::Element(
      ElementValue::new_with_attributes_and_children(name.to_string(), attributes, children),
    ));
  }

  fn parse_literal(input: ParseStream) -> Result<Self> {
    if !input.peek(Lit) {
      return Err(syn::Error::new(
        input.span(),
        "Expected a Literal or an Element",
      ));
    }
    let lit: Lit = input.parse()?;

    match lit {
      Lit::Str(value) => return Ok(Element::Literal(value.value())),
      Lit::ByteStr(value) => {
        let value = match std::str::from_utf8(&value.value()[..]) {
          Ok(value) => value.to_string(),
          Err(_) => return Err(syn::Error::new(value.span(), "Invalid UTF-8 byte string")),
        };
        return Ok(Element::Literal(value));
      }
      Lit::Byte(value) => return Ok(Element::Literal(value.value().to_string())),
      Lit::Char(value) => return Ok(Element::Literal(value.value().to_string())),
      Lit::Int(value) => return Ok(Element::Literal(value.base10_digits().to_string())),
      Lit::Float(value) => return Ok(Element::Literal(value.base10_digits().to_string())),
      Lit::Bool(value) => return Ok(Element::Literal(value.value().to_string())),
      Lit::Verbatim(value) => return Ok(Element::Literal(value.to_string())),
      _ => return Err(syn::Error::new(lit.span(), "Unknown literal type")),
    };
  }

  fn parse_expr(input: ParseStream) -> Result<Self> {
    let mut block: Block = input.parse()?;

    // Like IDK if I should support multiple statements
    if block.stmts.len() != 1 {
      return Err(syn::Error::new(
        block.brace_token.span.span(),
        "Expected a single statement",
      ));
    }
    let stmt = block.stmts.pop().expect("Should exist");

    let expr = match stmt {
      Stmt::Expr(expr, _) => expr,
      _ => return Err(syn::Error::new(stmt.span(), "Expected an Expr")),
    };

    let path = match expr {
      Expr::Path(path) => path,
      _ => {
        return Err(syn::Error::new(
          expr.span(),
          "Expected a Path (i.e. variable ident)",
        ))
      }
    };

    return Ok(Element::Updateable(path.into()));
  }
}

// Used to check the token we get, cause I need to be sure
#[allow(dead_code)]
pub fn expr_type(expr: &Expr) -> &'static str {
  return match expr {
    Expr::Array(_) => "Array",
    Expr::Assign(_) => "Assign",
    Expr::Async(_) => "Async",
    Expr::Await(_) => "Await",
    Expr::Binary(_) => "Binary",
    Expr::Block(_) => "Block",
    Expr::Break(_) => "Break",
    Expr::Call(_) => "Call",
    Expr::Cast(_) => "Cast",
    Expr::Closure(_) => "Closure",
    Expr::Const(_) => "Const",
    Expr::Continue(_) => "Continue",
    Expr::Field(_) => "Field",
    Expr::ForLoop(_) => "ForLoop",
    Expr::Group(_) => "Group",
    Expr::If(_) => "If",
    Expr::Index(_) => "Index",
    Expr::Infer(_) => "Infer",
    Expr::Let(_) => "Let",
    Expr::Lit(_) => "Lit",
    Expr::Loop(_) => "Loop",
    Expr::Macro(_) => "Macro",
    Expr::Match(_) => "Match",
    Expr::MethodCall(_) => "MethodCall",
    Expr::Paren(_) => "Paren",
    Expr::Path(_) => "Path",
    Expr::Range(_) => "Range",
    Expr::Reference(_) => "Reference",
    Expr::Repeat(_) => "Repeat",
    Expr::Return(_) => "Return",
    Expr::Struct(_) => "Struct",
    Expr::Try(_) => "Try",
    Expr::TryBlock(_) => "TryBlock",
    Expr::Tuple(_) => "Tuple",
    Expr::Unary(_) => "Unary",
    Expr::Unsafe(_) => "Unsafe",
    Expr::Verbatim(_) => "Verbatim",
    Expr::While(_) => "While",
    Expr::Yield(_) => "Yield",
    _ => "Unknown",
  };
}

impl Parse for Element {
  fn parse(input: ParseStream) -> Result<Self> {
    if input.peek(Token![<]) {
      return Element::parse_element(input);
    }
    if input.peek(Lit) {
      return Element::parse_literal(input);
    }
    return Element::parse_expr(input);
  }
}
