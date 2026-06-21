use tench_study_lib::ui::StudyApp;
use tench_ui_automation_core::{
    find_node, UiAutomationAction, UiAutomationCapture, UiAutomationCaptureRequest,
    UiAutomationKey, UiAutomationModifiers, UiAutomationNode, UiAutomationRect,
    UiAutomationSelector,
};
use tench_ui_test::{harness::HarnessConfig, snapshot::is_nonblank, TestHarness};

fn harness() -> TestHarness {
    TestHarness::with_config(StudyApp::new(), HarnessConfig::with_viewport(1280.0, 720.0))
}

fn selector(debug_id: &str) -> UiAutomationSelector {
    UiAutomationSelector::ByDebugId {
        debug_id: debug_id.to_string(),
    }
}

fn decode_png(capture: &UiAutomationCapture) -> image::RgbaImage {
    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let image = image::load_from_memory(&capture.png_bytes)
        .expect("valid automation png")
        .to_rgba8();
    assert_eq!(image.width(), capture.width);
    assert_eq!(image.height(), capture.height);
    assert!(
        is_nonblank(&image, 64),
        "capture should be visually non-blank"
    );
    image
}

fn tree(capture: &UiAutomationCapture) -> &UiAutomationNode {
    capture.ui_tree.as_ref().expect("automation tree")
}

fn assert_selector(capture: &UiAutomationCapture, debug_id: &str) {
    assert!(
        find_node(tree(capture), &selector(debug_id)).is_some(),
        "missing selector {debug_id}"
    );
}

fn assert_absent(capture: &UiAutomationCapture, debug_id: &str) {
    assert!(
        find_node(tree(capture), &selector(debug_id)).is_none(),
        "unexpected selector {debug_id}"
    );
}

fn assert_bounds_inside(capture: &UiAutomationCapture, debug_id: &str) {
    let node = find_node(tree(capture), &selector(debug_id)).expect("selector node");
    assert_rect_inside(
        &node.bounds,
        capture.width as f64,
        capture.height as f64,
        debug_id,
    );
}

fn assert_rect_inside(rect: &UiAutomationRect, width: f64, height: f64, label: &str) {
    assert!(
        rect.width > 0.0,
        "{label} width should be positive: {rect:?}"
    );
    assert!(
        rect.height > 0.0,
        "{label} height should be positive: {rect:?}"
    );
    assert!(rect.x >= 0.0, "{label} x should be on-screen: {rect:?}");
    assert!(rect.y >= 0.0, "{label} y should be on-screen: {rect:?}");
    assert!(
        rect.x + rect.width <= width,
        "{label} should fit viewport width: {rect:?} / {width}"
    );
    assert!(
        rect.y + rect.height <= height,
        "{label} should fit viewport height: {rect:?} / {height}"
    );
}

fn click(harness: &mut TestHarness, debug_id: &str) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::Click {
            selector: selector(debug_id),
            modifiers: Default::default(),
        })
        .unwrap_or_else(|err| panic!("click {debug_id}: {err:?}"))
}

fn key(
    harness: &mut TestHarness,
    key: UiAutomationKey,
    modifiers: UiAutomationModifiers,
) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::KeyPress { key, modifiers })
        .expect("key press")
}

fn type_text(harness: &mut TestHarness, text: &str) -> UiAutomationCapture {
    let mut capture = harness.automation_capture(UiAutomationCaptureRequest::default());
    for ch in text.chars() {
        capture = key(
            harness,
            UiAutomationKey::Character(ch.to_string()),
            UiAutomationModifiers::default(),
        );
    }
    capture
}

fn complete_profile_setup(harness: &mut TestHarness) -> UiAutomationCapture {
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    assert_selector(&initial, "study.profile.learner_id");
    assert_selector(&initial, "study.profile.display_name");
    assert_bounds_inside(&initial, "study.profile.next");

    let learner_focus = click(harness, "study.profile.learner_id");
    decode_png(&learner_focus);
    type_text(harness, "learner01");

    let name_focus = click(harness, "study.profile.display_name");
    decode_png(&name_focus);
    type_text(harness, "Tench Student");

    let domain_step = click(harness, "study.profile.next");
    decode_png(&domain_step);
    assert_selector(&domain_step, "study.profile.domain.0");
    let domain_selected = click(harness, "study.profile.domain.0");
    decode_png(&domain_selected);
    let level_selected = click(harness, "study.profile.level.high-school");
    decode_png(&level_selected);

    let locale_step = click(harness, "study.profile.next");
    decode_png(&locale_step);
    assert_selector(&locale_step, "study.profile.locale.ko-KR");
    let locale_selected = click(harness, "study.profile.locale.ko-KR");
    decode_png(&locale_selected);

    let main = click(harness, "study.profile.next");
    decode_png(&main);
    assert_absent(&main, "study.profile");
    main
}

#[test]
fn study_plan_profile_and_learn_controls_use_real_events_ui_e2e() {
    let mut harness = harness();
    let main = complete_profile_setup(&mut harness);
    let main_image = decode_png(&main);

    for debug_id in [
        "study.header.stats",
        "study.header.stage",
        "study.header.shortcuts",
        "study.header.high_contrast",
        "study.header.goals",
        "study.curriculum.search",
        "study.curriculum.bookmark",
        "study.curriculum.notes",
        "study.review.queue",
        "study.unit.0",
        "study.concept.0.0",
        "study.learn.start_practice",
        "study.visual.play_pause",
        "study.visual.autoplay",
        "study.tutor.hint.1",
        "study.tutor.hint.2",
        "study.tutor.hint.3",
        "study.tutor.chat.input",
        "study.tutor.chat.send",
        "study.glossary.search",
        "study.glossary.term.0",
    ] {
        assert_selector(&main, debug_id);
        assert_bounds_inside(&main, debug_id);
    }

    let visual_play = click(&mut harness, "study.visual.play_pause");
    let visual_image = decode_png(&visual_play);
    assert!(
        main_image.as_raw() != visual_image.as_raw(),
        "visual play button should visibly update the learning visual state"
    );

    let visual_autoplay = click(&mut harness, "study.visual.autoplay");
    let autoplay_image = decode_png(&visual_autoplay);
    assert!(
        visual_image.as_raw() != autoplay_image.as_raw(),
        "visual autoplay toggle should visibly update the visual control state"
    );

    let high_contrast = click(&mut harness, "study.header.high_contrast");
    let contrast_image = decode_png(&high_contrast);
    assert!(
        autoplay_image.as_raw() != contrast_image.as_raw(),
        "high contrast toggle should visibly change the UI palette"
    );

    let search_focus = click(&mut harness, "study.curriculum.search");
    decode_png(&search_focus);
    let search_text = type_text(&mut harness, "linear");
    let search_image = decode_png(&search_text);
    assert!(
        contrast_image.as_raw() != search_image.as_raw(),
        "curriculum search input should visibly render typed text/filter state"
    );

    let notes_panel = click(&mut harness, "study.curriculum.notes");
    let notes_panel_image = decode_png(&notes_panel);
    assert!(
        search_image.as_raw() != notes_panel_image.as_raw(),
        "notes toggle should visibly open the notes overlay"
    );
    assert_selector(&notes_panel, "study.notes.input");
    assert_selector(&notes_panel, "study.notes.save");
    let notes_focus = click(&mut harness, "study.notes.input");
    decode_png(&notes_focus);
    let note_text = type_text(&mut harness, "remember this concept");
    let note_text_image = decode_png(&note_text);
    assert!(
        notes_panel_image.as_raw() != note_text_image.as_raw(),
        "notes input should visibly render typed text"
    );
    let note_saved = click(&mut harness, "study.notes.save");
    let note_saved_image = decode_png(&note_saved);
    assert!(
        note_text_image.as_raw() != note_saved_image.as_raw(),
        "notes save button should visibly add an existing note row"
    );
    assert_selector(&note_saved, "study.notes.row.0");

    let chat_focus = click(&mut harness, "study.tutor.chat.input");
    decode_png(&chat_focus);
    let chat_text = type_text(&mut harness, "explain again");
    let chat_text_image = decode_png(&chat_text);
    assert!(
        note_saved_image.as_raw() != chat_text_image.as_raw(),
        "tutor chat input should visibly render typed text"
    );
    let chat_sent = click(&mut harness, "study.tutor.chat.send");
    let chat_sent_image = decode_png(&chat_sent);
    assert!(
        chat_text_image.as_raw() != chat_sent_image.as_raw(),
        "tutor send button should visibly add chat messages"
    );

    let shortcuts = click(&mut harness, "study.header.shortcuts");
    decode_png(&shortcuts);
    assert_selector(&shortcuts, "study.modal.shortcuts");
    let shortcuts_closed = click(&mut harness, "study.modal.close");
    decode_png(&shortcuts_closed);
    assert_absent(&shortcuts_closed, "study.modal.shortcuts");

    let goals = click(&mut harness, "study.header.goals");
    decode_png(&goals);
    assert_selector(&goals, "study.modal.goal");
    let goals_closed = click(&mut harness, "study.modal.close");
    decode_png(&goals_closed);
    assert_absent(&goals_closed, "study.modal.goal");
}

#[test]
fn study_plan_practice_review_and_shortcut_flows_use_real_events_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    let practice = click(&mut harness, "study.learn.start_practice");
    let practice_image = decode_png(&practice);
    for debug_id in [
        "study.practice.submit",
        "study.practice.skip",
        "study.practice.pause",
        "study.practice.math_palette",
    ] {
        assert_selector(&practice, debug_id);
        assert_bounds_inside(&practice, debug_id);
    }

    let answer = type_text(&mut harness, "wrong");
    let answer_image = decode_png(&answer);
    assert!(
        practice_image.as_raw() != answer_image.as_raw(),
        "practice character input should visibly update the answer field"
    );

    let palette = click(&mut harness, "study.practice.math_palette");
    let palette_image = decode_png(&palette);
    assert!(
        answer_image.as_raw() != palette_image.as_raw(),
        "math palette button should visibly show the symbol palette"
    );
    for debug_id in [
        "study.practice.math_symbol.power",
        "study.practice.math_symbol.sqrt",
        "study.practice.math_symbol.fraction",
        "study.practice.math_symbol.pi",
        "study.practice.math_symbol.alpha",
        "study.practice.math_symbol.beta",
        "study.practice.math_symbol.infinity",
        "study.practice.math_symbol.sum",
    ] {
        assert_selector(&palette, debug_id);
        assert_bounds_inside(&palette, debug_id);
    }

    let symbol_inserted = click(&mut harness, "study.practice.math_symbol.sqrt");
    let symbol_image = decode_png(&symbol_inserted);
    assert!(
        palette_image.as_raw() != symbol_image.as_raw(),
        "math symbol button should visibly insert into the practice answer"
    );

    let submitted = click(&mut harness, "study.practice.submit");
    let submitted_image = decode_png(&submitted);
    assert!(
        palette_image.as_raw() != submitted_image.as_raw(),
        "submit answer button should visibly show practice feedback"
    );
    for debug_id in [
        "study.practice.retry",
        "study.practice.review_concept",
        "study.practice.next",
    ] {
        assert_selector(&submitted, debug_id);
        assert_bounds_inside(&submitted, debug_id);
    }

    let skipped = click(&mut harness, "study.practice.skip");
    let skipped_image = decode_png(&skipped);
    assert!(
        submitted_image.as_raw() != skipped_image.as_raw(),
        "skip problem button should visibly queue the item for review"
    );

    let result_closed = key(
        &mut harness,
        UiAutomationKey::Escape,
        UiAutomationModifiers::default(),
    );
    let result_closed_image = decode_png(&result_closed);

    let stats = key(
        &mut harness,
        UiAutomationKey::Character("s".to_string()),
        UiAutomationModifiers {
            control: true,
            ..UiAutomationModifiers::default()
        },
    );
    let stats_image = decode_png(&stats);
    assert!(
        result_closed_image.as_raw() != stats_image.as_raw(),
        "Ctrl+S should visibly open the stats modal"
    );

    let closed = key(
        &mut harness,
        UiAutomationKey::Escape,
        UiAutomationModifiers::default(),
    );
    let closed_image = decode_png(&closed);

    let stage_review = click(&mut harness, "study.header.stage");
    let review_image = decode_png(&stage_review);
    assert!(
        closed_image.as_raw() != review_image.as_raw(),
        "stage pill click should visibly cycle the study stage"
    );
    for debug_id in [
        "study.review.rating.again",
        "study.review.rating.hard",
        "study.review.rating.good",
        "study.review.rating.easy",
    ] {
        assert_selector(&stage_review, debug_id);
        assert_bounds_inside(&stage_review, debug_id);
    }
}
