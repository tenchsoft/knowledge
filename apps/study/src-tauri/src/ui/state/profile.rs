use super::*;

impl StudyState {
    pub fn complete_profile_setup(
        &mut self,
        learner_id: impl Into<String>,
        display_name: impl Into<String>,
        primary_locale: impl Into<String>,
        target_locales: Vec<String>,
    ) -> Result<(), String> {
        let learner_id = learner_id.into();
        let display_name = display_name.into();
        let primary_locale = primary_locale.into();
        if learner_id.trim().is_empty() {
            return Err("learner id is required".to_string());
        }
        if display_name.trim().is_empty() {
            return Err("display name is required".to_string());
        }
        if tench_study_core::ContentLocale::parse(&primary_locale).is_none() {
            return Err(format!("invalid locale {primary_locale}"));
        }
        let target_locales = if target_locales.is_empty() {
            vec![primary_locale.clone()]
        } else {
            target_locales
        };
        for locale in &target_locales {
            if tench_study_core::ContentLocale::parse(locale).is_none() {
                return Err(format!("invalid target locale {locale}"));
            }
        }
        self.profile_setup = ProfileSetupState {
            learner_id,
            display_name,
            primary_locale: primary_locale.clone(),
            target_locales,
            completed: true,
        };
        self.selection.locale = primary_locale;
        Ok(())
    }

    pub fn select_domain_level_locale(
        &mut self,
        domain: tench_study_core::SubjectDomain,
        level: tench_study_core::EducationLevel,
        locale: impl Into<String>,
    ) -> Result<(), String> {
        let locale = locale.into();
        if tench_study_core::ContentLocale::parse(&locale).is_none() {
            return Err(format!("invalid locale {locale}"));
        }
        let unit_idx = self
            .units
            .iter()
            .position(|unit| unit.domain == domain)
            .ok_or_else(|| "selected domain is not available".to_string())?;
        let concept_idx = self.units[unit_idx]
            .concepts
            .iter()
            .position(|concept| concept.level == level)
            .ok_or_else(|| "selected level is not available".to_string())?;
        self.selection = StudySelectionState {
            domain,
            level,
            locale,
        };
        self.select_concept(unit_idx, concept_idx);
        Ok(())
    }

    pub fn advance_profile_step(&mut self) {
        self.profile_setup_step = match self.profile_setup_step {
            ProfileSetupStep::Identity => ProfileSetupStep::DomainLevel,
            ProfileSetupStep::DomainLevel => ProfileSetupStep::Locale,
            ProfileSetupStep::Locale => {
                // Complete setup
                let result = self.complete_profile_setup(
                    self.wizard_learner_id.clone(),
                    self.wizard_display_name.clone(),
                    self.wizard_primary_locale.clone(),
                    self.wizard_target_locales.clone(),
                );
                if result.is_ok() {
                    // Apply domain/level selection
                    let domains: Vec<_> = self.units.iter().map(|u| u.domain.clone()).collect();
                    let levels: Vec<_> = tench_study_core::EducationLevel::all().to_vec();
                    if let (Some(domain), Some(level)) = (
                        domains.get(self.wizard_domain_idx),
                        levels.get(self.wizard_level_idx),
                    ) {
                        let _ = self.select_domain_level_locale(
                            domain.clone(),
                            *level,
                            self.wizard_primary_locale.clone(),
                        );
                    }
                    self.show_profile_setup_modal = false;
                    self.profile_setup_step = ProfileSetupStep::Done;
                }
                return;
            }
            ProfileSetupStep::Done => return,
        };
    }

    pub fn go_back_profile_step(&mut self) {
        self.profile_setup_step = match self.profile_setup_step {
            ProfileSetupStep::Identity => ProfileSetupStep::Identity,
            ProfileSetupStep::DomainLevel => ProfileSetupStep::Identity,
            ProfileSetupStep::Locale => ProfileSetupStep::DomainLevel,
            ProfileSetupStep::Done => ProfileSetupStep::Locale,
        };
    }
}
