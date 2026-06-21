# Tench Study — UI 사용자 관점 검토

- **작성일**: 2026-05-06
- **검토 대상**: `apps/study/src-tauri/src/ui/` (약 5.9k 줄)
- **검토 관점**: 학생이 처음 앱을 열고 한 개념을 *배우고 → 풀고 → 복습*하는 5~15분 흐름

## 첫인상

3-pane 레이아웃(Outline / Surface / Tutor) + 헤더 breadcrumb·streak·타이머·stage pill + 3단 학습 흐름(`Learn` → `Practice` → `Review`). 첫 실행 시 *Profile setup wizard* 모달이 자동으로 뜨고, **실제 builtin curriculum/lesson/glossary/visual spec 데이터가 즉시 로드**된다(`builtin_curricula()`). 다른 제품들의 "샘플 데이터"가 아닌 **진짜 학습 콘텐츠**가 들어있어 *바로 학습 시작 가능*. Tench 제품군 중에서 가장 *완성도 높은 첫 사용 경험* 후보.

## 강점

- **실제 빌트인 커리큘럼**: 16 lesson × 6 subject (Linear Algebra/Calculus/Physics/English/Korean/CS) — 단위/개념/문제/glossary/visual spec까지 모두 데이터로 존재
- **실제 자동 채점**(`tench_study_core::grade_answer`)
- **실제 SM-2 spaced repetition**(`apply_spaced_repetition_rating`)와 due review 카운터
- **3단계 hint 시스템** — 잠금/해제 시각화
- **다국어 i18n** 카탈로그 + locale 선택
- **세션 결과 모달, 통계 모달, 일일 대시보드(due/new/accuracy)** 등 학습 메타 정보 풍부
- **반응형 레이아웃**: 좁은 창에서 tutor 패널 숨김, 작은 너비에서 breadcrumb 축소
- **고대비 모드 토글**, **포커스 인디케이터**, **키보드 단축키 도움말** — 접근성 의식
- **저자(Authoring) 패널** — 콘텐츠 작성 모드 별도

---

## A. 핵심 결함 (P0~P1)

### A-1. AI 튜터가 echo placeholder (P1)

`state.rs:846~853`:
```rust
text: format!(
    "Let me think about '{}' in the context of {}...",
    self.tutor_chat_input,
    self.active_concept().label
),
```
사용자가 무엇을 묻든 *"Let me think about '< 입력 >' in the context of < 개념 >..."* 만 응답. 코드 코멘트도 *"Placeholder response - in production this connects to Engine"* 라며 인정. 학습 도우미라는 포지셔닝의 핵심 가치가 비어 있음.

### A-2. 답안 입력이 단일 라인 텍스트 (P1)

`practice.rs:70~89`. answer field가 한 줄. *"essay, code, diagram"*은 Planned에 있으니 알지만, **수학 답안에서도 분수·여러 줄 풀이는 못 적음**. Math palette가 `^`, `sqrt`, `frac`, `pi`, `alpha`, `beta`, `inf`, `sum` 단어를 *문자열로 삽입*하는 방식 → 사용자는 `frac`를 그대로 텍스트로 입력해 채점기에 보내게 됨. `tench_study_core::grade_answer`가 이런 토큰을 이해하는지 별도 확인 필요(스펙상 가능성 있지만 보장 안 됨).

### A-3. 정답 입력 placeholder가 텍스트 입력 cursor 시각화 부재 (P1)

`practice.rs:73~89`에 텍스트가 있을 때/없을 때 색상만 다르게 그릴 뿐 **커서 깜빡임이나 위치 표시가 없음**. 사용자는 자신이 어디 입력하고 있는지 시각적으로 확인 어려움. 비교: docs/sheets는 cursor blink 처리.

### A-4. Learn surface의 Y 좌표가 모두 절대 상수 (P2)

`learn.rs:21~`:
```
y + 38.0    // summary
y + 70.0    // definition begin
y + 154.0   // definition end
y + 176.0   // example
y + 260.0
y + 292.0
y + 354.0
y + 406.0
y + 520.0
y + 528.0
```
정의·예제·quick check·visual·play 컨트롤·시간선이 *모두 고정 위치*. 정의 텍스트가 길면 다음 섹션을 침범, 짧으면 빈 공간이 큼. *적응형 레이아웃 부재*.

### A-5. "Quick check" 섹션이 *질문은 표시하지만 답 입력 UI 없음* (P2)

`learn.rs:101~121`. `study.learn.quick_question` i18n 키만 출력. 답 필드, Submit 버튼, 정답 확인 흐름이 없음. *학습 직후 즉석 점검* 의도였을 텐데 미완성.

### A-6. Math palette는 텍스트 토큰 삽입 (P2)

`practice.rs:104~127`:
```rust
let math_symbols = ["^", "sqrt", "frac", "pi", "alpha", "beta", "inf", "sum"];
```
8개 버튼이 "그 단어를 입력 박스에 삽입"하는 동작. 표시될 때도 *그 단어 그대로* 노출 → 사용자가 한눈에 √나 π를 식별 못 함. 또한 `frac`만 있고 `frac{a}{b}` 같은 구조 입력 보조는 없음.

---

## B. 주요 UX 이슈

### B-1. 헤더의 "hc" 토글이 의미 불명 (P2)

`curriculum.rs:115`:
```rust
if state.high_contrast_mode { "HC" } else { "hc" }
```
대소문자만 토글. 보통 사용자는 *고대비 모드*라는 옵션을 알지 못하면 무엇인지 모름. 호버 툴팁 없음. 모양도 평범한 회색 박스라 클릭 어포던스 약함.

### B-2. Stage pill 클릭으로 stage 사이클 (P2)

`mod.rs:178`:
```rust
Some(StudyHit::StageClick) => self.state.cycle_stage(false),
```
사용자가 *"Learn"* 라벨 클릭 → Practice로 강제 이동. 아직 Learn 모드에서 학습이 끝나지 않았더라도 즉시 전환됨. 라벨이 클릭 가능하다는 시각 신호도 약함.

### B-3. Tutor 패널의 glossary 텍스트 클리핑이 36자 (P2)

`tutor.rs:204` `clipped_text(&term.definition, 36)` — 한국어 학습 콘텐츠에서 *글자 수* 36이지만 **반각·전각 차이** 무시. 또한 화면 폭이 260px(`PANEL_W`)고 폰트 12px이므로 36자가 한 줄에 다 들어가지 않을 가능성.

### B-4. Visual play/pause 라벨 `||` / `>` (P2)

`learn.rs:130` 등. 버튼 라벨이 `||` (paused 상태) / `>` (playing 상태). 보통 ▶/⏸ 유니코드를 쓰는데 ASCII로 작성. 시각 빈약함.

### B-5. 좁은 창에서 streak/타이머 사라짐 (P3)

`curriculum.rs:145~170`. `width >= 560.0` 조건. 모바일 뷰포트 활성 시 `touch_review.enabled` 상태와 별도 — *사용자가 "왜 streak이 사라졌지?" 혼란할 수 있음*. dashboard 미니 위젯도 `>= 700.0`에서만 보임.

### B-6. Profile setup wizard가 모든 진입에서 강제 표시? (P3)

`state.rs:99` `show_profile_setup_modal: true` 디폴트. 한번 완료한 뒤에도 새 세션마다 다시 뜨는지 확인 필요(시간상 미검증). 만약 매번 뜨면 큰 마찰.

### B-7. 핵심 시각 자료(Visual Specs)의 표시 위치 (P3)

`learn.rs:123` `paint_active_visual_surface` 호출은 있는데, 좁은 창에서 잘리거나 잘 안 보일 수 있음. 또한 자동 재생 토글이 별도(`Auto: ON/OFF` 라벨 텍스트, `learn.rs:160`) — 아이콘이면 좋음.

---

## C. 작은 디테일

- **Practice submit button**이 `surface.x0 + 32.0 ~ +118.0` (86px) — 한국어 라벨 *"제출"*은 짧지만 영어 *"Submit Answer"* 길어지면 잘림
- **Feedback 박스**(`practice.rs:182~242`)가 정답 시 초록·오답 시 빨강 + cause_tag 표시 — UX 친절
- **Review surface**가 wrong answer 빨강 / correct answer 초록 / cause·related concept를 한 화면에 — 학습 동기 강함
- **HH:MM:SS 타이머** 표시 — 정확
- **Streak 표시**가 emoji 없는 텍스트(`STATUS_WARNING` 색만) — 시각 흥미 약함
- **Concept bookmark / Notes panel toggle** 존재 — 좋음

---

## D. 다른 제품 대비 위치

| 항목 | docs / sheets / slides | study |
|---|---|---|
| 실제 데이터 | 빈 placeholder | **빌트인 16 lesson + 6 subject** |
| 핵심 알고리즘 | 일부 (sheets 수식만 진짜) | **SM-2 + 채점 모두 실제** |
| AI 통합 | 모두 placeholder | placeholder (echo) |
| 첫 사용 경험 | 빈 문서 | **wizard → curriculum → 학습** |
| 다국어 | 일부 | i18n catalog |
| 접근성 | 없음 | **고대비 + 포커스 indicator** |

study는 Tench 제품군 중 *바로 사용 가능한 가장 완성된 제품*에 가깝다.

---

## 우선순위 권장

| Priority | 항목 | 사용자 임팩트 |
|---|---|---|
| **P1** | AI 튜터 실제 Engine 연결 (A-1) | "AI tutor" 카피와 일치 |
| **P1** | 답안 입력에 cursor blink·multi-line 옵션 (A-3, A-2) | 입력 정확성 |
| **P1** | Quick check 섹션에 답 입력+제출 UI (A-5) | 학습 직후 점검 |
| **P1** | Math palette를 시각 기호(√, π, ÷, ∫)로 + structured insert (A-6) | 수학 친화적 입력 |
| **P2** | Learn surface 적응형 레이아웃 (A-4) | 콘텐츠 길이 변동 |
| **P2** | "hc" 토글에 명시적 라벨/툴팁 (B-1) | 옵션 발견 가능성 |
| **P2** | Stage pill 클릭 시 확인 또는 좀 더 명시적 트리거 (B-2) | 실수 방지 |
| **P2** | Glossary 텍스트 width 기반 클리핑 (B-3) | 한국어 학습 콘텐츠 |
| **P2** | Play/Pause 버튼을 ▶/⏸ 유니코드로 (B-4) | 시각 통일 |
| **P3** | Profile wizard 1회 완료 후 미표시 보장 (B-6) | 재진입 마찰 |
| **P3** | 좁은 창에서 streak/타이머 보존 또는 축약 표시 (B-5) | 모바일 UX |
| **P3** | Submit 버튼 폭 동적 (C) | 다국어 |
| **P3** | Streak에 시각적 강조(이모지·아이콘) | 학습 동기 |

---

## 한 줄 요약

진짜 커리큘럼·진짜 SM-2 채점·튜터/Glossary/세션 통계까지 **데이터와 알고리즘 측면에서 가장 완성된 제품**. 첫 사용 경험도 wizard로 부드럽게 시작된다. 남은 큰 구멍은 *AI 튜터 placeholder*와 *답안 입력 UI(특히 수학)의 빈약함* 두 가지. P1 네 개를 메우면 바로 학습 도구로 쓸 수준.
