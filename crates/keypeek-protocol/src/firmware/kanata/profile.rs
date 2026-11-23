use super::KanataPresenter;
use keypeek_core::keymap_editor::{
    CandidateGroup, EditorProfile, EditorSection, FeatureManifest, LayerTapTarget, SidebarSection,
};
use keypeek_core::{KeyPresenter, KeySpec, LayoutKey};

pub struct KanataEditorProfile;

impl KeyPresenter for KanataEditorProfile {
    fn present_key(&self, spec: &KeySpec, layer_names: &[String]) -> Option<LayoutKey> {
        KanataPresenter.present_key(spec, layer_names)
    }
}

impl EditorProfile for KanataEditorProfile {
    fn presenter(&self) -> &dyn KeyPresenter {
        &KanataPresenter
    }

    fn name(&self) -> &'static str {
        "Kanata"
    }

    fn sidebar_sections(&self) -> &[SidebarSection<FeatureManifest>] {
        &[]
    }

    fn tap_categories(&self) -> &'static [CandidateGroup] {
        &[]
    }

    fn section_groups(&self, _section: EditorSection) -> &'static [CandidateGroup] {
        &[]
    }

    fn layer_groups(
        &self,
        _layer_count: usize,
        _layer_names: &[String],
        _tap: LayerTapTarget,
    ) -> Vec<CandidateGroup> {
        Vec::new()
    }
}
