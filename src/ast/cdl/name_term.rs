use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(visitor_path = "process_name")]
pub struct NameTerm {
  #[prop]
  value: String,
}
