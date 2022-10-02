use super::traits::ast_term::ASTTerm;
use super::traits::module_term_type::ModuleTermType;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_term_type::ServiceTermType;
use super::traits::service_usable_term::ServiceUsableTerm;

pub struct EntityTerm {
  name: Box<dyn ASTTerm>,
  applied_aspects: Vec<Box<dyn ASTTerm>>,
  fields: Vec<Box<dyn ASTTerm>>,
}

impl ASTTerm for EntityTerm {}

impl ModuleUsableTerm for EntityTerm {
  fn get_type(&self) -> ModuleTermType {
    ModuleTermType::Entity
  }
}

impl ServiceUsableTerm for EntityTerm {
  fn get_type(&self) -> ServiceTermType {
    ServiceTermType::Entity
  }
}

impl EntityTerm {
  pub fn new_boxed(
    name: Box<dyn ASTTerm>,
    applied_aspects: Vec<Box<dyn ASTTerm>>,
    fields: Vec<Box<dyn ASTTerm>>,
  ) -> Box<EntityTerm> {
    Box::new(EntityTerm::new(name, applied_aspects, fields))
  }

  pub fn new(
    name: Box<dyn ASTTerm>,
    applied_aspects: Vec<Box<dyn ASTTerm>>,
    fields: Vec<Box<dyn ASTTerm>>,
  ) -> EntityTerm {
    EntityTerm {
      name,
      applied_aspects,
      fields,
    }
  }
}
