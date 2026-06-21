#[tauri::command]
pub fn add_study_curriculum_node(
    mut graph: tench_study_core::CurriculumGraph,
    node: tench_study_core::CurriculumNode,
) -> Result<tench_study_core::CurriculumGraph, String> {
    graph.add_node(node)?;
    Ok(graph)
}

#[tauri::command]
pub fn add_study_curriculum_edge(
    mut graph: tench_study_core::CurriculumGraph,
    edge: tench_study_core::CurriculumEdge,
) -> Result<tench_study_core::CurriculumGraph, String> {
    graph.add_edge(edge)?;
    Ok(graph)
}

#[tauri::command]
pub fn study_curriculum_orphan_nodes(
    graph: tench_study_core::CurriculumGraph,
) -> Vec<tench_study_core::CurriculumNodeId> {
    graph.orphan_nodes()
}
