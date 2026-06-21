# Design: study-format-roundtrip

## 한 줄 정의
Study 프로필 저장/불러오기 시 덱, 카드, 학습 진행률, 복습 일정, 성취 기록이 손실 없이 복원된다. 신규 시각 요소 없음 — 기존 UI 컴포넌트가 데이터 복원 후 자동 렌더.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Profile wizard | `Dialog` | `study.profile` |
| Curriculum outline | 기존 | `study.unit.*`, `study.concept.*.*` |
| Stats modal | 기존 | `study.modal.stats` |

모두 기존 디자인 사용. 저장/불러오기는 File 메뉴 기존 컴포넌트 재사용. 복원된 데이터가 기존 UI에 자동 반영되므로 별도 visual properties 불필요.

## Out of scope
- 클라우드 동기화 (별도 spec).
- 덱 공유/내보내기 (별도 spec).
