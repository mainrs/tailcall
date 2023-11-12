use crate::blueprint::from_config2::types::Transform;
use crate::blueprint::from_config2::Ctx;
use crate::blueprint::FieldDefinition;
use crate::config;
use crate::config::Field;
use crate::lambda::Lambda;
use crate::valid::{Valid, ValidStructCompatibility};

pub struct ModifyTransform {
  r#type: config::Type,
}

impl Transform for ModifyTransform {
  type Input = Field;
  type Output = FieldDefinition;
  type Error = String;

  fn transform(self, input: &Self::Input, mut value: Self::Output, ctx: &dyn Ctx) -> Valid<Self::Output, Self::Error> {
    // We define a capturing lambda that contains our main logic. When calling the lambda, we can
    // now easily add a `trace` method call without having to add the call to each possible
    // return value branch.
    let inner = move || {
      // Early return for the case that there is no `@modify` directive.
      let Some(modify_field) = &input.modify else {
        return Ok(value);
      };

      // We only process `@modify` directives that contain a non-null `name` attribute.
      let Some(new_name) = modify_field.name.as_ref() else {
        return Ok(value);
      };

      // For the given `name` attribute value, we ensure that the field we try to modify is
      // not defined as part of an interface. We can only modify fields of a type definition.
      for name in self.r#type.implements.iter() {
        let interface = ctx.find_type(name);
        if let Some(interface) = interface {
          if interface.fields.iter().any(|(name, _)| name == new_name) {
            return Valid::fail("Field is already implemented by interface".to_owned());
          }
        }
      }

      // We now know that the `@modify` directive has been applied correctly. We create a
      // matching identity resolver if non is present already, since we do not modify the
      // underlying data.
      //
      // We also change the name of the GraphQL attribute.
      let lambda = Lambda::context_field(value.name);
      value = value.resolver_or_default(lambda, |r| r);
      value = value.name(new_name.clone());
      Ok(value)
    };

    inner().trace("@modify")
  }
}
