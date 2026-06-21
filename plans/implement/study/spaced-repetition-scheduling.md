# Implement: spaced-repetition-scheduling

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: SM-2 알고리즘으로 평가(Easy/Good/Hard/Again)에 따라 간격, 반복 횟수, 난이도 계수를 갱신한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| state.rs | apply_spaced_repetition_rating 메서드 | grep 'fn apply_spaced_repetition_rating' apps/study/ |
| state/types.rs | SpacedRepetitionEntry / SpacedRepetitionRating | grep 'SpacedRepetitionEntry\|SpacedRepetitionRating' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. SM-2 품질 매핑
- **입력**: SpacedRepetitionRating 입력
- **처리**: Again→0, Hard→3, Good→4, Easy→5로 품질 변환
- **출력/사이드 이펙트**: quality 값으로 이후 계산 수행

### 2. 간격 계산
- **입력**: quality < 3인 경우
- **처리**: repetitions=0, interval_days=1 리셋
- **출력/사이드 이펙트**: 다음 리뷰가 1일 후로 설정

### 3. 간격 계산 (통과)
- **입력**: quality >= 3인 경우
- **처리**: rep 0→1일, rep 1→6일, rep 2+→interval*easiness_factor
- **출력/사이드 이펙트**: 반복 횟수 증가, 간격 확장

### 4. 난이도 계수 갱신
- **입력**: quality 값
- **처리**: EF = EF + (0.1 - (5-q)*(0.08 + (5-q)*0.02)), clamp(1.3, 10.0)
- **출력/사이드 이펙트**: concept별 맞춤 간격 조절


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
(KB 노드 — 단축키 전용, 별도 자동화 노드 없음)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
