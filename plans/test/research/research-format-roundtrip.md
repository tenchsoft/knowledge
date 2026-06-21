# Test: research-format-roundtrip

## 검증 대상
spec(`plans/spec/research/research-format-roundtrip.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: 논문 메타데이터가 손실 없이 복원 | `format_roundtrip_preserves_reference_metadata` |
| AC2: PDF 첨부파일이 그대로 접근 가능 | `format_roundtrip_preserves_attachments` |
| AC3: PDF 위의 하이라이트, 밑줄, 취소선, 스티키 노트가 원래 위치에 복원 | `format_roundtrip_preserves_annotations` |
| AC4: 컬렉션 트리 구조가 유지 | `format_roundtrip_preserves_collection_tree` |
| AC5: 태그와 태그-논문 연결이 유지 | `format_roundtrip_preserves_tags` |
| AC6: 즐겨찾기 상태가 유지 | `format_roundtrip_preserves_favorites` |
| AC7: BibTeX/RIS 가져오기 데이터가 정확하게 복원 | `format_roundtrip_preserves_bibtex_import` |
| AC8: Q&A 항목과 연결된 논문 페이지가 유지 | `format_roundtrip_preserves_qa_items` |
| AC9: 마지막으로 열었던 논문과 페이지 위치가 복원 | `format_roundtrip_preserves_last_opened` |

## 테스트 파일 위치
`crates/research-core/tests/format_roundtrip.rs`

## Required Test Shape
- **Success**: 라이브러리 저장 후 로드 시 모든 데이터 복원 -> 함수: `format_roundtrip_preserves_reference_metadata`
- **Success**: BibTeX 가져오기 후 라운드트립 -> 함수: `format_roundtrip_preserves_bibtex_import`
- **Failure**: 손상된 파일 로드 시 에러 처리 -> 함수: `format_roundtrip_handles_corrupted_file`
- **Edge case**: 대규모 라이브러리(100+ 논문) 라운드트립 -> 함수: `format_roundtrip_large_library`

## 사용할 자동화 노드
implement(`plans/implement/research/research-format-roundtrip.md`)에 명시된 바와 같이 자동화 노드 없음. 백엔드 크레이트 단위 테스트.

## 의존
- 선행 implement: `plans/implement/research/research-format-roundtrip.md`
- 픽스처: 불필요 (테스트에서 직접 스냅샷 생성)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p research-core format_roundtrip
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. `save_library_snapshot` / `load_library_snapshot` API를 grep해 위치 확인.
3. 각 시나리오 함수 작성 -- 패턴 D(라운드트립) 사용. 저장 -> 로드 -> 필드 비교.
4. `cargo test -p research-core format_roundtrip` 통과.
5. `cargo check --workspace --locked` 통과.
