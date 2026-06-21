use serde::{Deserialize, Serialize};

use crate::{EducationLevel, SubjectDomain};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SubjectCatalog {
    pub tracks: Vec<SubjectTrackDefinition>,
}

impl SubjectCatalog {
    pub fn production_scope() -> Self {
        Self {
            tracks: vec![
                mathematics_track(),
                science_track(),
                language_track(),
                programming_track(),
            ],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SubjectTrackDefinition {
    pub id: String,
    pub domain: SubjectDomain,
    pub title: String,
    pub levels: Vec<EducationLevel>,
    pub strands: Vec<SubjectStrand>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SubjectStrand {
    pub id: String,
    pub title: String,
    pub level_start: EducationLevel,
    pub level_end: EducationLevel,
    pub required_content: Vec<String>,
    pub visual_expectations: Vec<String>,
}

pub fn mathematics_track() -> SubjectTrackDefinition {
    use EducationLevel::*;
    SubjectTrackDefinition {
        id: s("mathematics"),
        domain: SubjectDomain::Mathematics,
        title: s("Mathematics"),
        levels: vec![
            Kindergarten,
            ElementaryLower,
            ElementaryUpper,
            MiddleSchool,
            HighSchool,
            UndergraduateLower,
            UndergraduateUpper,
            GraduateMasters,
            GraduateDoctoral,
        ],
        strands: vec![
            SubjectStrand {
                id: s("number-sense"),
                title: s("Number sense and operations"),
                level_start: Kindergarten,
                level_end: ElementaryUpper,
                required_content: strings(&[
                    "counting",
                    "place value",
                    "addition and subtraction",
                    "multiplication and division",
                    "fractions",
                    "decimals",
                    "ratios",
                ]),
                visual_expectations: strings(&["manipulatives", "number line", "base-ten blocks"]),
            },
            SubjectStrand {
                id: s("algebra"),
                title: s("Algebra and functions"),
                level_start: MiddleSchool,
                level_end: UndergraduateLower,
                required_content: strings(&[
                    "expressions",
                    "linear equations",
                    "systems",
                    "polynomials",
                    "functions",
                    "exponential and logarithmic models",
                    "abstract algebra foundations",
                ]),
                visual_expectations: strings(&[
                    "function graph",
                    "equation balance",
                    "parameter scrubber",
                ]),
            },
            SubjectStrand {
                id: s("geometry"),
                title: s("Geometry and spatial reasoning"),
                level_start: ElementaryLower,
                level_end: UndergraduateUpper,
                required_content: strings(&[
                    "shapes",
                    "angles",
                    "congruence",
                    "similarity",
                    "trigonometry",
                    "analytic geometry",
                    "differential geometry foundations",
                ]),
                visual_expectations: strings(&[
                    "interactive construction",
                    "3d solids",
                    "proof diagram",
                ]),
            },
            SubjectStrand {
                id: s("calculus-analysis"),
                title: s("Calculus and analysis"),
                level_start: HighSchool,
                level_end: GraduateDoctoral,
                required_content: strings(&[
                    "limits",
                    "derivatives",
                    "integrals",
                    "series",
                    "multivariable calculus",
                    "real analysis",
                    "complex analysis",
                    "functional analysis",
                ]),
                visual_expectations: strings(&[
                    "curve tangent",
                    "area accumulation",
                    "field visualization",
                ]),
            },
            SubjectStrand {
                id: s("probability-statistics"),
                title: s("Probability, statistics, and data"),
                level_start: ElementaryUpper,
                level_end: GraduateDoctoral,
                required_content: strings(&[
                    "data collection",
                    "distributions",
                    "inference",
                    "regression",
                    "bayesian reasoning",
                    "stochastic processes",
                    "statistical learning theory",
                ]),
                visual_expectations: strings(&[
                    "distribution simulator",
                    "sampling animation",
                    "regression explorer",
                ]),
            },
            SubjectStrand {
                id: s("linear-discrete"),
                title: s("Linear algebra, discrete math, and computation"),
                level_start: HighSchool,
                level_end: GraduateDoctoral,
                required_content: strings(&[
                    "vectors",
                    "matrices",
                    "eigenvalues",
                    "graphs",
                    "combinatorics",
                    "optimization",
                    "numerical methods",
                ]),
                visual_expectations: strings(&[
                    "matrix transform",
                    "graph traversal",
                    "optimization landscape",
                ]),
            },
        ],
    }
}

pub fn science_track() -> SubjectTrackDefinition {
    use EducationLevel::*;
    SubjectTrackDefinition {
        id: s("science"),
        domain: SubjectDomain::Science,
        title: s("Science"),
        levels: vec![
            Kindergarten,
            ElementaryLower,
            ElementaryUpper,
            MiddleSchool,
            HighSchool,
            UndergraduateLower,
            UndergraduateUpper,
            GraduateMasters,
            GraduateDoctoral,
        ],
        strands: vec![
            SubjectStrand {
                id: s("life-science"),
                title: s("Life science and biology"),
                level_start: Kindergarten,
                level_end: GraduateDoctoral,
                required_content: strings(&[
                    "organisms",
                    "cells",
                    "genetics",
                    "evolution",
                    "ecology",
                    "anatomy",
                    "molecular biology",
                    "systems biology",
                ]),
                visual_expectations: strings(&[
                    "organ model",
                    "cell simulation",
                    "ecosystem network",
                ]),
            },
            SubjectStrand {
                id: s("physical-science"),
                title: s("Physics and chemistry"),
                level_start: ElementaryLower,
                level_end: GraduateDoctoral,
                required_content: strings(&[
                    "matter",
                    "motion",
                    "forces",
                    "energy",
                    "waves",
                    "electricity",
                    "atomic structure",
                    "thermodynamics",
                    "quantum foundations",
                ]),
                visual_expectations: strings(&[
                    "force simulation",
                    "molecule model",
                    "field lines",
                ]),
            },
            SubjectStrand {
                id: s("earth-space"),
                title: s("Earth and space science"),
                level_start: Kindergarten,
                level_end: UndergraduateUpper,
                required_content: strings(&[
                    "weather",
                    "geology",
                    "climate",
                    "astronomy",
                    "planetary systems",
                    "remote sensing",
                ]),
                visual_expectations: strings(&[
                    "orbital model",
                    "layered earth model",
                    "climate timeline",
                ]),
            },
            SubjectStrand {
                id: s("scientific-practice"),
                title: s("Scientific inquiry and lab practice"),
                level_start: ElementaryLower,
                level_end: GraduateDoctoral,
                required_content: strings(&[
                    "observation",
                    "measurement",
                    "experimental design",
                    "uncertainty",
                    "instrumentation",
                    "reproducibility",
                ]),
                visual_expectations: strings(&[
                    "lab procedure animation",
                    "measurement overlay",
                    "error bar explorer",
                ]),
            },
        ],
    }
}

pub fn language_track() -> SubjectTrackDefinition {
    use EducationLevel::*;
    SubjectTrackDefinition {
        id: s("language"),
        domain: SubjectDomain::Language,
        title: s("Language"),
        levels: vec![
            Kindergarten,
            ElementaryLower,
            ElementaryUpper,
            MiddleSchool,
            HighSchool,
            UndergraduateLower,
            UndergraduateUpper,
            GraduateMasters,
            GraduateDoctoral,
        ],
        strands: vec![
            SubjectStrand {
                id: s("literacy"),
                title: s("Reading and literacy"),
                level_start: Kindergarten,
                level_end: UndergraduateLower,
                required_content: strings(&[
                    "phonological awareness",
                    "decoding",
                    "fluency",
                    "vocabulary",
                    "comprehension",
                    "literary analysis",
                    "disciplinary reading",
                ]),
                visual_expectations: strings(&[
                    "phoneme map",
                    "sentence parse",
                    "annotation layers",
                ]),
            },
            SubjectStrand {
                id: s("writing"),
                title: s("Writing and rhetoric"),
                level_start: ElementaryLower,
                level_end: GraduateDoctoral,
                required_content: strings(&[
                    "sentence construction",
                    "paragraphs",
                    "argument",
                    "genre",
                    "revision",
                    "academic writing",
                    "research writing",
                ]),
                visual_expectations: strings(&[
                    "outline graph",
                    "argument map",
                    "revision timeline",
                ]),
            },
            SubjectStrand {
                id: s("second-language"),
                title: s("Second language acquisition"),
                level_start: Kindergarten,
                level_end: GraduateDoctoral,
                required_content: strings(&[
                    "listening",
                    "speaking",
                    "pronunciation",
                    "grammar",
                    "conversation",
                    "culture",
                    "translation",
                ]),
                visual_expectations: strings(&[
                    "pronunciation waveform",
                    "dialogue role play",
                    "grammar tree",
                    "translation alignment",
                    "stroke order",
                ]),
            },
        ],
    }
}

pub fn programming_track() -> SubjectTrackDefinition {
    use EducationLevel::*;
    SubjectTrackDefinition {
        id: s("programming"),
        domain: SubjectDomain::Programming,
        title: s("Programming"),
        levels: vec![
            Kindergarten,
            ElementaryLower,
            ElementaryUpper,
            MiddleSchool,
            HighSchool,
            UndergraduateLower,
            UndergraduateUpper,
            GraduateMasters,
            GraduateDoctoral,
        ],
        strands: vec![
            SubjectStrand {
                id: s("computational-thinking"),
                title: s("Computational thinking"),
                level_start: Kindergarten,
                level_end: MiddleSchool,
                required_content: strings(&[
                    "sequencing",
                    "patterns",
                    "loops",
                    "conditions",
                    "decomposition",
                    "debugging",
                ]),
                visual_expectations: strings(&[
                    "block flow",
                    "step-through animation",
                    "state table",
                ]),
            },
            SubjectStrand {
                id: s("software-development"),
                title: s("Software development"),
                level_start: MiddleSchool,
                level_end: UndergraduateUpper,
                required_content: strings(&[
                    "syntax",
                    "data structures",
                    "algorithms",
                    "testing",
                    "version control",
                    "architecture",
                    "systems programming",
                ]),
                visual_expectations: strings(&[
                    "call stack",
                    "memory model",
                    "algorithm animation",
                ]),
            },
            SubjectStrand {
                id: s("advanced-computing"),
                title: s("Advanced computing"),
                level_start: UndergraduateLower,
                level_end: GraduateDoctoral,
                required_content: strings(&[
                    "operating systems",
                    "databases",
                    "networking",
                    "compilers",
                    "distributed systems",
                    "security",
                    "machine learning systems",
                ]),
                visual_expectations: strings(&[
                    "system topology",
                    "query plan",
                    "compiler pipeline",
                ]),
            },
        ],
    }
}

fn s(value: &str) -> String {
    value.to_string()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_catalog_contains_only_supported_domains() {
        let catalog = SubjectCatalog::production_scope();

        assert_eq!(catalog.tracks.len(), 4);
        assert!(catalog
            .tracks
            .iter()
            .all(|track| track.domain.product_supported()));
    }

    #[test]
    fn every_track_covers_kindergarten_to_graduate() {
        let catalog = SubjectCatalog::production_scope();

        for track in catalog.tracks {
            assert!(track.levels.contains(&EducationLevel::Kindergarten));
            assert!(track.levels.contains(&EducationLevel::GraduateDoctoral));
        }
    }
}
