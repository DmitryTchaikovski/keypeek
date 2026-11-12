use keypeek_core::{CustomKind, KeyPresenter, KeySpec, Label, LayoutKey};

#[derive(Debug, Default, Clone, Copy)]
pub struct KanataPresenter;

impl KeyPresenter for KanataPresenter {
    fn present_key(&self, spec: &KeySpec, layer_names: &[String]) -> Option<LayoutKey> {
        match spec {
            KeySpec::Custom(binding) if binding.kind == CustomKind::Raw => {
                let name = binding.name.as_deref()?;
                let short = name.chars().take(7).collect::<String>();
                Some(LayoutKey {
                    tap: Label::with_short(name, short),
                    ..Default::default()
                })
            }
            _ => crate::key_presenter::StandardKeyPresenter.present_key(spec, layer_names),
        }
    }
}
